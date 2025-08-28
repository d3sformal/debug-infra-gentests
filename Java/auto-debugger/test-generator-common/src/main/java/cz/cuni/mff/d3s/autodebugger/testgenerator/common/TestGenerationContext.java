package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;
import lombok.Builder;
import lombok.Getter;
import lombok.Singular;

import java.nio.file.Path;
import java.util.List;
import java.util.Map;

/**
 * Context information for test generation, providing additional metadata
 * and configuration options for different test generation strategies.
 */
@Builder
@Getter
public class TestGenerationContext {
    
    /**
     * Structured identifier for the target method.
     * This provides type-safe access to method information without string parsing.
     * Preferred over the deprecated string-based fields.
     */
    private final MethodIdentifier targetMethod;
    
    /**
     * Output directory where generated tests should be placed.
     */
    private final Path outputDirectory;
    
    /**
     * Test framework to use (e.g., "junit5", "junit4", "testng").
     */
    @Builder.Default
    private final String testFramework = "junit5";
    
    /**
     * Additional classpath entries needed for test compilation.
     */
    @Singular
    private final List<Path> classpathEntries;
    
    /**
     * Environment variables that may affect test generation.
     */
    @Singular
    private final Map<String, String> environmentVariables;
    
    /**
     * Maximum number of tests to generate.
     */
    @Builder.Default
    private final int maxTestCount = 50;
    
    /**
     * Whether to generate edge case tests.
     */
    @Builder.Default
    private final boolean generateEdgeCases = true;
    
    /**
     * Whether to generate negative test cases (testing error conditions).
     */
    @Builder.Default
    private final boolean generateNegativeTests = true;
    
    /**
     * Test naming strategy to use.
     */
    @Builder.Default
    private final TestNamingStrategy namingStrategy = TestNamingStrategy.DESCRIPTIVE;
    
    /**
     * Additional metadata for test generation.
     */
    @Singular("metadataEntry")
    private final Map<String, Object> metadata;
    
    /**
     * Whether to include performance assertions in generated tests.
     */
    @Builder.Default
    private final boolean includePerformanceAssertions = false;
    
    /**
     * Timeout for individual test execution (in milliseconds).
     */
    @Builder.Default
    private final long testTimeoutMs = 5000;
    
    /**
     * Strategy for handling complex object creation in tests.
     */
    @Builder.Default
    private final ObjectCreationStrategy objectCreationStrategy = ObjectCreationStrategy.SIMPLE;
    
    /**
     * Whether to generate parameterized tests when applicable.
     */
    @Builder.Default
    private final boolean generateParameterizedTests = true;

    // Convenience computed getters

    /**
     * Returns the fully-qualified method signature from the structured identifier.
     * @throws IllegalStateException if targetMethod is null
     */
    public String getTargetMethodSignature() {
        if (targetMethod == null) {
            throw new IllegalStateException("targetMethod is not set in TestGenerationContext");
        }
        return targetMethod.getFullyQualifiedSignature();
    }

    /**
     * Returns the fully qualified class name containing the target method.
     * @throws IllegalStateException if targetMethod is null
     */
    public String getTargetClassName() {
        if (targetMethod == null) {
            throw new IllegalStateException("targetMethod is not set in TestGenerationContext");
        }
        return targetMethod.getFullyQualifiedClassName();
    }

    /**
     * Returns the package name of the target method's class.
     * @throws IllegalStateException if targetMethod is null
     */
    public String getPackageName() {
        if (targetMethod == null) {
            throw new IllegalStateException("targetMethod is not set in TestGenerationContext");
        }
        return targetMethod.getPackageName();
    }
}
