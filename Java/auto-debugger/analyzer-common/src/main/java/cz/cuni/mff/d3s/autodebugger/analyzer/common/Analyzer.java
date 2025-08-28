package cz.cuni.mff.d3s.autodebugger.analyzer.common;

import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
import cz.cuni.mff.d3s.autodebugger.model.common.tests.TestSuite;

/**
 * Interface for analyzing instrumented applications and collecting runtime traces.
 * Implementations should handle the execution of instrumented applications and
 * process the collected data into structured traces.
 */
public interface Analyzer {

    /**
     * Runs analysis on the instrumented application and generates tests inside the analysis process.
     *
     * @param instrumentation Instrumentation artifacts and conventions produced by the instrumentor
     * @return TestSuite of generated test files produced by the analysis process
     */
    TestSuite runAnalysis(InstrumentationResult instrumentation);

    /**
     * Validates that the analyzer can process the given instrumentation.
     *
     * @param instrumentation Instrumentation artifacts and conventions produced by the instrumentor
     * @throws IllegalArgumentException if the instrumentation is not compatible
     */
    void validateInstrumentation(InstrumentationResult instrumentation);
}
