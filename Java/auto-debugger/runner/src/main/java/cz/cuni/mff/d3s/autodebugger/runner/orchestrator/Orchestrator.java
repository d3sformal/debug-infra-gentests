package cz.cuni.mff.d3s.autodebugger.runner.orchestrator;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.InstrumentationModel;
import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
import cz.cuni.mff.d3s.autodebugger.model.common.technique.TestTechniqueConfig;
import cz.cuni.mff.d3s.autodebugger.model.common.tests.TestSuite;

import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;
import cz.cuni.mff.d3s.autodebugger.runner.factories.AnalyzerFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.InstrumentationModelFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.InstrumentorFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.RunConfigurationFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.TestRunnerFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.TestTechniqueConfigFactory;
import cz.cuni.mff.d3s.autodebugger.runner.strategies.TestGenerationStrategy;
import cz.cuni.mff.d3s.autodebugger.runner.strategies.TestGenerationStrategyProvider;
import cz.cuni.mff.d3s.autodebugger.testrunner.common.TestExecutionResult;
import lombok.extern.slf4j.Slf4j;

import java.util.Arrays;

/**
 * Central orchestrator that coordinates the complete auto-debugger workflow.
 * Uses factory pattern to create language-specific components and manages
 * the sequential execution of instrumentation, analysis, test generation, and test execution.
 */
@Slf4j
public class Orchestrator {

    private final RunConfiguration runConfiguration;
    private final TestTechniqueConfig technique;

    public Orchestrator(Arguments arguments) {
        this.runConfiguration = RunConfigurationFactory.createRunConfiguration(arguments);
        this.technique = TestTechniqueConfigFactory.fromArguments(arguments);
        // Validate strategy early (strict mode)
        boolean valid = Arrays.stream(getAvailableTestGenerationTechniques()).anyMatch(s -> s.equals(technique.getId()));
        if (!valid) {
            throw new IllegalArgumentException("Unknown test generation strategy: " + technique.getId());
        }
    }

    public InstrumentationModel buildInstrumentationModel() {
        return InstrumentationModelFactory.buildInstrumentationModel(runConfiguration);
    }

    public InstrumentationResult createInstrumentation(InstrumentationModel instrumentationModel) {
        var instrumentor = InstrumentorFactory.createInstrumentor(runConfiguration, technique);
        return instrumentor.generateInstrumentation(instrumentationModel);
    }

    public TestSuite runAnalysis(InstrumentationResult instrumentation) {
        var analyzer = AnalyzerFactory.createAnalyzer(runConfiguration);
        TestSuite suite = analyzer.runAnalysis(instrumentation);
        if (suite == null || suite.getTestFiles() == null || suite.getTestFiles().isEmpty()) {
            log.warn("Analysis completed but generated no test files. Terminating.");
            throw new IllegalStateException("Analysis produced no test files");
        }
        return suite;
    }

    public TestExecutionResult runTests(TestSuite testSuite) {
        var testRunner = TestRunnerFactory.createTestRunner(runConfiguration);
        return testRunner.executeTests(testSuite.getTestFiles());
    }

    public TargetLanguage getSupportedLanguage() {
        return runConfiguration.getLanguage();
    }

    public String[] getAvailableTestGenerationTechniques() {
        return TestGenerationStrategyProvider.getAvailableStrategies()
                .stream()
                .map(TestGenerationStrategy::getId)
                .toArray(String[]::new);
    }

    public String getTestGenerationStrategy() {
        return technique.getId();
    }
}
