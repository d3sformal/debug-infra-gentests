package cz.cuni.mff.d3s.autodebugger.model.java;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.*;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Path;
import java.util.Arrays;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test class for JavaRunConfiguration TestGenerationContext creation methods.
 * These tests verify that the RunConfiguration can create TestGenerationContext
 * without requiring direct dependencies on test-generator modules.
 */
class JavaRunConfigurationTestGenerationContextTest {

    @TempDir
    Path tempDir;

    private JavaRunConfiguration javaRunConfiguration;

    @BeforeEach
    void setUp() {
        // Create a complete Java method identifier
        JavaPackageIdentifier packageIdentifier = new JavaPackageIdentifier("com.example.service");
        JavaClassIdentifier classIdentifier = new JavaClassIdentifier(
            ClassIdentifierParameters.builder()
                .packageIdentifier(packageIdentifier)
                .className("UserService")
                .build()
        );
        
        JavaMethodIdentifier methodIdentifier = new JavaMethodIdentifier(
            MethodIdentifierParameters.builder()
                .ownerClassIdentifier(classIdentifier)
                .methodName("authenticate")
                .returnType("boolean")
                .parameterTypes(Arrays.asList("java.lang.String", "java.lang.String"))
                .build()
        );

        // Create Java run configuration
        javaRunConfiguration = JavaRunConfiguration.builder()
                .applicationPath(Path.of("test-app.jar"))
                .sourceCodePath(Path.of("src/main/java"))
                .targetMethod(methodIdentifier)
                .outputDirectory(tempDir)
                .build();
    }

    @Test
    void givenMissingTestGeneratorModule_whenCreatingContext_thenThrows() {
        // When test-generator-java module is not available, should throw UnsupportedOperationException
        // This test verifies the expected behavior when the module is not on the classpath

        UnsupportedOperationException exception = assertThrows(UnsupportedOperationException.class, () -> {
            javaRunConfiguration.createTestGenerationContext();
        });

        assertTrue(exception.getMessage().contains("JavaTestGenerationContextFactory not found"));
        assertTrue(exception.getCause() instanceof ClassNotFoundException);
    }

    @Test
    void givenCustomSettings_whenCreatingContext_thenThrows() {
        // Test creating context with custom settings
        // Should throw UnsupportedOperationException when module is not available

        UnsupportedOperationException exception = assertThrows(UnsupportedOperationException.class, () -> {
            javaRunConfiguration.createTestGenerationContext(null);
        });

        assertTrue(exception.getMessage().contains("JavaTestGenerationContextFactory") ||
                  exception.getMessage().contains("TestGenerationSettings"));
        assertTrue(exception.getCause() instanceof ClassNotFoundException);
    }

    @Test
    void givenReflectionBasedApproach_whenClassNotFound_thenThrows() {
        // Test that the reflection-based approach correctly throws exception when class not found
        UnsupportedOperationException exception = assertThrows(UnsupportedOperationException.class, () -> {
            javaRunConfiguration.createTestGenerationContext();
        });

        // Verify the exception contains the expected message and cause
        assertTrue(exception.getMessage().contains("JavaTestGenerationContextFactory not found"));
        assertNotNull(exception.getCause());
        assertTrue(exception.getCause() instanceof ClassNotFoundException);
    }

    @Test
    void givenNullTargetMethod_whenCreatingContext_thenThrows() {
        // Test behavior when target method is null - should still throw UnsupportedOperationException
        // because the module is not available, regardless of the method being null
        JavaRunConfiguration configWithNullMethod = JavaRunConfiguration.builder()
                .applicationPath(Path.of("test-app.jar"))
                .sourceCodePath(Path.of("src/main/java"))
                .targetMethod(null)
                .outputDirectory(tempDir)
                .build();

        UnsupportedOperationException exception = assertThrows(UnsupportedOperationException.class, () -> {
            configWithNullMethod.createTestGenerationContext();
        });

        assertTrue(exception.getMessage().contains("JavaTestGenerationContextFactory not found"));
        assertTrue(exception.getCause() instanceof ClassNotFoundException);
    }
}
