package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.trace.Trace;

import java.nio.file.Path;
import java.util.List;

/**
 * Interface for generating test cases from runtime traces.
 * Implementations should support different test generation strategies
 * and produce executable test files.
 */
public interface TestGenerator {
    
    /**
     * Generates tests from the provided trace data.
     * 
     * @param trace Runtime trace data collected during execution
     * @return List of paths to generated test files
     */
    List<Path> generateTests(Trace trace);
    
    /**
     * Generates tests with additional context information.
     *
     * @param trace Runtime trace data
     * @param sourceCodePath Path to the source code file
     * @param context Additional context for test generation
     * @return List of paths to generated test files
     */
    List<Path> generateTests(Trace trace, Path sourceCodePath, TestGenerationContext context);

    /**
     * Generates tests using RunConfiguration directly.
     * This method creates TestGenerationContext internally from the RunConfiguration,
     * eliminating the need for manual context construction.
     *
     * @param trace Runtime trace data
     * @param configuration Run configuration containing structured identifiers
     * @return List of paths to generated test files
     */
    default List<Path> generateTests(Trace trace, RunConfiguration configuration) {
        // Default implementation creates context from configuration and delegates
        TestGenerationContext context = TestGenerationContextFactory.createFromRunConfiguration(configuration);
        return generateTests(trace, configuration.getSourceCodePath(), context);
    }
    
    /**
     * Gets the test generation technique used by this generator.
     * 
     * @return Test generation technique identifier
     */
    String getGenerationTechnique();

    /**
     * Validates that the generator can process the given trace.
     * 
     * @param trace The trace to validate
     * @throws IllegalArgumentException if the trace is not compatible
     */
    void validateTrace(Trace trace);
}
