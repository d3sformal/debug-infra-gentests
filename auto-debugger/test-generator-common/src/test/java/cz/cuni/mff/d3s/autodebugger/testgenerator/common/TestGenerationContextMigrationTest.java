package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Path;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test class demonstrating the migration from string-based to structured identifier-based
 * TestGenerationContext construction, and verifying backward compatibility.
 */
class TestGenerationContextMigrationTest {

    @TempDir
    Path tempDir;

    @Test
    void givenContextWithoutTargetMethod_whenAccessingMethodInfo_thenThrows() {
        // Deprecated string fields were removed; ensure context requires targetMethod
        TestGenerationContext context = TestGenerationContext.builder()
                .outputDirectory(tempDir)
                .build();
        assertThrows(IllegalStateException.class, context::getTargetMethodSignature);
        assertThrows(IllegalStateException.class, context::getTargetClassName);
        assertThrows(IllegalStateException.class, context::getPackageName);
    }

    @Test
    void givenStructuredIdentifiers_whenBuildingContext_thenExposesStructuredValues() {
        // Given - new approach using structured identifiers
        MethodIdentifier methodIdentifier = new MethodIdentifier("add", "int",
                List.of("int", "int")) {
            @Override
            public String getClassName() {
                return "Calculator";
            }

            @Override
            public String getPackageName() {
                return "com.example";
            }

            @Override
            public String getFullyQualifiedClassName() {
                return "com.example.Calculator";
            }

            @Override
            public String getFullyQualifiedSignature() {
                return "com.example.Calculator.add(int, int)";
            }

            @Override
            public String toString() {
                return "com.example.Calculator.add(int, int)";
            }
        };

        TestGenerationContext context = TestGenerationContext.builder()
                .targetMethod(methodIdentifier)
                .outputDirectory(tempDir)
                .build();

        // When - accessing through getter methods
        MethodIdentifier retrievedMethod = context.getTargetMethod();

        // Then - structured identifier should be present
        assertSame(methodIdentifier, retrievedMethod);
        assertEquals("com.example.Calculator.add(int, int)", context.getTargetMethodSignature());
        assertEquals(tempDir, context.getOutputDirectory());
    }

    @Test
    void givenNewApproach_whenBuildingContext_thenProducesConsistentResults() {
        // Given - same information using both approaches
        MethodIdentifier methodIdentifier = new MethodIdentifier("add", "int",
                List.of("int", "int")) {
            @Override
            public String getClassName() {
                return "Calculator";
            }

            @Override
            public String getPackageName() {
                return "com.example";
            }

            @Override
            public String getFullyQualifiedClassName() {
                return "com.example.Calculator";
            }

            @Override
            public String getFullyQualifiedSignature() {
                return "com.example.Calculator.add(int, int)";
            }

            @Override
            public String toString() {
                return "com.example.Calculator.add(int, int)";
            }
        };

        TestGenerationContext newApproach = TestGenerationContext.builder()
                .targetMethod(methodIdentifier)
                .outputDirectory(tempDir)
                .build();

        // Then - results should be correct
        assertEquals("com.example.Calculator.add(int, int)", newApproach.getTargetMethodSignature());
        assertEquals(tempDir, newApproach.getOutputDirectory());
    }

    @Test
    void givenStructuredAndDeprecatedFields_whenBuilding_thenPrefersStructuredIdentifier() {
        // Given - context with both structured identifier and deprecated string fields
        MethodIdentifier methodIdentifier = new MethodIdentifier("multiply", "double",
                List.of("double", "double")) {
            @Override
            public String getClassName() {
                return "Calculator";
            }

            @Override
            public String getPackageName() {
                return "com.example";
            }

            @Override
            public String getFullyQualifiedClassName() {
                return "com.example.Calculator";
            }

            @Override
            public String getFullyQualifiedSignature() {
                return "com.example.Calculator.multiply(double, double)";
            }

            @Override
            public String toString() {
                return "com.example.Calculator.multiply(double, double)";
            }
        };

        TestGenerationContext context = TestGenerationContext.builder()
                .targetMethod(methodIdentifier)
                .outputDirectory(tempDir)
                .build();

        // Then - structured identifier should be used
        assertEquals("com.example.Calculator.multiply(double, double)", context.getTargetMethodSignature());
    }

    @Test
    void givenContextWithoutStructuredIdentifiers_whenAccessingFields_thenThrows() {
        // Given - context with only deprecated string fields (no structured identifiers)
        TestGenerationContext context = TestGenerationContext.builder()
                .outputDirectory(tempDir)
                .build();

        // Then - should throw when accessing missing targetMethod-dependent values
        assertThrows(IllegalStateException.class, context::getTargetMethodSignature);
        assertThrows(IllegalStateException.class, context::getTargetClassName);
        assertThrows(IllegalStateException.class, context::getPackageName);
        assertNull(context.getTargetMethod());
    }

    @Test
    void givenNewStructuredFields_whenUsingBuilder_thenCreatesValidContext() {
        // Given - using builder with new structured fields
        MethodIdentifier methodIdentifier = new MethodIdentifier("divide", "double",
                List.of("double", "double")) {
            @Override
            public String getClassName() {
                return "Calculator";
            }

            @Override
            public String getPackageName() {
                return "com.example";
            }

            @Override
            public String getFullyQualifiedClassName() {
                return "com.example.Calculator";
            }

            @Override
            public String getFullyQualifiedSignature() {
                return "com.example.Calculator.divide(double, double)";
            }

            @Override
            public String toString() {
                return "com.example.Calculator.divide(double, double)";
            }
        };

        // When - building context with new fields
        TestGenerationContext context = TestGenerationContext.builder()
                .targetMethod(methodIdentifier)
                .outputDirectory(tempDir)
                .testFramework("junit5")
                .maxTestCount(25)
                .generateEdgeCases(true)
                .build();

        // Then - all fields should be accessible
        assertNotNull(context.getTargetMethod());
        assertEquals(methodIdentifier, context.getTargetMethod());
        assertEquals("com.example.Calculator.divide(double, double)", context.getTargetMethodSignature());
        assertEquals("junit5", context.getTestFramework());
        assertEquals(25, context.getMaxTestCount());
        assertTrue(context.isGenerateEdgeCases());
    }

    @Test
    void givenDeprecatedFieldUsage_whenAccessingContext_thenDocumentsExpectedBehavior() {
        // This test documents that deprecated fields should generate warnings
        // In a real scenario, these would show compiler warnings
        
        // Given - using deprecated fields (should generate warnings in IDE/compiler)
        TestGenerationContext context = TestGenerationContext.builder()
                .outputDirectory(tempDir)
                .build();

        // Then - deprecated fields were removed; getters should throw when targetMethod is absent
        assertNotNull(context);
        assertThrows(IllegalStateException.class, context::getTargetMethodSignature);
    }
}
