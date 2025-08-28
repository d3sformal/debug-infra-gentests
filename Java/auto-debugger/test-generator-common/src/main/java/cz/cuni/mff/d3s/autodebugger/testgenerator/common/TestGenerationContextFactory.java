package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;

/**
 * Factory class for creating TestGenerationContext instances from RunConfiguration.
 * This factory eliminates the need for manual string-based construction by leveraging
 * the structured identifiers already present in RunConfiguration.
 */
public class TestGenerationContextFactory {

    protected TestGenerationContextFactory() { }

    /**
     * Creates a TestGenerationContext from a RunConfiguration.
     * Uses the structured identifiers in the configuration to avoid manual string construction.
     *
     * @param runConfiguration The run configuration containing structured identifiers
     * @return A TestGenerationContext with information extracted from the configuration
     * @throws IllegalArgumentException if the configuration is not supported
     */
    public static TestGenerationContext createFromRunConfiguration(RunConfiguration runConfiguration) {
        return createFromRunConfiguration(runConfiguration, TestGenerationSettings.defaultSettings());
    }

    /**
     * Creates a TestGenerationContext from a RunConfiguration with custom settings.
     *
     * @param runConfiguration The run configuration containing structured identifiers
     * @param settings Custom test generation settings
     * @return A TestGenerationContext with the specified settings
     * @throws IllegalArgumentException if the configuration is not supported
     */
    public static TestGenerationContext createFromRunConfiguration(
            RunConfiguration runConfiguration,
            TestGenerationSettings settings) {

        if (runConfiguration == null) {
            throw new IllegalArgumentException("RunConfiguration cannot be null");
        }

        if (settings == null) {
            settings = TestGenerationSettings.defaultSettings();
        }

        MethodIdentifier targetMethod = runConfiguration.getTargetMethod();

        TestGenerationContext.TestGenerationContextBuilder builder = TestGenerationContext.builder()
                .outputDirectory(runConfiguration.getOutputDirectory())
                .testFramework(settings.getTestFramework())
                .maxTestCount(settings.getMaxTestCount())
                .generateEdgeCases(settings.isGenerateEdgeCases())
                .generateNegativeTests(settings.isGenerateNegativeTests())
                .namingStrategy(settings.getNamingStrategy())
                .includePerformanceAssertions(settings.isIncludePerformanceAssertions())
                .testTimeoutMs(settings.getTestTimeoutMs())
                .objectCreationStrategy(settings.getObjectCreationStrategy())
                .generateParameterizedTests(settings.isGenerateParameterizedTests());

        // Use structured identifiers only; do not set deprecated string fields
        if (targetMethod != null) {
            builder.targetMethod(targetMethod);
        }
        return builder.build();
    }

    /**
     * Create a context directly from strings available in the Collector (package, class, simple signature, return type).
     * This avoids a dependency on model-java identifiers in the Collector.
     */
    public static TestGenerationContext createFromStrings(
            String packageName,
            String className,
            String simpleMethodSignature,
            String returnType,
            java.nio.file.Path outputDirectory) {

        if (className == null || className.isEmpty()) {
            className = "UnknownClass";
        }
        if (packageName == null) {
            packageName = "";
        }
        if (simpleMethodSignature == null || simpleMethodSignature.isEmpty()) {
            simpleMethodSignature = "unknown()";
        }
        if (returnType == null || returnType.isEmpty()) {
            returnType = "void";
        }

        // Parse simpleMethodSignature: name(param1, param2)
        String methodName = simpleMethodSignature;
        java.util.List<String> params = new java.util.ArrayList<>();
        int paren = simpleMethodSignature.indexOf('(');
        if (paren > 0) {
            methodName = simpleMethodSignature.substring(0, paren);
            int close = simpleMethodSignature.lastIndexOf(')');
            if (close > paren + 1) {
                String inside = simpleMethodSignature.substring(paren + 1, close);
                for (String p : inside.split(",")) {
                    String t = p.trim();
                    if (!t.isEmpty()) { params.add(t); }
                }
            }
        }

        final String fPackage = packageName;
        final String fClass = className;
        final String fMethod = methodName;
        final String fReturn = returnType;
        final java.util.List<String> fParams = java.util.Collections.unmodifiableList(params);

        MethodIdentifier id = new MethodIdentifier(fMethod, fReturn, fParams) {
            @Override
            public String getFullyQualifiedSignature() {
                return getFullyQualifiedClassName() + "." + fMethod + "(" + String.join(", ", fParams) + ")";
            }

            @Override
            public String getFullyQualifiedClassName() {
                return (fPackage == null || fPackage.isEmpty()) ? fClass : (fPackage + "." + fClass);
            }

            @Override
            public String getPackageName() {
                return fPackage == null ? "" : fPackage;
            }

            @Override
            public String getClassName() {
                return fClass;
            }
        };

        return TestGenerationContext.builder()
                .targetMethod(id)
                .outputDirectory(outputDirectory)
                .build();
    }

    /**
     * Creates a TestGenerationContext with minimal information for testing purposes.
     *
     * @param targetMethod The target method signature
     * @param outputDirectory The output directory for generated tests
     * @return A TestGenerationContext with minimal configuration
     */
    public static TestGenerationContext createMinimal(MethodIdentifier targetMethod,
                                                     java.nio.file.Path outputDirectory) {
        return TestGenerationContext.builder()
                .targetMethod(targetMethod)
                .outputDirectory(outputDirectory)
                .build();
    }
}
