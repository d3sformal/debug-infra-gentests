package cz.cuni.mff.d3s.autodebugger.analyzer.common;

import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
import cz.cuni.mff.d3s.autodebugger.model.common.tests.TestSuite;

/**
 * Interface for analyzing instrumented applications and collecting runtime traces.
 */
public interface Analyzer {

    /**
     * Executes analysis on the instrumented application and returns analysis artifacts.
     * This method only performs the instrumentation execution and trace collection.
     *
     * @param instrumentation Instrumentation artifacts produced by the instrumentor
     * @return AnalysisResult containing paths to trace file and identifier mapping
     */
    default AnalysisResult executeAnalysis(InstrumentationResult instrumentation) {
        throw new UnsupportedOperationException(
            "executeAnalysis() not yet implemented. Use deprecated runAnalysis() or implement this method.");
    }

    /**
     * Runs analysis and generates tests (legacy combined method).
     *
     * @deprecated Use {@link #executeAnalysis(InstrumentationResult)} followed by
     *             Orchestrator.generateTests() for cleaner separation.
     */
    @Deprecated
    TestSuite runAnalysis(InstrumentationResult instrumentation);

    /**
     * Validates that the analyzer can process the given instrumentation.
     */
    void validateInstrumentation(InstrumentationResult instrumentation);
}
