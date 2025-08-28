package cz.cuni.mff.d3s.autodebugger.runner.demo;

import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;
import cz.cuni.mff.d3s.autodebugger.runner.orchestrator.OrchestratorFactory;
import lombok.extern.slf4j.Slf4j;

import java.util.List;

/**
 * Demonstration of the new orchestrator-based API for the auto-debugger.
 * This class shows how to use the orchestrator to create and coordinate
 * all the core components for different programming languages.
 */
@Slf4j
public class OrchestratorDemo {
    
    public static void main(String[] args) {
        demonstrateOrchestratorAPI();
    }
    
    public static void demonstrateOrchestratorAPI() {
        log.info("=== Orchestrator API Demonstration ===");
        
        try {
            // Create sample arguments
            Arguments arguments = createSampleArguments();
            
            // Step 1: Create orchestrator for the target language
            log.info("Step 1: Creating orchestrator for language: {}", arguments.language.getDisplayName());
            var orchestrator = OrchestratorFactory.create(arguments.language);
            log.info("✓ Created orchestrator: {}", orchestrator.getClass().getSimpleName());
            
            // Step 2: Build instrumentation model (orchestrator already has run configuration)
            log.info("Step 2: Building instrumentation model");
            var instrumentationModel = orchestrator.buildInstrumentationModel();
            log.info("✓ Built instrumentation model: {}", instrumentationModel.getClass().getSimpleName());
            
            // Step 3: Show available test generation techniques
            log.info("Step 3: Available test generation techniques:");
            String[] techniques = orchestrator.getAvailableTestGenerationTechniques();
            for (String technique : techniques) {
                log.info("  - {}", technique);
            }
            
            // Step 4: Test generation and execution (skipped in demo due to dependencies)
            log.info("Step 4: Test generation and execution (skipped in demo due to dependencies)");
            
            log.info("=== Orchestrator API demonstration completed successfully ===");

        } catch (Exception e) {
            log.error("Orchestrator API demonstration failed", e);
        }
    }

    private static Arguments createSampleArguments() {
        Arguments arguments = new Arguments();
        arguments.applicationJarPath = "demo/sample-app.jar";
        arguments.sourceCodePath = "demo/src/main/java";
        arguments.dislHomePath = "demo/disl";
        arguments.targetMethodReference = "org.example.Calculator.add(int,int)";
        arguments.targetParameters = List.of("0:int", "1:int");
        arguments.targetFields = List.of("int:counter", "String:name");
        arguments.language = TargetLanguage.JAVA;
        arguments.testGenerationStrategy = "trace-based-basic";
        arguments.classpath = List.of();
        arguments.runtimeArguments = List.of();

        log.info("Created sample arguments:");
        log.info("  JAR: {}", arguments.applicationJarPath);
        log.info("  Source: {}", arguments.sourceCodePath);
        log.info("  DiSL Home: {}", arguments.dislHomePath);
        log.info("  Method: {}", arguments.targetMethodReference);
        log.info("  Parameters: {}", arguments.targetParameters);
        log.info("  Fields: {}", arguments.targetFields);
        log.info("  Language: {}", arguments.language.getDisplayName());
        log.info("  Test Strategy: {}", arguments.testGenerationStrategy);

        return arguments;
    }
    
    /**
     * Demonstrates the workflow that would be used in the main application.
     */
    public static void demonstrateMainWorkflow() {
        log.info("=== Main Workflow Demonstration ===");
        
        Arguments arguments = createSampleArguments();
        
        try {
            // This mirrors the workflow in Runner.java
            var orchestrator = new cz.cuni.mff.d3s.autodebugger.runner.orchestrator.Orchestrator(arguments);
            var instrumentationModel = orchestrator.buildInstrumentationModel();

            log.info("Main workflow components created successfully:");
            log.info("  - Orchestrator: {}", orchestrator.getClass().getSimpleName());
            log.info("  - Instrumentation Model: {}", instrumentationModel.getClass().getSimpleName());
            
            // The actual execution would continue with:
            // var instrumentationPath = instrumentor.runInstrumentation();
            // var trace = analyzer.runAnalysis(instrumentationPath);
            // var tests = testGenerator.generateTests(trace);
            // var testResults = testRunner.runTests(tests);
            
            log.info("=== Main workflow demonstration completed ===");
            
        } catch (Exception e) {
            log.error("Main workflow demonstration failed", e);
        }
    }
}
