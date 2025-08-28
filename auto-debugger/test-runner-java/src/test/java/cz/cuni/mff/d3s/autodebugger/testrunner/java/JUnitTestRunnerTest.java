package cz.cuni.mff.d3s.autodebugger.testrunner.java;

import cz.cuni.mff.d3s.autodebugger.testrunner.common.*;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Files;
import java.nio.file.Path;
import java.time.Duration;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class JUnitTestRunnerTest {
    
    @TempDir
    Path tempDir;
    
    private JUnitTestRunner testRunner;
    private TestRunnerConfiguration configuration;
    
    @BeforeEach
    void setUp() {
        testRunner = new JUnitTestRunner();
        
        configuration = TestRunnerConfiguration.builder()
                .workingDirectory(tempDir)
                .classpathEntry(Path.of(System.getProperty("java.class.path")))
                .executionTimeout(Duration.ofMinutes(1))
                .enableInstrumentation(false)
                .captureExecutionTraces(false)
                .testFramework("junit5")
                .build();
        
        testRunner.configure(configuration);
    }
    
    @Test
    void givenValidConfiguration_whenConfigure_thenSucceeds() {
        assertDoesNotThrow(() -> testRunner.configure(configuration));
    }
    
    @Test
    void givenEmptyList_whenExecuteTests_thenReportsZeroTests() {
        TestExecutionResult result = testRunner.executeTests(List.of());
        
        assertNotNull(result);
        assertEquals(TestSuiteStatus.PASSED, result.getOverallStatus());
        assertEquals(0, result.getTotalTestCount());
        assertTrue(result.getTestResults().isEmpty());
    }
    
    @Test
    void givenUnconfiguredRunner_whenExecuteTests_thenThrows() {
        JUnitTestRunner unconfiguredRunner = new JUnitTestRunner();
        
        assertThrows(IllegalStateException.class, () -> 
            unconfiguredRunner.executeTests(List.of()));
    }
    
    @Test
    void givenSimpleTestFile_whenExecuteTest_thenReturnsResult() throws Exception {
        // Create a simple test file
        Path testFile = createSimpleTestFile();
        
        TestExecutionResult result = testRunner.executeTest(testFile);
        
        assertNotNull(result);
        assertNotNull(result.getOverallStatus());
        assertNotNull(result.getTestResults());
        assertTrue(result.getTotalExecutionTime().toMillis() >= 0);
    }
    
    private Path createSimpleTestFile() throws Exception {
        String testContent = """
            import org.junit.jupiter.api.Test;
            import static org.junit.jupiter.api.Assertions.*;
            
            public class SimpleTest {
                @Test
                void testSimpleAssertion() {
                    assertEquals(2, 1 + 1);
                }
                
                @Test
                void testTrueAssertion() {
                    assertTrue(true);
                }
            }
            """;
        
        Path testFile = tempDir.resolve("SimpleTest.java");
        Files.write(testFile, testContent.getBytes());
        return testFile;
    }
}
