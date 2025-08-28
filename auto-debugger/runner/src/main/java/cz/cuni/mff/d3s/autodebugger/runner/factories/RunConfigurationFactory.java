package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaMethodIdentifier;
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

    private static JavaRunConfiguration createJavaRunConfiguration(Arguments arguments) {
        log.info("Creating Java run configuration from arguments");

        try {
            // Parse paths
            Path applicationPath = Path.of(arguments.applicationJarPath);
            Path sourceCodePath = Path.of(arguments.sourceCodePath);
            Path outputDir = arguments.outputDirectory != null ? Path.of(arguments.outputDirectory) : Path.of("auto-debugger-output");
            Path dislHomePath = Path.of(arguments.dislHomePath);
            List<Path> classpathEntries = arguments.classpath.stream().map(Path::of).toList();

            JavaMethodSignatureParser parser = new JavaMethodSignatureParser();

            // Parse the method reference
            JavaMethodIdentifier methodIdentifier = parser.parseMethodReference(arguments.targetMethodReference);

            // Convert target parameters and fields to ExportableValues
            List<JavaValueIdentifier> parameterValues = parser.parseTargetParameters(arguments.targetParameters, methodIdentifier);
            List<JavaValueIdentifier> fieldValues = parser.parseTargetFields(arguments.targetFields, methodIdentifier);

            // Combine all exportable values
            List<JavaValueIdentifier> exportableValues = new ArrayList<>();
            exportableValues.addAll(parameterValues);
            exportableValues.addAll(fieldValues);

            // Determine trace mode
            TraceMode traceMode = (arguments.traceMode != null && arguments.traceMode.equalsIgnoreCase("temporal"))
                    ? TraceMode.TEMPORAL
                    : TraceMode.NAIVE;

            // Create the Java run configuration
            JavaRunConfiguration configuration = JavaRunConfiguration.builder()
                    .applicationPath(applicationPath)
                    .sourceCodePath(sourceCodePath)
                    .targetMethod(methodIdentifier)
                    .exportableValues(exportableValues)
                    .runtimeArguments(arguments.runtimeArguments)
                    .classpathEntries(classpathEntries)
                    .dislHomePath(dislHomePath)
                    .outputDirectory(outputDir)
                    .traceMode(traceMode)
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
}
