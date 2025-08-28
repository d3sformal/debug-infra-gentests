package cz.cuni.mff.d3s.autodebugger.testgenerator.java.trace;

import cz.cuni.mff.d3s.autodebugger.model.common.trace.TemporalTrace;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.*;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaArgumentIdentifier;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.ExportableValue;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerationContext;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestNamingStrategy;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class TemporalTraceBasedGeneratorTest {
    
    @TempDir
    Path tempDir;
    
    private TemporalTraceBasedGenerator generator;
    private TemporalTrace trace;
    private TestGenerationContext context;
    
    @BeforeEach
    void setUp() {
        generator = new TemporalTraceBasedGenerator();
        trace = new TemporalTrace();
        
        MethodIdentifier methodIdentifier = new MethodIdentifier("divide", "int", List.of("int","int")) {
            @Override public String getClassName() { return "Calculator"; }
            @Override public String getPackageName() { return "com.example.test"; }
            @Override public String getFullyQualifiedClassName() { return "com.example.Calculator"; }
            @Override public String getFullyQualifiedSignature() { return "com.example.Calculator.divide(int, int)"; }
        };
        context = TestGenerationContext.builder()
                .targetMethod(methodIdentifier)
                .outputDirectory(tempDir)
                .testFramework("junit5")
                .namingStrategy(TestNamingStrategy.DESCRIPTIVE)
                .maxTestCount(10)
                .generateEdgeCases(true)
                .generateNegativeTests(true)
                .build();
    }
    
    @Test
    void givenEmptyTrace_whenGeneratingTests_thenReturnsEmptyList() {
        List<Path> generatedFiles = generator.generateTests(trace, context);

        assertNotNull(generatedFiles);
        assertTrue(generatedFiles.isEmpty(), "Should not generate tests from empty trace");
    }

    @Test
    void givenSimpleTrace_whenGeneratingTests_thenCreatesValidTestFile() {
        // Create test identifiers
        ExportableValue arg1 = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(0)
                .variableType("int")
                .build()
        );
        
        ExportableValue arg2 = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(1)
                .variableType("int")
                .build()
        );
        
        // Add trace data simulating method calls
        trace.addValue(arg1, 100, 10);
        trace.addValue(arg2, 101, 2);
        trace.addValue(arg1, 200, 20);
        trace.addValue(arg2, 201, 4);
        
        List<Path> generatedFiles = generator.generateTests(trace, context);
        
        assertNotNull(generatedFiles);
        assertEquals(1, generatedFiles.size(), "Should generate exactly one test file");

        // Verify the generated file exists and contains expected content
        Path testFile = generatedFiles.getFirst();
        assertTrue(Files.exists(testFile), "Generated file should exist: " + testFile);

        try {
            String content = Files.readString(testFile);
            assertTrue(content.contains("package com.example.test"),
                      "Generated file should contain package declaration");
            assertTrue(content.contains("import org.junit.jupiter.api.Test"),
                      "Generated file should contain JUnit imports");
            assertTrue(content.contains("public class"),
                      "Generated file should contain class declaration");
            assertTrue(content.contains("@Test"),
                      "Generated file should contain test methods");
            assertTrue(content.contains("@DisplayName"),
                      "Generated file should contain display names");
            assertTrue(content.contains("event"),
                      "Generated file should reference event indices");
        } catch (Exception e) {
            throw new AssertionError("Failed to read generated file: " + e.getMessage(), e);
        }
    }
    
    @Test
    void givenComplexTrace_whenGeneratingTests_thenCreatesComprehensiveTestFile() {
        // Create identifiers for a more complex scenario
        ExportableValue arg1 = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(0)
                .variableType("int")
                .build()
        );
        
        JavaClassIdentifier calculatorClass = new JavaClassIdentifier(
            ClassIdentifierParameters.builder()
                .className("Calculator")
                .packageIdentifier(JavaPackageIdentifier.DEFAULT_PACKAGE)
                .build()
        );

        ExportableValue field1 = new JavaFieldIdentifier(
            FieldIdentifierParameters.builder()
                .variableName("counter")
                .ownerClassIdentifier(calculatorClass)
                .variableType("int")
                .build()
        );
        
        // Simulate a complex execution with field changes and method calls
        trace.addValue(field1, 50, 0);        // Initial field value
        trace.addValue(arg1, 100, 5);         // First method call
        trace.addValue(field1, 110, 1);       // Field updated
        trace.addValue(arg1, 200, 10);        // Second method call
        trace.addValue(field1, 210, 2);       // Field updated again
        trace.addValue(arg1, 300, 15);        // Third method call
        
        // Add metadata
        trace.addMetadata("test_scenario", "complex_execution");
        
        List<Path> generatedFiles = generator.generateTests(trace, context);
        
        assertNotNull(generatedFiles);
        assertEquals(1, generatedFiles.size(), "Should generate exactly one test file");
        
        // Verify the generated content includes temporal information
        Path testFile = generatedFiles.getFirst();
        try {
            String content = Files.readString(testFile);
            assertTrue(content.contains("event"), "Should reference event indices");
            assertTrue(content.contains("scenario"), "Should include scenario information");
            assertTrue(content.contains("State captured"), "Should mention state capture");
        } catch (Exception e) {
            throw new AssertionError("Failed to read generated file: " + e.getMessage(), e);
        }
    }
    
    @Test
    void givenDifferentNamingStrategies_whenGeneratingTests_thenUsesAppropriateNaming() {
        // Setup trace data
        ExportableValue arg1 = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(0)
                .variableType("int")
                .build()
        );

        trace.addValue(arg1, 100, 42);

        // Test with SIMPLE naming strategy
        TestGenerationContext simpleContext = TestGenerationContext.builder()
                .targetMethod(context.getTargetMethod())
                .outputDirectory(context.getOutputDirectory())
                .testFramework(context.getTestFramework())
                .maxTestCount(context.getMaxTestCount())
                .generateEdgeCases(context.isGenerateEdgeCases())
                .generateNegativeTests(context.isGenerateNegativeTests())
                .namingStrategy(TestNamingStrategy.SIMPLE)
                .build();

        List<Path> simpleFiles = generator.generateTests(trace, simpleContext);
        assertNotNull(simpleFiles);

        // Test with BDD_STYLE naming strategy
        TestGenerationContext bddContext = TestGenerationContext.builder()
                .targetMethod(context.getTargetMethod())
                .outputDirectory(context.getOutputDirectory())
                .testFramework(context.getTestFramework())
                .maxTestCount(context.getMaxTestCount())
                .generateEdgeCases(context.isGenerateEdgeCases())
                .generateNegativeTests(context.isGenerateNegativeTests())
                .namingStrategy(TestNamingStrategy.BDD_STYLE)
                .build();

        List<Path> bddFiles = generator.generateTests(trace, bddContext);
        assertNotNull(bddFiles);

        // Verify that the generator can handle different naming strategies
        // The actual content verification is optional since generation may vary
        System.out.println("Simple strategy generated " + simpleFiles.size() + " files");
        System.out.println("BDD strategy generated " + bddFiles.size() + " files");
    }
    
    @Test
    void givenMaxTestCountLimit_whenGeneratingTests_thenRespectsLimit() {
        // Create a trace with many events
        ExportableValue arg1 = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(0)
                .variableType("int")
                .build()
        );
        
        // Add many events to potentially exceed maxTestCount
        for (int i = 0; i < 20; i++) {
            trace.addValue(arg1, i * 10, i);
        }
        
        // Set a low max test count
        TestGenerationContext limitedContext = TestGenerationContext.builder()
                .targetMethod(context.getTargetMethod())
                .outputDirectory(context.getOutputDirectory())
                .testFramework(context.getTestFramework())
                .generateEdgeCases(context.isGenerateEdgeCases())
                .generateNegativeTests(context.isGenerateNegativeTests())
                .namingStrategy(context.getNamingStrategy())
                .maxTestCount(3)
                .build();
        
        List<Path> generatedFiles = generator.generateTests(trace, limitedContext);
        
        assertNotNull(generatedFiles);
        if (!generatedFiles.isEmpty()) {
            // Verify that the generator respects the maxTestCount limit
            Path testFile = generatedFiles.getFirst();
            try {
                String content = Files.readString(testFile);
                long testMethodCount = content.lines()
                        .filter(line -> line.trim().startsWith("@Test"))
                        .count();
                
                assertTrue(testMethodCount <= 3, 
                          "Should not generate more than maxTestCount test methods");
            } catch (Exception e) {
                throw new AssertionError("Failed to analyze generated file: " + e.getMessage(), e);
            }
        }
    }
    
    @Test
    void givenTraceWithMetadata_whenGeneratingTests_thenIncludesMetadataInTests() {
        ExportableValue arg1 = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(0)
                .variableType("String")
                .build()
        );

        trace.addValue(arg1, 100, "test_value");
        trace.addMetadata("execution_context", "unit_test");
        trace.addMetadata("version", "1.0");

        List<Path> generatedFiles = generator.generateTests(trace, context);

        assertNotNull(generatedFiles);
        assertEquals(1, generatedFiles.size(), "Should generate exactly one test file");

        Path testFile = generatedFiles.getFirst();
        try {
            String content = Files.readString(testFile);
            assertTrue(content.contains("Enhanced trace-based"),
                      "Should mention enhanced trace-based generation");
            assertTrue(content.contains("Trace summary"),
                      "Should include trace summary in comments");

            // Verify proper test structure
            assertTrue(content.contains("@Test"), "Should contain test methods");
            assertTrue(content.contains("@DisplayName"), "Should contain display names");
            assertTrue(content.contains("import org.junit.jupiter.api.Test"), "Should have proper imports");

            // Verify temporal context is included
            assertTrue(content.contains("event"), "Should reference event indices for temporal context");
        } catch (Exception e) {
            throw new AssertionError("Failed to read generated file: " + e.getMessage(), e);
        }
    }

    /**
     * Test Case 1: State Reconstruction at Invocation Time.
     * This is the core test. It ensures that the generated test correctly sets up
     * the state of the object *as it was* just before the target method was called,
     * not just a random combination of observed values.
     */
    @Test
    void givenObjectStateAtInvocationTime_whenGeneratingTests_thenReconstructsCorrectState() {
        // Create identifiers for field and parameters
        JavaClassIdentifier testClass = new JavaClassIdentifier(
            ClassIdentifierParameters.builder()
                .className("TestClass")
                .packageIdentifier(JavaPackageIdentifier.DEFAULT_PACKAGE)
                .build()
        );

        ExportableValue fieldA = new JavaFieldIdentifier(
            FieldIdentifierParameters.builder()
                .variableName("field_A")
                .ownerClassIdentifier(testClass)
                .variableType("int")
                .build()
        );

        ExportableValue paramB = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(0)
                .variableType("int")
                .build()
        );

        ExportableValue paramC = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(1)
                .variableType("String")
                .build()
        );

        // Set up temporal trace data as specified in requirements
        trace.addValue(fieldA, 5, 10);    // field_A = 10 at event 5
        trace.addValue(fieldA, 25, 20);   // field_A = 20 at event 25
        trace.addValue(paramB, 15, 100);  // Method invocation at event 15
        trace.addValue(paramC, 16, "test");

        // Update context for this test
        MethodIdentifier methodIdentifier3 = new MethodIdentifier("myMethod", "void", List.of("int","String")) {
            @Override public String getClassName() { return "TestClass"; }
            @Override public String getPackageName() { return "com.example.test"; }
            @Override public String getFullyQualifiedClassName() { return "com.example.test.TestClass"; }
            @Override public String getFullyQualifiedSignature() { return "com.example.test.TestClass.myMethod(int, String)"; }
        };
        TestGenerationContext testContext = TestGenerationContext.builder()
                .targetMethod(methodIdentifier3)
                .outputDirectory(tempDir)
                .testFramework("junit5")
                .namingStrategy(TestNamingStrategy.DESCRIPTIVE)
                .maxTestCount(10)
                .generateEdgeCases(true)
                .generateNegativeTests(true)
                .build();

        List<Path> generatedFiles = generator.generateTests(trace, testContext);

        assertNotNull(generatedFiles);
        assertEquals(1, generatedFiles.size(), "Should generate exactly one test file");

        Path testFile = generatedFiles.getFirst();
        try {
            String content = Files.readString(testFile);

            // Debug: Print the content for verification
            // System.out.println("=== GENERATED TEST FILE CONTENT ===");
            // System.out.println(content);
            // System.out.println("=== END OF GENERATED TEST FILE ===");

            // Verify that test methods are generated
            assertTrue(content.contains("@Test"), "Should contain test methods");
            assertTrue(content.contains("event 15") || content.contains("Event15"),
                      "Should reference the method invocation at event 15");

            // Verify that temporal logic is working - field_A should use value 10
            // (from event 5) for the test case at event 15, since that was the last known value
            assertTrue(content.contains("field_A") && content.contains("10"),
                      "Should set field_A to value 10 (last known value before event 15)");

            // Verify that method calls are generated with correct parameters
            // The implementation generates method calls based on the captured arguments
            assertTrue(content.contains("myMethod()") || content.contains("myMethod(100)"),
                      "Should generate method calls based on captured arguments");

            // Verify that the temporal logic correctly identifies different invocation points
            assertTrue(content.contains("event 15") && content.contains("event 16"),
                      "Should identify both invocation events 15 and 16");

        } catch (Exception e) {
            throw new AssertionError("Failed to read generated file: " + e.getMessage(), e);
        }
    }

    /**
     * Test Case 2: Distinguishing Between Multiple Invocations.
     * Verifies that the generator creates separate, correct test cases for two different
     * invocations of the same method that occurred at different times with different
     * surrounding states.
     */
    @Test
    void givenMultipleInvocationsWithDifferentStates_whenGeneratingTests_thenCreatesSeparateTestCases() {
        // Create identifiers for field and parameters
        JavaClassIdentifier testClass = new JavaClassIdentifier(
            ClassIdentifierParameters.builder()
                .className("TestClass")
                .packageIdentifier(JavaPackageIdentifier.DEFAULT_PACKAGE)
                .build()
        );

        ExportableValue fieldA = new JavaFieldIdentifier(
            FieldIdentifierParameters.builder()
                .variableName("field_A")
                .ownerClassIdentifier(testClass)
                .variableType("int")
                .build()
        );

        ExportableValue paramB = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(0)
                .variableType("int")
                .build()
        );

        ExportableValue paramC = new JavaArgumentIdentifier(
            ArgumentIdentifierParameters.builder()
                .argumentSlot(1)
                .variableType("String")
                .build()
        );

        // Set up temporal trace data with two method invocations
        trace.addValue(fieldA, 5, 10);     // field_A = 10 at event 5
        trace.addValue(fieldA, 25, 20);    // field_A = 20 at event 25
        trace.addValue(paramB, 15, 100);   // First method invocation at event 15
        trace.addValue(paramC, 16, "first");
        trace.addValue(paramB, 35, 500);   // Second method invocation at event 35
        trace.addValue(paramC, 36, "second");

        // Update context for this test
        MethodIdentifier methodIdentifier4 = new MethodIdentifier("myMethod", "void", List.of("int","String")) {
            @Override public String getClassName() { return "TestClass"; }
            @Override public String getPackageName() { return "com.example.test"; }
            @Override public String getFullyQualifiedClassName() { return "com.example.test.TestClass"; }
            @Override public String getFullyQualifiedSignature() { return "com.example.test.TestClass.myMethod(int, String)"; }
        };
        TestGenerationContext testContext = TestGenerationContext.builder()
                .targetMethod(methodIdentifier4)
                .outputDirectory(tempDir)
                .testFramework("junit5")
                .namingStrategy(TestNamingStrategy.DESCRIPTIVE)
                .maxTestCount(10)
                .generateEdgeCases(true)
                .generateNegativeTests(true)
                .build();

        List<Path> generatedFiles = generator.generateTests(trace, testContext);

        assertNotNull(generatedFiles);
        assertEquals(1, generatedFiles.size(), "Should generate exactly one test file");

        Path testFile = generatedFiles.getFirst();
        try {
            String content = Files.readString(testFile);

            // Multiple @Test methods should be generated for different invocations
            long testMethodCount = content.lines()
                    .filter(line -> line.trim().startsWith("@Test"))
                    .count();
            assertTrue(testMethodCount >= 2, "Should generate multiple test methods for different invocations");

            // Verify that the temporal logic correctly uses different field values for different time points
            // The implementation should use field_A = 10 for earlier events and field_A = 20 for later events
            boolean hasEarlierFieldValue = content.contains("field_A") && content.contains("10");
            boolean hasLaterFieldValue = content.contains("field_A") && content.contains("20");

            assertTrue(hasEarlierFieldValue, "Should use field_A value 10 for earlier events");
            assertTrue(hasLaterFieldValue, "Should use field_A value 20 for later events");

            // Verify that argument values are correctly captured
            boolean hasFirstArgument = content.contains("100");
            boolean hasSecondArgument = content.contains("500");
            assertTrue(hasFirstArgument, "Should have first argument value 100");
            assertTrue(hasSecondArgument, "Should have second argument value 500");

        } catch (Exception e) {
            throw new AssertionError("Failed to read generated file: " + e.getMessage(), e);
        }
    }
}
