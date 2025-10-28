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
import java.util.Map;
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
     * captures output streams, and deserializes the collected trace data.
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

        // Check for DiSL analysis marker file and handle test generation
        handleDiSLAnalysisResults(instrumentation);

        // Read generated test paths from results file created by Collector
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

    private void handleDiSLAnalysisResults(InstrumentationResult instrumentation) {
        try {
            // Check for DiSL analysis marker file
            Path outputDir = runConfiguration.getOutputDirectory();
            Path markerFile = outputDir.resolve("disl-analysis-complete.marker");

            log.info("Checking for DiSL analysis marker file at: {}", markerFile);
            log.info("Output directory contents:");
            try {
                Files.list(outputDir).forEach(path -> log.info("  - {}", path.getFileName()));
            } catch (Exception e) {
                log.warn("Could not list output directory contents: {}", e.getMessage());
            }

            if (Files.exists(markerFile)) {
                log.info("Found DiSL analysis marker file, processing collected values");

                // Read marker file to get metadata
                String markerContent = Files.readString(markerFile);
                log.info("Marker file content: {}", markerContent);

                // Extract paths and metadata from marker file
                Path collectedValuesFile = outputDir.resolve("collected-values.ser");

                // Get identifier mapping path from instrumentation
                Path identifierMappingFile = instrumentation.getIdentifiersMappingPath();

                if (Files.exists(collectedValuesFile) && identifierMappingFile != null && Files.exists(identifierMappingFile)) {
                    log.info("Processing collected values and generating tests");
                    processCollectedValuesAndGenerateTests(collectedValuesFile, identifierMappingFile, instrumentation, markerContent);
                } else {
                    log.warn("Missing required files - collectedValues: {}, identifierMapping: {}",
                            Files.exists(collectedValuesFile), identifierMappingFile != null && Files.exists(identifierMappingFile));
                }
            } else {
                log.warn("No DiSL analysis marker file found at: {}", markerFile);
                log.warn("This means the DiSL Collector did not execute properly or did not write the marker file");
                log.warn("Check if the DiSL RE server is working and the Collector class is being loaded");
            }
        } catch (Exception e) {
            log.error("Error handling DiSL analysis results", e);
        }
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

    @SuppressWarnings("unchecked")
    private void processCollectedValuesAndGenerateTests(Path collectedValuesFile, Path identifierMappingFile,
                                                       InstrumentationResult instrumentation, String markerContent) {
        try {
            log.info("Loading collected invocations from: {}", collectedValuesFile);

            // Load collected invocations
            List<Map<Integer, Object>> allInvocations;
            try (java.io.FileInputStream fileIn = new java.io.FileInputStream(collectedValuesFile.toFile());
                 java.io.ObjectInputStream objectIn = new java.io.ObjectInputStream(fileIn)) {
                allInvocations = (List<Map<Integer, Object>>) objectIn.readObject();
            }
            log.info("Loaded {} invocations", allInvocations.size());

            // Debug: Print all invocations
            for (int i = 0; i < allInvocations.size(); i++) {
                Map<Integer, Object> invocation = allInvocations.get(i);
                log.info("Invocation {}: {} parameters", i + 1, invocation.size());
                for (Map.Entry<Integer, Object> entry : invocation.entrySet()) {
                    log.info("  Slot {}: {} (type: {})",
                            entry.getKey(), entry.getValue(), entry.getValue().getClass().getSimpleName());
                }
            }

            // Load identifier mapping
            log.info("Loading identifier mapping from: {}", identifierMappingFile);
            Map<Integer, cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier> identifierMapping;
            try (java.io.FileInputStream fileIn = new java.io.FileInputStream(identifierMappingFile.toFile());
                 java.io.ObjectInputStream objectIn = new java.io.ObjectInputStream(fileIn)) {
                identifierMapping = (Map<Integer, cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier>) objectIn.readObject();
            }
            log.info("Loaded identifier mapping with {} entries", identifierMapping.size());

            // Debug: Print all identifier mappings
            for (Map.Entry<Integer, cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier> entry : identifierMapping.entrySet()) {
                log.info("Identifier mapping - Slot {}: {}", entry.getKey(), entry.getValue());
            }

            // Generate tests using proper test generator infrastructure
            log.info("Generating tests using NaiveTraceBasedGenerator for {} invocations", allInvocations.size());

            // Create Trace from all invocations (for compatibility)
            var trace = new cz.cuni.mff.d3s.autodebugger.model.common.trace.Trace();
            int addedValues = 0;
            for (Map<Integer, Object> invocation : allInvocations) {
                for (Map.Entry<Integer, Object> entry : invocation.entrySet()) {
                Object value = entry.getValue();
                if (value instanceof Integer) {
                    trace.addIntValue(entry.getKey(), (Integer) value);
                    addedValues++;
                    log.info("Added int value to trace - Slot {}: {}", entry.getKey(), value);
                } else if (value instanceof Byte) {
                    trace.addByteValue(entry.getKey(), (Byte) value);
                    addedValues++;
                    log.info("Added byte value to trace - Slot {}: {}", entry.getKey(), value);
                } else if (value instanceof Character) {
                    trace.addCharValue(entry.getKey(), (Character) value);
                    addedValues++;
                    log.info("Added char value to trace - Slot {}: {}", entry.getKey(), value);
                } else if (value instanceof Short) {
                    trace.addShortValue(entry.getKey(), (Short) value);
                    addedValues++;
                    log.info("Added short value to trace - Slot {}: {}", entry.getKey(), value);
                } else if (value instanceof Long) {
                    trace.addLongValue(entry.getKey(), (Long) value);
                    addedValues++;
                    log.info("Added long value to trace - Slot {}: {}", entry.getKey(), value);
                } else if (value instanceof Float) {
                    trace.addFloatValue(entry.getKey(), (Float) value);
                    addedValues++;
                    log.info("Added float value to trace - Slot {}: {}", entry.getKey(), value);
                } else if (value instanceof Double) {
                    trace.addDoubleValue(entry.getKey(), (Double) value);
                    addedValues++;
                    log.info("Added double value to trace - Slot {}: {}", entry.getKey(), value);
                } else if (value instanceof Boolean) {
                    trace.addBooleanValue(entry.getKey(), (Boolean) value);
                    addedValues++;
                    log.info("Added boolean value to trace - Slot {}: {}", entry.getKey(), value);
                } else {
                    log.warn("Unsupported value type for trace - Slot {}: {} (type: {})",
                            entry.getKey(), value, value.getClass().getSimpleName());
                }
            }
            }
            log.info("Created Trace with {} total values from {} invocations", addedValues, allInvocations.size());

            // Use proper test generator infrastructure
            try {
                // Create test generator with identifier mapping
                var testGenerator = new cz.cuni.mff.d3s.autodebugger.testgenerator.java.trace.NaiveTraceBasedGenerator(identifierMapping);

                // Generate tests using the JavaRunConfiguration
                List<Path> generatedTestFiles = testGenerator.generateTests(trace, runConfiguration);

                log.info("Generated {} test files using NaiveTraceBasedGenerator", generatedTestFiles.size());

                // Write generated test file paths to results list
                Path resultsFile = instrumentation.getResultsListPath();
                for (Path testFile : generatedTestFiles) {
                    Files.writeString(resultsFile, testFile.toAbsolutePath().toString() + System.lineSeparator(),
                                    java.nio.file.StandardOpenOption.CREATE, java.nio.file.StandardOpenOption.APPEND);
                    log.info("Added test file to results: {}", testFile);
                }

            } catch (Exception e) {
                log.error("Failed to generate tests using NaiveTraceBasedGenerator", e);
                // Fallback to simple test generation if needed
                log.warn("Test generation failed, but DiSL analysis completed successfully");
            }

        } catch (Exception e) {
            log.error("Error processing collected values and generating tests", e);
        }
    }
}
