package cz.cuni.mff.d3s.autodebugger.analyzer.java;

import cz.cuni.mff.d3s.autodebugger.analyzer.common.Analyzer;
import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
import cz.cuni.mff.d3s.autodebugger.model.common.tests.TestSuite;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.java.helper.DiSLPathHelper;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import cz.cuni.mff.d3s.autodebugger.model.common.trace.Trace;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerationContext;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerator;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.JavaTestGenerationContextFactory;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.trace.NaiveTraceBasedGenerator;
import java.io.FileInputStream;
import java.io.ObjectInputStream;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.TimeUnit;

/**
 * Java-specific analyzer implementation that executes instrumented Java applications
 * and collects runtime traces through DiSL instrumentation.
 */
@Slf4j
@RequiredArgsConstructor
public class DiSLAnalyzer implements Analyzer {

    private static final int DEFAULT_TIMEOUT_SECONDS = 300; // 5 minutes

    @Getter
    private final JavaRunConfiguration runConfiguration;

    /**
     * Gets the timeout in seconds for process execution.
     * Protected to allow overriding in tests.
     */
    protected long getTimeoutSeconds() {
        return DEFAULT_TIMEOUT_SECONDS;
    }

    /**
     * Executes the instrumented Java application and collects runtime traces.
     * Runs the application as a separate process with DiSL instrumentation,
     * captures output streams, and generates tests locally.
     *
     * The Collector (running in the DiSL process) is responsible for:
     * - Collecting runtime values
     * - Building the Trace
     * - Serializing the Trace to disk
     *
     * This method orchestrates the execution, deserializes the trace, and generates tests.
     */
    @Override
    public TestSuite runAnalysis(InstrumentationResult instrumentation) {
        log.info("Starting Java analysis on instrumented application: {}", instrumentation);

        validateInstrumentation(instrumentation);
        Path instrumentationJarPath = instrumentation.getPrimaryArtifact();
        try {
            // Build the command to execute the instrumented JAR
            List<String> command = buildExecutionCommand(instrumentationJarPath);

            // Execute the instrumented application
            int exitCode = runCommandAsProcess(command);

            if (exitCode != 0) {
                log.error("Analysis process failed with exit code: {}", exitCode);
                throw new RuntimeException("DiSL analysis failed with exit code: " + exitCode);
            }
        } catch (IOException | InterruptedException e) {
            log.error("Failed to execute instrumented application", e);
            throw new RuntimeException("Analysis execution failed", e);
        }

        // Post-processing: deserialize trace and generate tests locally
        Trace trace = deserializeTrace(instrumentation.getTraceFilePath());
        if (trace == null || isTraceEmpty(trace)) {
            log.warn("No trace data collected, returning empty test suite");
            return TestSuite.builder()
                    .baseDirectory(runConfiguration.getOutputDirectory())
                    .testFiles(List.of())
                    .build();
        }

        NaiveTraceBasedGenerator generator = createTestGenerator(instrumentation.getIdentifiersMappingPath());
        TestGenerationContext context = JavaTestGenerationContextFactory
                .createFromJavaRunConfiguration(runConfiguration);
        List<Path> generatedTests = generator.generateTests(trace, context);

        return TestSuite.builder()
                .baseDirectory(runConfiguration.getOutputDirectory())
                .testFiles(generatedTests)
                .build();
    }

