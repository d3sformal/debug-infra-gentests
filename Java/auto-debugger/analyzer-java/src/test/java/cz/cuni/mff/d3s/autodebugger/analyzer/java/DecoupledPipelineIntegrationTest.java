package cz.cuni.mff.d3s.autodebugger.analyzer.java;

import cz.cuni.mff.d3s.autodebugger.instrumentor.java.DiSLInstrumentor;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.DiSLModel;
import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
import cz.cuni.mff.d3s.autodebugger.model.common.tests.TestSuite;
import cz.cuni.mff.d3s.autodebugger.model.common.trace.Trace;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.*;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.io.FileOutputStream;
import java.io.IOException;
import java.io.ObjectOutputStream;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

import static org.junit.jupiter.api.Assertions.*;

/**
 * End-to-end integration test for the decoupled pipeline in analyzer-java.
 * 
 * This test validates the complete workflow:
 * 1. Stage 1 (Instrumentation): DiSLInstrumentor generates files and JAR correctly
 * 2. Stage 2 (Mock Trace): Manually serialize a Trace with known values
 * 3. Stage 3 (Test Generation): Use generateTestsFromExistingTrace() to generate tests
 */
class DecoupledPipelineIntegrationTest {

    @TempDir
    Path tempDir;

    private Path mockDislHome;
    private Path testOutputDirectory;
    private Path sourceCodePath;
    private Path outputDirectory;

    @BeforeEach
    void setUp() throws IOException {
        // Set up directory structure
        mockDislHome = tempDir.resolve("mock-disl");
        testOutputDirectory = tempDir.resolve("generated-code");
        sourceCodePath = tempDir.resolve("src");
        outputDirectory = tempDir.resolve("output");

        Files.createDirectories(testOutputDirectory);
        Files.createDirectories(sourceCodePath);
        Files.createDirectories(outputDirectory);

        // Create mock DiSL structure
        createMockDislStructure();
    }

    private void createMockDislStructure() throws IOException {
        Files.createDirectories(mockDislHome.resolve("bin"));
        Files.createDirectories(mockDislHome.resolve("output/lib"));
        Files.writeString(mockDislHome.resolve("bin/disl.py"), "# mock");
    }

    /**
     * Test Case 1: Verify that DiSLInstrumentor generates expected artifacts
     * (instrumentation JAR, identifier mapping, trace file paths).
     */
    @Test
    void givenTargetMethod_whenGeneratingInstrumentation_thenProducesExpectedArtifacts() throws IOException {
        // Given - Create a simple Calculator.add(int a, int b) target method
        JavaPackageIdentifier packageIdentifier = new JavaPackageIdentifier("com.example");
        JavaClassIdentifier classIdentifier = new JavaClassIdentifier(
                ClassIdentifierParameters.builder()
                        .packageIdentifier(packageIdentifier)
                        .className("Calculator")
                        .build()
        );

        JavaMethodIdentifier methodIdentifier = new JavaMethodIdentifier(
                MethodIdentifierParameters.builder()
                        .ownerClassIdentifier(classIdentifier)
                        .methodName("add")
                        .returnType("int")
                        .parameterTypes(List.of("int", "int"))
                        .build()
        );

        // Create exportable values for both parameters (slot 0 and slot 1)
        JavaArgumentIdentifier arg0 = new JavaArgumentIdentifier(
                ArgumentIdentifierParameters.builder()
                        .argumentSlot(0)
                        .variableType("int")
                        .build()
        );

        JavaArgumentIdentifier arg1 = new JavaArgumentIdentifier(
                ArgumentIdentifierParameters.builder()
                        .argumentSlot(1)
                        .variableType("int")
                        .build()
        );

        List<JavaValueIdentifier> exportableValues = List.of(arg0, arg1);

        // Create a dummy application JAR
        Path appJar = tempDir.resolve("calculator.jar");
        Files.createFile(appJar);

        JavaRunConfiguration runConfiguration = JavaRunConfiguration.builder()
                .applicationPath(appJar)
                .classpathEntry(appJar)
                .dislHomePath(mockDislHome)
                .sourceCodePath(sourceCodePath)
                .outputDirectory(outputDirectory)
                .targetMethod(methodIdentifier)
                .exportableValues(exportableValues)
                .build();

        Files.createDirectories(runConfiguration.getOutputDirectory());

        DiSLInstrumentor instrumentor = DiSLInstrumentor.builder()
                .instrumentationClassName(new JavaClassIdentifier(
                        ClassIdentifierParameters.builder()
                                .packageIdentifier(JavaPackageIdentifier.DEFAULT_PACKAGE)
                                .className("DiSLClass")
                                .build()))
                .runConfiguration(runConfiguration)
                .generatedCodeOutputDirectory(testOutputDirectory)
                .jarOutputPath(tempDir.resolve("instrumentation.jar"))
                .build();

        DiSLModel model = new DiSLModel(methodIdentifier, exportableValues);

        // When - Generate instrumentation
        InstrumentationResult result = instrumentor.generateInstrumentation(model);

        // Then - Verify all expected artifacts exist
        assertNotNull(result, "InstrumentationResult should not be null");
        assertNotNull(result.getPrimaryArtifact(), "Primary artifact (JAR) should not be null");
        assertTrue(Files.exists(result.getPrimaryArtifact()), "Instrumentation JAR should exist");
        
        assertNotNull(result.getIdentifiersMappingPath(), "Identifier mapping path should not be null");
        assertTrue(Files.exists(result.getIdentifiersMappingPath()), "Identifier mapping file should exist");
        
        assertNotNull(result.getTraceFilePath(), "Trace file path should not be null");
        // Note: Trace file doesn't exist yet - it will be created during execution
        
        assertNotNull(result.getResultsListPath(), "Results list path should not be null");
    }

