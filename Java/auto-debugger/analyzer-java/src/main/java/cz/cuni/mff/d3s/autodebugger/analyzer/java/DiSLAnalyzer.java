package cz.cuni.mff.d3s.autodebugger.analyzer.java;

import cz.cuni.mff.d3s.autodebugger.analyzer.common.Analyzer;
import cz.cuni.mff.d3s.autodebugger.analyzer.common.AnalysisResult;
import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
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
import java.util.Optional;
import java.util.concurrent.TimeUnit;

/**
 * Java-specific analyzer implementation that executes instrumented Java applications
 * and collects runtime traces through DiSL instrumentation.
 */
@Slf4j
@RequiredArgsConstructor
public class DiSLAnalyzer implements Analyzer {

    private static final int DEFAULT_TIMEOUT_SECONDS = 300; // 5 minutes

    /**
     * Environment variable for explicitly setting the model-common classpath.
     * Useful for tests running from temp directories where project structure isn't accessible.
     */
    public static final String MODEL_COMMON_CLASSPATH_ENV = "AUTODEBUGGER_MODEL_COMMON_CLASSPATH";

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
     * Executes analysis on the instrumented application and returns analysis artifacts.
     * This method only performs the instrumentation execution and trace collection.
     *
     * The Collector (running in the DiSL process) is responsible for:
     * - Collecting runtime values
     * - Building the Trace
     * - Serializing the Trace to disk
     *
     * This method orchestrates the execution and validates the output.
     */
    @Override
    public AnalysisResult executeAnalysis(InstrumentationResult instrumentation) {
        log.info("Starting Java analysis on instrumented application: {}", instrumentation);

        validateInstrumentation(instrumentation);
        Path instrumentationJarPath = instrumentation.getPrimaryArtifact();
        try {
            List<String> command = buildExecutionCommand(instrumentationJarPath);
            int exitCode = runCommandAsProcess(command);

            if (exitCode != 0) {
                log.error("Analysis process failed with exit code: {}", exitCode);
                throw new RuntimeException("DiSL analysis failed with exit code: " + exitCode);
            }
        } catch (IOException | InterruptedException e) {
            log.error("Failed to execute instrumented application", e);
            throw new RuntimeException("Analysis execution failed", e);
        }

        // Validate analysis produced output
        Path traceFilePath = instrumentation.getTraceFilePath();
        Path identifierMappingPath = instrumentation.getIdentifiersMappingPath();
        validateAnalysisOutput(traceFilePath, identifierMappingPath);

        return AnalysisResult.builder()
                .traceFilePath(traceFilePath)
                .identifiersMappingPath(identifierMappingPath)
                .outputDirectory(runConfiguration.getOutputDirectory())
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

        // Add the evaluation (Shadow VM) classpath via JVM options
        // Include the instrumentation JAR so Shadow VM can find the Collector class
        // Convert relative paths to absolute paths since DiSL runs from output directory
        // DiSL script requires '=' syntax for arguments starting with dash (see disl.py help)
        // Format: -e_opts=-cp -e_opts=<classpath>
        String evaluationClasspath = getEvaluationClasspath(instrumentationJarPath);
        command.add("-e_opts=-cp");
        command.add("-e_opts=" + evaluationClasspath);

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
     * Validates that analysis produced required output files.
     */
    private void validateAnalysisOutput(Path traceFilePath, Path identifierMappingPath) {
        if (traceFilePath == null) {
            throw new IllegalStateException(
                "Trace file path is null - DiSL execution may not have configured trace output");
        }
        if (!Files.exists(traceFilePath)) {
            throw new IllegalStateException(
                "Trace file not created after DiSL execution: " + traceFilePath +
                ". Check DiSL process logs for errors.");
        }
        if (identifierMappingPath == null || !Files.exists(identifierMappingPath)) {
            throw new IllegalStateException(
                "Identifier mapping file not found: " + identifierMappingPath);
        }
    }

    /**
     * Constructs the classpath for the ShadowVM (evaluation JVM).
     * Includes the instrumentation JAR and model-common for Trace serialization.
     *
     * @param instrumentationJarPath Path to the instrumentation JAR
     * @return Classpath string for the ShadowVM
     */
    private static String getEvaluationClasspath(Path instrumentationJarPath) {
        Path instrumentationJarAbsolutePath = instrumentationJarPath.toAbsolutePath();

        // Try to resolve model-common classpath
        Optional<String> modelCommonClasspath = resolveModelCommonClasspath();

        if (modelCommonClasspath.isPresent()) {
            return instrumentationJarAbsolutePath + ":" + modelCommonClasspath.get();
        } else {
            log.warn("model-common classpath not found. ShadowVM will only have instrumentation JAR. " +
                    "Trace serialization may fail if model-common classes are needed.");
            return instrumentationJarAbsolutePath.toString();
        }
    }

    /**
     * Resolves the model-common classpath using a multi-strategy approach:
     * 1. Check AUTODEBUGGER_MODEL_COMMON_CLASSPATH environment variable
     * 2. Try to find model-common relative to repository root (found via .git directory)
     * 3. Try from current working directory
     *
     * @return Optional containing the model-common classpath, or empty if not found
     */
    private static Optional<String> resolveModelCommonClasspath() {
        // Strategy 1: Check environment variable
        String envClasspath = System.getenv(MODEL_COMMON_CLASSPATH_ENV);
        if (envClasspath != null && !envClasspath.isBlank()) {
            Path envPath = Path.of(envClasspath);
            if (Files.exists(envPath)) {
                log.debug("Using model-common classpath from environment variable: {}", envClasspath);
                return Optional.of(envClasspath);
            } else {
                log.warn("Environment variable {} points to non-existent path: {}",
                        MODEL_COMMON_CLASSPATH_ENV, envClasspath);
            }
        }

        // Strategy 2: Try to find relative to repository root
        Optional<Path> repoRoot = findRepositoryRoot();
        if (repoRoot.isPresent()) {
            // Try Java/auto-debugger/model-common first (full project structure)
            Path modelCommonFull = repoRoot.get().resolve("Java/auto-debugger/model-common/build/libs");
            Optional<String> classpath = listJarsInDirectory(modelCommonFull);
            if (classpath.isPresent()) {
                log.debug("Found model-common classpath relative to repository root: {}", classpath.get());
                return classpath;
            }
            // Also try model-common directly (if running from auto-debugger directory)
            Path modelCommonPath = repoRoot.get().resolve("model-common/build/libs");
            classpath = listJarsInDirectory(modelCommonPath);
            if (classpath.isPresent()) {
                log.debug("Found model-common classpath relative to repository root (direct): {}", classpath.get());
                return classpath;
            }
        }

        // Strategy 3: Try from current working directory
        Path cwdModelCommon = Path.of("model-common/build/libs");
        Optional<String> classpath = listJarsInDirectory(cwdModelCommon);
        if (classpath.isPresent()) {
            log.debug("Found model-common classpath relative to current directory: {}", classpath.get());
            return classpath;
        }

        log.warn("Could not resolve model-common classpath. Tried: " +
                "1) {} environment variable, " +
                "2) repository root relative path, " +
                "3) current directory relative path",
                MODEL_COMMON_CLASSPATH_ENV);
        return Optional.empty();
    }

    /**
     * Lists all JAR files in a directory and returns them as a colon-separated classpath.
     */
    private static Optional<String> listJarsInDirectory(Path directory) {
        if (!Files.isDirectory(directory)) {
            return Optional.empty();
        }

        try (var files = Files.list(directory)) {
            String jars = files
                    .filter(p -> p.toString().endsWith(".jar"))
                    .map(Path::toAbsolutePath)
                    .map(Path::toString)
                    .reduce((a, b) -> a + ":" + b)
                    .orElse(null);

            if (jars != null && !jars.isEmpty()) {
                return Optional.of(jars);
            }
        } catch (IOException e) {
            log.warn("Failed to list JARs in directory: {}", directory, e);
        }

        return Optional.empty();
    }

    /**
     * Finds the repository root by walking up the directory tree looking for .git directory.
     *
     * @return Optional containing the repository root path, or empty if not found
     */
    private static Optional<Path> findRepositoryRoot() {
        Path current = Path.of("").toAbsolutePath();

        // Walk up the directory tree looking for .git
        while (current != null) {
            if (Files.isDirectory(current.resolve(".git"))) {
                return Optional.of(current);
            }
            current = current.getParent();
        }

        return Optional.empty();
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