    /**
     * Generates tests from an existing serialized trace file without running instrumentation.
     * This enables retry capability when test generation failed but the trace was successfully collected.
     *
     * @param traceFilePath Path to the serialized trace file
     * @param identifierMappingPath Path to the serialized identifier mapping
     * @return TestSuite containing generated test files
     * @throws IllegalArgumentException if either file is null or doesn't exist
     */
    public TestSuite generateTestsFromExistingTrace(Path traceFilePath, Path identifierMappingPath) {
        log.info("Generating tests from existing trace: {}", traceFilePath);

        if (traceFilePath == null || !Files.exists(traceFilePath)) {
            throw new IllegalArgumentException("Trace file not found: " + traceFilePath);
        }
        if (identifierMappingPath == null || !Files.exists(identifierMappingPath)) {
            throw new IllegalArgumentException("Identifier mapping file not found: " + identifierMappingPath);
        }

        Trace trace = deserializeTrace(traceFilePath);
        if (trace == null || isTraceEmpty(trace)) {
            log.warn("Trace is empty or could not be deserialized");
            return TestSuite.builder()
                    .baseDirectory(runConfiguration.getOutputDirectory())
                    .testFiles(List.of())
                    .build();
        }

        NaiveTraceBasedGenerator generator = createTestGenerator(identifierMappingPath);
        TestGenerationContext context = JavaTestGenerationContextFactory
                .createFromJavaRunConfiguration(runConfiguration);
        List<Path> generatedTests = generator.generateTests(trace, context);

        return TestSuite.builder()
                .baseDirectory(runConfiguration.getOutputDirectory())
                .testFiles(generatedTests)
                .build();
    }

    @Override
    public void validateInstrumentation(InstrumentationResult instrumentation) {
        if (instrumentation == null || instrumentation.getPrimaryArtifact() == null) {
            throw new IllegalArgumentException("Instrumentation primary artifact cannot be null");
        }
        var instrumentationPath = instrumentation.getPrimaryArtifact();
        if (!Files.exists(instrumentationPath)) {
            throw new IllegalArgumentException("Instrumentation file does not exist: " + instrumentationPath);
        }
        if (!instrumentationPath.toString().endsWith(".jar")) {
            throw new IllegalArgumentException("Expected JAR file for DiSL instrumentation, got: " + instrumentationPath);
        }
        log.debug("Instrumentation validation passed for: {}", instrumentationPath);
    }

    public List<String> buildExecutionCommand(Path instrumentationJarPath) {
        List<String> command = new ArrayList<>();

        // Run the disl.py script
        command.add("python3");
        command.add(DiSLPathHelper.getDislRunnerPath(runConfiguration).toString());

        // Add the DiSL home path
        command.add("-d");
        command.add(DiSLPathHelper.getDislHomePath(runConfiguration).toString());

        // Run with the client (target app), server (DiSL instrumentation server), and evaluation (ShadowVM)
        command.add("-cse");

        // Add the evaluation classpath
        // Note that this is not present in the basic distribution of DiSL - and might potentially be unnecessary
        // Subject to further testing...
        command.add("-e_cp");
        // Include the instrumentation JAR so DiSL RE server can find the Collector class
        // Convert relative paths to absolute paths since DiSL runs from output directory
        String evaluationClasspath = getEvaluationClasspath(instrumentationJarPath);
        command.add(evaluationClasspath);

        command.add("--");

        // Add the generated DiSL instrumentation JAR (use absolute path)
        command.add(instrumentationJarPath.toAbsolutePath().toString());

        // Add the target application JAR
        command.add("-jar");
        command.add(runConfiguration.getApplicationPath().toString());

        // Add runtime arguments for the target application
        if (!runConfiguration.getRuntimeArguments().isEmpty()) {
            command.addAll(runConfiguration.getRuntimeArguments());
        }

        return command;
    }

    private int runCommandAsProcess(List<String> command) throws IOException, InterruptedException {
        ProcessBuilder processBuilder = new ProcessBuilder(command);
        processBuilder.directory(runConfiguration.getOutputDirectory().toFile());

        log.info("Executing command: {}", String.join(" ", command));
        Process process = processBuilder.start();

        // Capture output and error streams
        StringBuilder output = new StringBuilder();
        StringBuilder errorOutput = new StringBuilder();

        // Read output in separate threads
        Thread outputReader = new Thread(() -> readStream(process.getInputStream(), output));
        Thread errorReader = new Thread(() -> readStream(process.getErrorStream(), errorOutput));

        outputReader.start();
        errorReader.start();

        // Wait for process completion with timeout
        boolean finished = process.waitFor(getTimeoutSeconds(), TimeUnit.SECONDS);

        if (!finished) {
            log.warn("Analysis process timed out after {} seconds, terminating", getTimeoutSeconds());
            process.destroyForcibly();
            throw new RuntimeException("Analysis process timed out");
        }

        // Wait for output readers to finish
        outputReader.join(5000);
        errorReader.join(5000);

        int exitCode = process.exitValue();
        log.info("Analysis process completed with exit code: {}", exitCode);

        // Always log both output and error streams for debugging
        log.info("Analysis stdout output: {}", output);
        log.info("Analysis stderr output: {}", errorOutput);

        return exitCode;
    }

