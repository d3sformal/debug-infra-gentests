package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.common.TempPathResolver;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier;
import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;
import cz.cuni.mff.d3s.autodebugger.model.common.TraceMode;
import cz.cuni.mff.d3s.autodebugger.runner.parsing.JavaMethodSignatureParser;
import lombok.extern.slf4j.Slf4j;

import java.nio.file.Path;
import java.util.ArrayList;
import java.util.List;

/**
 * Factory for creating language-specific RunConfiguration instances.
 * Uses factory pattern to abstract the creation of run configurations
 * based on target language, parsing command-line arguments appropriately.
 */
@Slf4j
public class RunConfigurationFactory {

    /**
     * Creates a language-specific RunConfiguration from command-line arguments.
     * Dispatches to appropriate language-specific factory method based on target language.
     */
    public static RunConfiguration createRunConfiguration(Arguments arguments) {
        if (arguments.language == TargetLanguage.JAVA) {
            return createJavaRunConfiguration(arguments);
        }

        throw new IllegalArgumentException("Unsupported language: " + arguments.language);
    }

    private static final String DISL_HOME_ENV = "DISL_HOME";

    private static JavaRunConfiguration createJavaRunConfiguration(Arguments arguments) {
        log.info("Creating Java run configuration from arguments");

        try {
            // Parse paths
            var applicationPath = Path.of(arguments.applicationJarPath);
            var sourceCodePath = Path.of(arguments.sourceCodePath);
            var outputDir = arguments.outputDirectory != null
                ? Path.of(arguments.outputDirectory)
                : TempPathResolver.getDefaultOutputDirectory();
            var dislHomePath = resolveDislHomePath(arguments.dislHomePath);
            var classpathEntries = arguments.classpath != null
                ? arguments.classpath.stream().map(Path::of).toList()
                : List.<Path>of();

            var parser = new JavaMethodSignatureParser();

            // Parse the method reference
            var methodIdentifier = parser.parseMethodReference(arguments.targetMethodReference);

            // Convert target parameters and fields to ExportableValues
            var parameterValues = parser.parseTargetParameters(arguments.targetParameters, methodIdentifier);
            var fieldValues = parser.parseTargetFields(arguments.targetFields, methodIdentifier);

            // Combine all exportable values
            List<JavaValueIdentifier> exportableValues = new ArrayList<>();
            exportableValues.addAll(parameterValues);
            exportableValues.addAll(fieldValues);

            // Determine trace mode
            var traceMode = (arguments.traceMode != null && arguments.traceMode.equalsIgnoreCase("temporal"))
                    ? TraceMode.TEMPORAL
                    : TraceMode.NAIVE;

            // Create the Java run configuration
            var configuration = JavaRunConfiguration.builder()
                    .applicationPath(applicationPath)
                    .sourceCodePath(sourceCodePath)
                    .targetMethod(methodIdentifier)
                    .exportableValues(exportableValues)
                    .runtimeArguments(arguments.runtimeArguments)
                    .classpathEntries(classpathEntries)
                    .dislHomePath(dislHomePath)
                    .outputDirectory(outputDir)
                    .traceMode(traceMode)
                    .testGenerationStrategy(arguments.testGenerationStrategy)
                    .build();

            // Validate the configuration
            configuration.validate();

            log.info("Successfully created Java run configuration for method: {}", methodIdentifier.getName());
            return configuration;

        } catch (Exception e) {
            log.error("Failed to create Java run configuration", e);
            throw new RuntimeException("Failed to create run configuration", e);
        }
    }

    /**
     * Resolves DiSL home path from CLI argument or DISL_HOME environment variable.
     *
     * @param cliDislHomePath CLI argument value (may be null)
     * @return Resolved DiSL home path
     * @throws IllegalArgumentException if DiSL path cannot be resolved
     */
    private static Path resolveDislHomePath(String cliDislHomePath) {
        // 1. CLI argument takes precedence
        if (cliDislHomePath != null && !cliDislHomePath.isBlank()) {
            log.debug("Using DiSL home from CLI argument: {}", cliDislHomePath);
            return Path.of(cliDislHomePath).toAbsolutePath().normalize();
        }

        // 2. Fall back to DISL_HOME environment variable
        String envDislHome = System.getenv(DISL_HOME_ENV);
        if (envDislHome != null && !envDislHome.isBlank()) {
            log.info("Using {} from environment: {}", DISL_HOME_ENV, envDislHome);
            return Path.of(envDislHome).toAbsolutePath().normalize();
        }

        // 3. No DiSL path available
        throw new IllegalArgumentException(
            "DiSL home path not specified. Use --disl-home argument or set " +
            DISL_HOME_ENV + " environment variable.");
    }
}
