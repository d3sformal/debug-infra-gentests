package cz.cuni.mff.d3s.autodebugger.instrumentor.common;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.InstrumentationModel;
import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;

/**
 * Interface for generating instrumentation code from instrumentation models.
 * Implementations should transform abstract instrumentation models into
 * concrete instrumentation artifacts (e.g., JAR files, source code).
 */
public interface Instrumentor {

    /**
     * Generates instrumentation artifacts from the provided model.
     * Transforms the abstract instrumentation model into concrete files
     * that can be used for runtime analysis.
     *
     * @param model The instrumentation model containing instrumentation specifications
     * @return InstrumentationResult describing produced artifacts and conventions
     */
    InstrumentationResult generateInstrumentation(InstrumentationModel model);
}