    /**
     * Deserializes a Trace object from a file.
     * @param traceFilePath Path to the serialized trace file
     * @return The deserialized Trace object, or null if not available
     */
    private Trace deserializeTrace(Path traceFilePath) {
        if (traceFilePath == null) {
            log.warn("Trace file path is null");
            return null;
        }
        if (!Files.exists(traceFilePath)) {
            log.warn("Trace file does not exist: {}", traceFilePath);
            return null;
        }

        log.info("Deserializing trace from: {}", traceFilePath);
        try (FileInputStream fileInput = new FileInputStream(traceFilePath.toFile());
             ObjectInputStream objectInput = new ObjectInputStream(fileInput)) {
            Trace trace = (Trace) objectInput.readObject();
            log.info("Successfully deserialized trace");
            return trace;
        } catch (Exception e) {
            log.error("Failed to deserialize trace from {}", traceFilePath, e);
            return null;
        }
    }

    /**
     * Creates the appropriate TestGenerator based on configuration.
     * @param identifierMappingPath Path to the serialized identifier mapping
     * @return A NaiveTraceBasedGenerator instance
     */
    private NaiveTraceBasedGenerator createTestGenerator(Path identifierMappingPath) {
        if (identifierMappingPath == null || !Files.exists(identifierMappingPath)) {
            throw new IllegalArgumentException("Identifier mapping file not found: " + identifierMappingPath);
        }
        log.info("Creating test generator with identifier mapping: {}", identifierMappingPath);
        return new NaiveTraceBasedGenerator(identifierMappingPath);
    }

    /**
     * Checks if a Trace object is empty (has no values).
     * @param trace The trace to check
     * @return true if the trace has no values, false otherwise
     */
    private boolean isTraceEmpty(Trace trace) {
        // Check if trace has any values in any slot
        // We need to check a reasonable range of slots (0-100)
        for (int slot = 0; slot < 100; slot++) {
            if (!trace.getIntValues(slot).isEmpty() ||
                !trace.getLongValues(slot).isEmpty() ||
                !trace.getBooleanValues(slot).isEmpty() ||
                !trace.getFloatValues(slot).isEmpty() ||
                !trace.getDoubleValues(slot).isEmpty() ||
                !trace.getCharValues(slot).isEmpty() ||
                !trace.getByteValues(slot).isEmpty() ||
                !trace.getShortValues(slot).isEmpty()) {
                return false; // Found at least one value
            }
        }
        return true; // No values found in any checked slot
    }

    private static String getEvaluationClasspath(Path instrumentationJarPath) {
        Path instrumentationJarAbsolutePath = instrumentationJarPath.toAbsolutePath();
        // Only need model-common for Trace serialization in ShadowVM
        // Navigate up to find the project root (where model-common is located)
        Path parent = instrumentationJarAbsolutePath.getParent();
        if (parent != null) {
            Path grandparent = parent.getParent();
            if (grandparent != null) {
                Path greatGrandparent = grandparent.getParent();
                if (greatGrandparent != null) {
                    return instrumentationJarAbsolutePath + ":" +
                           greatGrandparent.resolve("model-common/build/libs").toAbsolutePath() + "/*";
                }
            }
        }
        // Fallback: just include the instrumentation JAR itself
        return instrumentationJarAbsolutePath.toString();
    }

    private void readStream(InputStream inputStream, StringBuilder output) {
        try (BufferedReader reader = new BufferedReader(new InputStreamReader(inputStream))) {
            String line;
            while ((line = reader.readLine()) != null) {
                output.append(line).append(System.lineSeparator());
            }
        } catch (IOException e) {
            log.error("Error reading process stream", e);
        }
    }
}
