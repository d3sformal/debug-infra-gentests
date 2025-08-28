package cz.cuni.mff.d3s.autodebugger.model.common;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.ExportableValue;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;

import java.nio.file.Path;
import java.util.List;

/**
 * Interface representing a run configuration that contains all the necessary
 * information for instrumenting, analyzing, and generating tests for a target application.
 * This interface provides a language-agnostic contract that can be implemented
 * for different programming languages.
 */
public interface RunConfiguration {

    /**
     * Gets the target language for this run configuration.
     *
     * @return Target language enum value
     */
    TargetLanguage getLanguage();

    /**
     * Gets the path to the application JAR or executable file.
     *
     * @return Path to the application file
     */
    Path getApplicationPath();

    /**
     * Gets the path to the source code directory.
     *
     * @return Path to the source code
     */
    Path getSourceCodePath();

    /**
     * Gets the target method identifier that should be instrumented and analyzed.
     *
     * @return Method identifier for the target method
     */
    MethodIdentifier getTargetMethod();

    /**
     * Gets the list of exportable values (parameters, fields, etc.) that should be tracked.
     *
     * @return List of exportable values to track during execution
     */
    List<? extends ExportableValue> getExportableValues();

    /**
     * Gets the output directory where generated files should be placed.
     *
     * @return Output directory path
     */
    Path getOutputDirectory();

    /**
     * Gets the runtime arguments to pass to the application when running.
     *
     * @return List of runtime arguments
     */
    List<String> getRuntimeArguments();

    /**
     * Selects the trace collection mode used by the analyzer (naive vs temporal).
     * Default should be NAIVE if not specified by concrete implementations.
     */
    default TraceMode getTraceMode() { return TraceMode.NAIVE; }

    /**
     * Validates that this configuration is complete and valid.
     *
     * @throws IllegalStateException if the configuration is invalid
     */
    void validate();

    /**
     * Creates a TestGenerationContext from this RunConfiguration using default settings.
     * This is a convenience method that delegates to the appropriate factory based on the target language.
     *
     * Note: This method requires the test-generator-common module to be available at runtime.
     * If not available, implementations should throw UnsupportedOperationException.
     *
     * @return A TestGenerationContext with default settings
     * @throws UnsupportedOperationException if test generation context creation is not supported
     */
    default Object createTestGenerationContext() {
        throw new UnsupportedOperationException(
            "TestGenerationContext creation not implemented for " + getClass().getSimpleName() +
            ". Use TestGenerationContextFactory.createFromRunConfiguration() instead.");
    }

    /**
     * Creates a TestGenerationContext from this RunConfiguration with custom settings.
     * This is a convenience method that delegates to the appropriate factory based on the target language.
     *
     * Note: This method requires the test-generator-common module to be available at runtime.
     * If not available, implementations should throw UnsupportedOperationException.
     *
     * @param settings Custom test generation settings (implementation-specific type)
     * @return A TestGenerationContext with the specified settings
     * @throws UnsupportedOperationException if test generation context creation is not supported
     */
    default Object createTestGenerationContext(Object settings) {
        throw new UnsupportedOperationException(
            "TestGenerationContext creation with custom settings not implemented for " + getClass().getSimpleName() +
            ". Use TestGenerationContextFactory.createFromRunConfiguration() instead.");
    }
}
