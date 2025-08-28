package cz.cuni.mff.d3s.autodebugger.runner.orchestrator;

import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;

/**
 * Java-specific orchestrator implementation.
 * This is a specialized version of the Orchestrator for Java language support.
 * Currently, it delegates to the base Orchestrator class but provides a specific
 * type for testing and future Java-specific customizations.
 */
public class JavaOrchestrator extends Orchestrator {

    public JavaOrchestrator(Arguments arguments) {
        super(arguments);
    }

    /**
     * Default constructor for testing purposes.
     * Creates a JavaOrchestrator with dummy arguments.
     */
    public JavaOrchestrator() {
        super(createDummyJavaArguments());
    }

    /**
     * Creates dummy arguments for Java testing.
     */
    private static Arguments createDummyJavaArguments() {
        try {
            // Create temporary directories and files for testing
            java.nio.file.Path tempDir = java.nio.file.Files.createTempDirectory("autodebugger-test");
            java.nio.file.Path dummyJar = tempDir.resolve("dummy.jar");
            java.nio.file.Files.createFile(dummyJar);

            java.nio.file.Path sourceDir = tempDir.resolve("src");
            java.nio.file.Files.createDirectories(sourceDir);

            java.nio.file.Path dislHome = tempDir.resolve("disl");
            java.nio.file.Files.createDirectories(dislHome);

            // Create required DiSL structure
            java.nio.file.Path dislBin = dislHome.resolve("bin");
            java.nio.file.Files.createDirectories(dislBin);
            java.nio.file.Path dislPy = dislBin.resolve("disl.py");
            java.nio.file.Files.createFile(dislPy);

            java.nio.file.Path dislOutputLib = dislHome.resolve("output/lib");
            java.nio.file.Files.createDirectories(dislOutputLib);

            Arguments args = new Arguments();
            args.language = cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage.JAVA;
            args.applicationJarPath = dummyJar.toString();
            args.sourceCodePath = sourceDir.toString();
            args.dislHomePath = dislHome.toString();
            args.targetMethodReference = "org.example.Test.method()";
            args.testGenerationStrategy = "trace-based-basic";
            args.classpath = java.util.List.of();
            args.targetParameters = java.util.List.of();
            args.targetFields = java.util.List.of();
            args.runtimeArguments = java.util.List.of();
            return args;
        } catch (java.io.IOException e) {
            throw new RuntimeException("Failed to create dummy arguments for testing", e);
        }
    }
}
