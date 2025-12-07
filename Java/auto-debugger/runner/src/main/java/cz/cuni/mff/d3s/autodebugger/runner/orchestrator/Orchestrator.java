package cz.cuni.mff.d3s.autodebugger.runner.orchestrator;

import cz.cuni.mff.d3s.autodebugger.analyzer.common.AnalysisResult;
import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.InstrumentationModel;
import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
import cz.cuni.mff.d3s.autodebugger.model.common.technique.TestTechniqueConfig;
import cz.cuni.mff.d3s.autodebugger.model.common.tests.TestSuite;
import cz.cuni.mff.d3s.autodebugger.model.common.trace.Trace;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;

import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;
import cz.cuni.mff.d3s.autodebugger.runner.factories.AnalyzerFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.InstrumentationModelFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.InstrumentorFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.RunConfigurationFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.TestGeneratorFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.TestRunnerFactory;
import cz.cuni.mff.d3s.autodebugger.runner.factories.TestTechniqueConfigFactory;
import cz.cuni.mff.d3s.autodebugger.runner.strategies.TestGenerationStrategy;
import cz.cuni.mff.d3s.autodebugger.runner.strategies.TestGenerationStrategyProvider;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerationContext;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerator;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.JavaTestGenerationContextFactory;
import cz.cuni.mff.d3s.autodebugger.testrunner.common.TestExecutionResult;
import lombok.extern.slf4j.Slf4j;

import java.io.FileInputStream;
import java.io.ObjectInputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.List;

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

    /**
     * Executes analysis on the instrumented application.
     * This performs DiSL execution and trace collection without generating tests.
     *
     * @param instrumentation Instrumentation artifacts from the instrumentor
     * @return AnalysisResult containing paths to analysis outputs
     */
    public AnalysisResult executeAnalysis(InstrumentationResult instrumentation) {
        var analyzer = AnalyzerFactory.createAnalyzer(runConfiguration);
        AnalysisResult result = analyzer.executeAnalysis(instrumentation);

        if (result == null || result.getTraceFilePath() == null) {
            log.error("Analysis completed but produced no trace file");
            throw new IllegalStateException("Analysis produced no trace file");
        }

        log.info("Analysis completed. Trace at: {}", result.getTraceFilePath());
        return result;
    }

    /**
     * Generates tests from analysis results.
     * This takes analysis outputs and produces a test suite.
     *
     * @param analysisResult Result from executeAnalysis()
     * @return TestSuite containing generated test files
     */
    public TestSuite generateTests(AnalysisResult analysisResult) {
        log.info("Generating tests from analysis result: {}", analysisResult);

        // Deserialize the trace
        Trace trace = deserializeTrace(analysisResult.getTraceFilePath());
        if (trace == null) {
            throw new IllegalStateException("Failed to deserialize trace from: " +
                analysisResult.getTraceFilePath());
        }

        // Create the test generator
        TestGenerator generator = TestGeneratorFactory.createTestGenerator(
            runConfiguration,
            technique.getId(),
            technique.getApiKey());

        // Generate tests
        TestGenerationContext context = createTestGenerationContext();
        List<Path> generatedTests = generator.generateTests(
            trace,
            runConfiguration.getSourceCodePath(),
            context);

        if (generatedTests == null || generatedTests.isEmpty()) {
            log.warn("Test generation completed but produced no test files");
            throw new IllegalStateException("Test generation produced no test files");
        }

        return TestSuite.builder()
                .baseDirectory(analysisResult.getOutputDirectory())
                .testFiles(generatedTests)
                .build();
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

    /**
     * Deserializes a Trace object from a file.
     */
    private Trace deserializeTrace(Path traceFilePath) {
        if (traceFilePath == null || !Files.exists(traceFilePath)) {
            log.warn("Trace file not found: {}", traceFilePath);
            return null;
        }

        log.info("Deserializing trace from: {}", traceFilePath);
        try (FileInputStream fileInput = new FileInputStream(traceFilePath.toFile());
             ObjectInputStream objectInput = new ObjectInputStream(fileInput)) {
            Trace trace = (Trace) objectInput.readObject();
            log.info("Successfully deserialized trace");
            return trace;
        } catch (Exception e) {
            log.error("Failed to deserialize trace from {}", traceFilePath, e);
            return null;
        }
    }

    /**
     * Creates TestGenerationContext from current run configuration.
     */
    private TestGenerationContext createTestGenerationContext() {
        if (runConfiguration instanceof JavaRunConfiguration javaConfig) {
            return JavaTestGenerationContextFactory.createFromJavaRunConfiguration(javaConfig);
        }
        throw new IllegalStateException("Unsupported run configuration type: " +
            runConfiguration.getClass().getSimpleName());
    }
}
