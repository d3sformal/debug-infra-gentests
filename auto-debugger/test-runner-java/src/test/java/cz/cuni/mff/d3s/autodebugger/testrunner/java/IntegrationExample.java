package cz.cuni.mff.d3s.autodebugger.testrunner.java;

import cz.cuni.mff.d3s.autodebugger.testrunner.common.*;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Integration example showing how test-generator and test-runner components work together.
 */
class IntegrationExample {
    
    @TempDir
    Path tempDir;
    
    @Test
    void demonstrateComponentsCreation() throws Exception {
        // This test demonstrates that the components can be created and configured
        // The actual test execution would require a more complete implementation

        // 1. Demonstrate TestRunnerConfiguration creation
        TestRunnerConfiguration config = TestRunnerConfiguration.builder()
                .workingDirectory(tempDir)
                .classpathEntry(Path.of(System.getProperty("java.class.path")))
                .executionTimeout(Duration.ofMinutes(1))
                .enableInstrumentation(false)
                .captureExecutionTraces(false)
                .testFramework("junit5")
                .generateReports(true)
                .reportOutputDirectory(tempDir.resolve("reports"))
                .build();

        assertNotNull(config);
        assertEquals(tempDir, config.getWorkingDirectory());
        assertEquals("junit5", config.getTestFramework());
        assertFalse(config.isEnableInstrumentation());

        // 2. Demonstrate TestRunner creation and configuration
        JUnitTestRunner testRunner = new JUnitTestRunner();
        assertNotNull(testRunner);

        // Configure the test runner
        assertDoesNotThrow(() -> testRunner.configure(config));

        // 3. Demonstrate executing tests with empty list (should work)
        TestExecutionResult result = testRunner.executeTests(List.of());

        assertNotNull(result);
        assertEquals(TestSuiteStatus.PASSED, result.getOverallStatus());
        assertEquals(0, result.getTotalTestCount());
        assertTrue(result.getTestResults().isEmpty());

        // 4. Demonstrate TestGenerationContext creation
        cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerationContext generationContext =
            cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerationContext.builder()
                .targetMethodSignature("Calculator.add(int, int)")
                .targetClassName("com.example.Calculator")
                .packageName("com.example.test")
                .outputDirectory(tempDir)
                .testFramework("junit5")
                .maxTestCount(10)
                .generateEdgeCases(true)
                .generateNegativeTests(true)
                .build();

        assertNotNull(generationContext);
        assertEquals("Calculator.add(int, int)", generationContext.getTargetMethodSignature());
        assertEquals("com.example.Calculator", generationContext.getTargetClassName());
        assertEquals(10, generationContext.getMaxTestCount());

        System.out.println("✅ Component creation test completed successfully!");
        System.out.println("📊 Components verified:");
        System.out.println("   - TestRunnerConfiguration: ✓");
        System.out.println("   - JUnitTestRunner: ✓");
        System.out.println("   - TestGenerationContext: ✓");
        System.out.println("   - TestExecutionResult: ✓");
    }
}
