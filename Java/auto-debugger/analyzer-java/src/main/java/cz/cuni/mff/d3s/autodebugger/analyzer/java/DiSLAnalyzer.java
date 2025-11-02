package cz.cuni.mff.d3s.autodebugger.analyzer.java;

import cz.cuni.mff.d3s.autodebugger.analyzer.common.Analyzer;
import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
import cz.cuni.mff.d3s.autodebugger.model.common.tests.TestSuite;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.java.helper.DiSLPathHelper;
import lombok.Getter;
import lombok.RequiredArgsConstructor;
import lombok.extern.slf4j.Slf4j;

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
     * captures output streams, and collects generated test files.
     *
     * The Collector (running in the DiSL process) is responsible for:
     * - Collecting runtime values
     * - Building the Trace
     * - Generating tests
     * - Writing test file paths to the results list
     *
     * This method only orchestrates the execution and collects the results.
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

        // Collector has already generated tests - just collect the results
        List<Path> generatedTests = collectGeneratedTestFiles(instrumentation);

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

    private List<Path> collectGeneratedTestFiles(InstrumentationResult instrumentation) {
        List<Path> generatedTests = new ArrayList<>();
        try {
            Path resultsFile = instrumentation.getResultsListPath();
            if (resultsFile == null) {
                log.warn("No results list path available; no tests will be returned");
                return generatedTests;
            }
            if (!Files.exists(resultsFile)) {
                log.warn("No generated test list file found at {}", resultsFile);
                return generatedTests;
            }
            log.info("Reading generated test list from {}", resultsFile);
            var lines = Files.readAllLines(resultsFile);
            lines.forEach(l -> {
                log.info("Generated test: {}", l);
                generatedTests.add(Path.of(l));
            });
        } catch (Exception ex) {
            log.warn("Failed to read generated test list", ex);
        }
        return generatedTests;
    }

    private static String getEvaluationClasspath(Path instrumentationJarPath) {
        Path instrumentationJarAbsolutePath = instrumentationJarPath.toAbsolutePath();
        return instrumentationJarAbsolutePath + ":" +
                                   instrumentationJarAbsolutePath.getParent().getParent().getParent().resolve("test-generator-java/build/libs").toAbsolutePath() + "/*:" +
                                   instrumentationJarAbsolutePath.getParent().getParent().getParent().resolve("test-generator-common/build/libs").toAbsolutePath() + "/*:" +
                                   instrumentationJarAbsolutePath.getParent().getParent().getParent().resolve("model-common/build/libs").toAbsolutePath() + "/*:" +
                                   instrumentationJarAbsolutePath.getParent().getParent().getParent().resolve("model-java/build/libs").toAbsolutePath() + "/*";
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