    /**
     * Test Case 2: End-to-end pipeline test with mock trace and test generation.
     *
     * This test validates:
     * 1. Creating instrumentation artifacts
     * 2. Manually creating and serializing a mock Trace with known values
     * 3. Using DiSLAnalyzer.generateTestsFromExistingTrace() to generate tests
     * 4. Verifying the generated test contains the expected values
     */
    @Test
    void givenMockTrace_whenGeneratingTestsFromExistingTrace_thenGeneratesCorrectTests() throws IOException {
        // Given - Set up the same Calculator.add(int a, int b) scenario
        JavaPackageIdentifier packageIdentifier = new JavaPackageIdentifier("com.example");
        JavaClassIdentifier classIdentifier = new JavaClassIdentifier(
                ClassIdentifierParameters.builder()
                        .packageIdentifier(packageIdentifier)
                        .className("Calculator")
                        .build()
        );

        JavaMethodIdentifier methodIdentifier = new JavaMethodIdentifier(
                MethodIdentifierParameters.builder()
                        .ownerClassIdentifier(classIdentifier)
                        .methodName("add")
                        .returnType("int")
                        .parameterTypes(List.of("int", "int"))
                        .build()
        );

        // Create exportable values for both parameters
        JavaArgumentIdentifier arg0 = new JavaArgumentIdentifier(
                ArgumentIdentifierParameters.builder()
                        .argumentSlot(0)
                        .variableType("int")
                        .build()
        );

        JavaArgumentIdentifier arg1 = new JavaArgumentIdentifier(
                ArgumentIdentifierParameters.builder()
                        .argumentSlot(1)
                        .variableType("int")
                        .build()
        );

        List<JavaValueIdentifier> exportableValues = List.of(arg0, arg1);

        // Create identifier mapping and serialize it
        Map<Integer, JavaValueIdentifier> identifierMapping = new HashMap<>();
        identifierMapping.put(0, arg0);  // Key is argumentSlot, not internalId
        identifierMapping.put(1, arg1);

        Path identifierMappingPath = outputDirectory.resolve("identifiers.ser");
        Files.createDirectories(identifierMappingPath.getParent());
        try (FileOutputStream fos = new FileOutputStream(identifierMappingPath.toFile());
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {
            oos.writeObject(identifierMapping);
        }

        // Create a mock Trace with known values: add(42, 17)
        Trace trace = new Trace();
        trace.addIntValue(0, 42);  // First parameter: slot 0, value 42
        trace.addIntValue(1, 17);  // Second parameter: slot 1, value 17

        // Serialize the trace
        Path traceFilePath = outputDirectory.resolve("trace.ser");
        try (FileOutputStream fos = new FileOutputStream(traceFilePath.toFile());
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {
            oos.writeObject(trace);
        }

        // Create dummy application JAR
        Path appJar = tempDir.resolve("calculator.jar");
        Files.createFile(appJar);

        // Create run configuration with trace-based-basic strategy
        JavaRunConfiguration runConfiguration = JavaRunConfiguration.builder()
                .applicationPath(appJar)
                .classpathEntry(appJar)
                .dislHomePath(mockDislHome)
                .sourceCodePath(sourceCodePath)
                .outputDirectory(outputDirectory)
                .targetMethod(methodIdentifier)
                .exportableValues(exportableValues)
                .testGenerationStrategy("trace-based-basic")
                .build();

        // Create DiSLAnalyzer
        DiSLAnalyzer analyzer = new DiSLAnalyzer(runConfiguration);

        // When - Generate tests from the existing trace
        TestSuite testSuite = analyzer.generateTestsFromExistingTrace(traceFilePath, identifierMappingPath);

        // Then - Verify test generation succeeded
        assertNotNull(testSuite, "TestSuite should not be null");
        assertNotNull(testSuite.getTestFiles(), "Test files list should not be null");
        assertFalse(testSuite.getTestFiles().isEmpty(), "Should generate at least one test file");

        // Verify the generated test file exists and contains expected values
        Path generatedTestFile = testSuite.getTestFiles().get(0);
        assertTrue(Files.exists(generatedTestFile), "Generated test file should exist");

        String testContent = Files.readString(generatedTestFile);
        assertNotNull(testContent, "Test content should not be null");
        assertFalse(testContent.isEmpty(), "Test content should not be empty");

        // Verify the test contains the expected values (42 and 17)
        assertTrue(testContent.contains("42"), "Test should contain the value 42");
        assertTrue(testContent.contains("17"), "Test should contain the value 17");

        // Verify it's testing the Calculator.add method
        assertTrue(testContent.contains("Calculator") || testContent.contains("calculator"),
                "Test should reference Calculator class");
        assertTrue(testContent.contains("add"), "Test should reference add method");

        // Verify it's a proper JUnit test
        assertTrue(testContent.contains("@Test") || testContent.contains("test"),
                "Test should contain test annotations or methods");
    }
}
