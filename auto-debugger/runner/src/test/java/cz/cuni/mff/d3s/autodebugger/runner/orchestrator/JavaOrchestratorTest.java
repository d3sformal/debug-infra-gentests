package cz.cuni.mff.d3s.autodebugger.runner.orchestrator;

import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class JavaOrchestratorTest {

    @TempDir
    Path tempDir;

    private JavaOrchestrator orchestrator;
    private Arguments testArguments;
    
    @BeforeEach
    void setUp() throws Exception {
        // Create test JAR file
        Path testJar = tempDir.resolve("test.jar");
        Files.createFile(testJar);

        // Create test source directory
        Path sourceDir = tempDir.resolve("src");
        Files.createDirectories(sourceDir);

        // Create DiSL home directory with required structure
        Path dislHome = tempDir.resolve("disl");
        Files.createDirectories(dislHome);
        Files.createDirectories(dislHome.resolve("bin"));
        Files.createFile(dislHome.resolve("bin/disl.py"));
        Files.createDirectories(dislHome.resolve("output/lib"));

        // Setup test arguments
        testArguments = new Arguments();
        testArguments.applicationJarPath = testJar.toString();
        testArguments.sourceCodePath = sourceDir.toString();
        testArguments.dislHomePath = dislHome.toString();
        testArguments.targetMethodReference = "org.example.Calculator.add(int,int)";
        testArguments.targetParameters = List.of("0:int", "1:int");
        testArguments.targetFields = List.of("int:counter");
        testArguments.language = TargetLanguage.JAVA;
        testArguments.testGenerationStrategy = "trace-based-basic";
        testArguments.classpath = List.of();
        testArguments.runtimeArguments = List.of();

        orchestrator = new JavaOrchestrator(testArguments);
    }
    
    @Test
    void givenJavaOrchestrator_whenGetSupportedLanguage_thenReturnsJava() {
        assertEquals(TargetLanguage.JAVA, orchestrator.getSupportedLanguage());
    }

    @Test
    void givenJavaOrchestrator_whenGettingAvailableTechniques_thenReturnsExpectedList() {
        String[] techniques = orchestrator.getAvailableTestGenerationTechniques();

        assertNotNull(techniques);
        assertTrue(techniques.length > 0);

        // Should contain at least the basic trace-based technique
        boolean hasTraceBasic = false;
        for (String technique : techniques) {
            if ("trace-based-basic".equals(technique)) {
                hasTraceBasic = true;
                break;
            }
        }
        assertTrue(hasTraceBasic, "Should contain trace-based-basic technique");
    }
    
    @Test
    void givenJavaOrchestrator_whenBuildingInstrumentationModel_thenSucceeds() {
        // This should work without throwing exceptions
        assertDoesNotThrow(() -> {
            orchestrator.buildInstrumentationModel();
        });
    }
}
