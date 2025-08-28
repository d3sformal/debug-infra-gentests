package cz.cuni.mff.d3s.autodebugger.intellijplugin;

import cz.cuni.mff.d3s.autodebugger.runner.orchestrator.Orchestrator;
import cz.cuni.mff.d3s.autodebugger.intellijplugin.services.OutputService;

class OrchestratorRunner {
    void run(Orchestrator orchestrator, OutputService outputService) {
        // Phase 1: build instrumentation model
        outputService.print(OutputService.OutputType.TOOL_OUTPUT, "Phase 1: Building instrumentation model...");
        var model = orchestrator.buildInstrumentationModel();
        // Phase 2: generate instrumentation
        outputService.print(OutputService.OutputType.TOOL_OUTPUT, "Phase 2: Generating instrumentation...");
        var instrumentation = orchestrator.createInstrumentation(model);
        // Phase 3: run analysis (DiSL execution); this may take time
        outputService.print(OutputService.OutputType.ANALYSIS_RUN, "Phase 3: Running analysis on instrumented target...");
        var suite = orchestrator.runAnalysis(instrumentation);
        // Phase 4: run tests
        outputService.print(OutputService.OutputType.TESTS, "Phase 4: Running generated tests...");
        var results = orchestrator.runTests(suite);
        outputService.print(OutputService.OutputType.TESTS, "Test execution completed: " + results.getOverallStatus());
    }
}

