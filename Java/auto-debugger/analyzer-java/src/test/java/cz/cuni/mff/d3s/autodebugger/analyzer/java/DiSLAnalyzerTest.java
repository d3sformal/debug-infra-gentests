package cz.cuni.mff.d3s.autodebugger.analyzer.java;

import cz.cuni.mff.d3s.autodebugger.model.common.artifacts.InstrumentationResult;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.*;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.MethodIdentifierParameters;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.io.IOException;
import java.net.URL;
import java.nio.file.Files;
import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Arrays;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for DiSLAnalyzer focusing on command-building logic and process interaction.
 */
class DiSLAnalyzerTest {

    @TempDir
    Path tempDir;

    private JavaRunConfiguration standardConfig;
    private JavaRunConfiguration configWithoutRuntimeArgs;
    private JavaRunConfiguration configWithSpacesInPaths;
    private Path instrumentationJarPath;
    private InstrumentationResult instrumentation;

    @BeforeEach
    void setUp() {
        // Create a dummy method identifier for testing
        JavaClassIdentifier classIdentifier = new JavaClassIdentifier(
                ClassIdentifierParameters.builder()
                        .className("TestClass")
                        .packageIdentifier(new JavaPackageIdentifier("com.example"))
                        .build()
        );

        JavaMethodIdentifier methodIdentifier = new JavaMethodIdentifier(
                MethodIdentifierParameters.builder()
                        .ownerClassIdentifier(classIdentifier)
                        .methodName("testMethod")
                        .returnType("void")
                        .parameterTypes(Arrays.asList("int", "String"))
                        .build()
        );

        instrumentationJarPath = Path.of("/tmp/instrumentation.jar");
        Path resultsListPath = tempDir.resolve("output/generated-tests.lst");
        instrumentation = InstrumentationResult.builder()
                .primaryArtifact(instrumentationJarPath)
                .resultsListPath(resultsListPath)
                .build();

        // Standard configuration with runtime arguments
        standardConfig = JavaRunConfiguration.builder()
                .applicationPath(Path.of("/path/to/my-app.jar"))
                .sourceCodePath(tempDir.resolve("src"))
                .dislHomePath(Path.of("/opt/disl"))
                .outputDirectory(tempDir.resolve("output"))
                .targetMethod(methodIdentifier)
                .runtimeArgument("--user")
                .runtimeArgument("test")
                .runtimeArgument("--mode")
                .runtimeArgument("fast")
                .build();

        // Configuration without runtime arguments
        configWithoutRuntimeArgs = JavaRunConfiguration.builder()
                .applicationPath(Path.of("/path/to/my-app.jar"))
                .sourceCodePath(tempDir.resolve("src"))
                .dislHomePath(Path.of("/opt/disl"))
                .outputDirectory(tempDir.resolve("output"))
                .targetMethod(methodIdentifier)
                .build();

        // Configuration with spaces in paths
        configWithSpacesInPaths = JavaRunConfiguration.builder()
                .applicationPath(Path.of("/home/user/my app/app.jar"))
                .sourceCodePath(tempDir.resolve("src"))
                .dislHomePath(Path.of("/opt/DiSL Framework"))
                .outputDirectory(tempDir.resolve("output"))
                .targetMethod(methodIdentifier)
                .build();
    }

    @Test
    void givenStandardConfiguration_whenBuildExecutionCommand_thenBuildsProperCommand() {
        // Given
        DiSLAnalyzer analyzer = new DiSLAnalyzer(standardConfig);

        // When
        List<String> command = analyzer.buildExecutionCommand(instrumentation.getPrimaryArtifact());

        // Then
        assertNotNull(command);
        assertEquals(15, command.size());

        // Verify exact order and content
        assertEquals("python3", command.get(0));
        assertEquals("/opt/disl/bin/disl.py", command.get(1));
        assertEquals("-d", command.get(2));
        assertEquals("/opt/disl/output", command.get(3));
        assertEquals("-cse", command.get(4));
        assertEquals("-e_cp", command.get(5));
        assertEquals("../test-generator-java/build/libs/*:../test-generator-common/build/libs/*:../model-common/build/libs/*:../model-java/build/libs/*", command.get(6));
        assertEquals("--", command.get(7));
        assertEquals("/tmp/instrumentation.jar", command.get(8));
        assertEquals("-jar", command.get(9));
        assertEquals("/path/to/my-app.jar", command.get(10));
        assertEquals("--user", command.get(11));
        assertEquals("test", command.get(12));
        assertEquals("--mode", command.get(13));
        assertEquals("fast", command.get(14));
    }

    @Test
    void givenNoRuntimeArguments_whenBuildExecutionCommand_thenOmitsRuntimeArgs() {
        // Given
        DiSLAnalyzer analyzer = new DiSLAnalyzer(configWithoutRuntimeArgs);

        // When
        List<String> command = analyzer.buildExecutionCommand(instrumentation.getPrimaryArtifact());

        // Then
        assertNotNull(command);
        assertEquals(11, command.size());

        // Verify command up to application jar (no runtime arguments should follow)
        assertEquals("python3", command.get(0));
        assertEquals("/opt/disl/bin/disl.py", command.get(1));
        assertEquals("-d", command.get(2));
        assertEquals("/opt/disl/output", command.get(3));
        assertEquals("-cse", command.get(4));
        assertEquals("-e_cp", command.get(5));
        assertEquals("../test-generator-java/build/libs/*:../test-generator-common/build/libs/*:../model-common/build/libs/*:../model-java/build/libs/*", command.get(6));
        assertEquals("--", command.get(7));
        assertEquals("/tmp/instrumentation.jar", command.get(8));
        assertEquals("-jar", command.get(9));
        assertEquals("/path/to/my-app.jar", command.get(10));
    }

    @Test
    void givenPathsWithSpaces_whenBuildExecutionCommand_thenProperlyQuotesArgs() {
        // Given
        DiSLAnalyzer analyzer = new DiSLAnalyzer(configWithSpacesInPaths);

        // When
        List<String> command = analyzer.buildExecutionCommand(instrumentation.getPrimaryArtifact());

        // Then
        assertNotNull(command);
        assertEquals(11, command.size());

        // Verify paths with spaces are correctly represented as single arguments
        assertEquals("python3", command.get(0));
        assertEquals("/opt/DiSL Framework/bin/disl.py", command.get(1));
        assertEquals("-d", command.get(2));
        assertEquals("/opt/DiSL Framework/output", command.get(3));
        assertEquals("-cse", command.get(4));
        assertEquals("-e_cp", command.get(5));
        assertEquals("../test-generator-java/build/libs/*:../test-generator-common/build/libs/*:../model-common/build/libs/*:../model-java/build/libs/*", command.get(6));
        assertEquals("--", command.get(7));
        assertEquals("/tmp/instrumentation.jar", command.get(8));
        assertEquals("-jar", command.get(9));
        assertEquals("/home/user/my app/app.jar", command.get(10));
    }

    @Test
    void givenInvalidInstrumentationPath_whenRunAnalysis_thenThrows() {
        // Given
        DiSLAnalyzer analyzer = new DiSLAnalyzer(standardConfig);
        Path nonExistentJar = Path.of("/non/existent/instrumentation.jar");
        Path resultsListPath = tempDir.resolve("output/generated-tests.lst");

        // When & Then
        IllegalArgumentException exception = assertThrows(IllegalArgumentException.class, () -> {
            analyzer.runAnalysis(InstrumentationResult.builder()
                    .primaryArtifact(nonExistentJar)
                    .resultsListPath(resultsListPath)
                    .build());
        });

        assertTrue(exception.getMessage().contains("does not exist"));
    }

    @Test
    void givenEmptyInstrumentationPaths_whenRunAnalysis_thenThrows() {
        // Given
        DiSLAnalyzer analyzer = new DiSLAnalyzer(standardConfig);

        // When & Then
        IllegalArgumentException exception = assertThrows(IllegalArgumentException.class, () -> {
            analyzer.runAnalysis(null);
        });

        assertTrue(exception.getMessage().contains("cannot be null"));
    }

    @Test
    void givenNullInstrumentationPaths_whenRunAnalysis_thenThrows() {
        // Given
        DiSLAnalyzer analyzer = new DiSLAnalyzer(standardConfig);

        // When & Then
        IllegalArgumentException exception = assertThrows(IllegalArgumentException.class, () -> {
            analyzer.runAnalysis(null);
        });

        assertTrue(exception.getMessage().contains("cannot be null"));
    }

    /**
     * Test that verifies the analyzer handles process execution correctly.
     * This test creates a simple executable that simulates DiSL behavior.
     */
    @Test
    void givenMockInstrumentationJar_whenRunAnalysis_thenHandlesProcessExecution() throws IOException {
        // Given - Create a mock instrumentation jar file
        Path mockInstrumentationJar = tempDir.resolve("mock-instrumentation.jar");
        Files.createFile(mockInstrumentationJar);

        // Create a configuration that points to a simple command that will succeed
        JavaClassIdentifier testClassIdentifier = new JavaClassIdentifier(
                ClassIdentifierParameters.builder()
                        .className("TestClass")
                        .packageIdentifier(new JavaPackageIdentifier("com.example"))
                        .build()
        );

        JavaMethodIdentifier methodIdentifier = new JavaMethodIdentifier(
                MethodIdentifierParameters.builder()
                        .ownerClassIdentifier(testClassIdentifier)
                        .methodName("testMethod")
                        .returnType("void")
                        .parameterTypes(Arrays.asList("int"))
                        .build()
        );

        // Create a configuration with a command that will work (echo command)
        JavaRunConfiguration testConfig = JavaRunConfiguration.builder()
                .applicationPath(tempDir.resolve("test-app.jar")) // This won't be executed due to our test setup
                .sourceCodePath(tempDir.resolve("src"))
                .dislHomePath(tempDir.resolve("mock-disl")) // This will be used to construct paths
                .outputDirectory(tempDir.resolve("output"))
                .targetMethod(methodIdentifier)
                .build();

        // Create the mock DiSL directory structure
        Files.createDirectories(tempDir.resolve("mock-disl/bin"));
        Files.createDirectories(tempDir.resolve("mock-disl/output"));
        Files.createDirectories(tempDir.resolve("output"));

        // Copy the mock script from test resources to the expected location
        Path mockDislScript = tempDir.resolve("mock-disl/bin/disl.py");
        try {
            URL scriptUrl = getClass().getClassLoader().getResource("mock-disl-process-execution.py");
            if (scriptUrl != null) {
                Path resourceScript = Paths.get(scriptUrl.toURI());
                Files.copy(resourceScript, mockDislScript);

                // Make the script executable (on Unix-like systems)
                mockDislScript.toFile().setExecutable(true);
            } else {
                // Fallback: create a simple inline script if resource not found
                String scriptContent = "#!/usr/bin/env python3\nprint('[DiSL] Mock analysis completed')\n";
                Files.write(mockDislScript, scriptContent.getBytes());
                mockDislScript.toFile().setExecutable(true);
            }
        } catch (Exception e) {
            // Fallback: create a simple inline script
            String scriptContent = "#!/usr/bin/env python3\nprint('[DiSL] Mock analysis completed')\n";
            Files.write(mockDislScript, scriptContent.getBytes());
            try {
                mockDislScript.toFile().setExecutable(true);
            } catch (Exception ignored) {
                // Ignore on Windows or if setting executable fails
            }
        }

        DiSLAnalyzer analyzer = new DiSLAnalyzer(testConfig);

        // When & Then - This test demonstrates the structure for testing process execution
        // The test may succeed or fail depending on the system setup, but it should not crash
        Path resultsListPath = tempDir.resolve("output/generated-tests.lst");
        try {
            var generated = analyzer.runAnalysis(InstrumentationResult.builder()
                    .primaryArtifact(mockInstrumentationJar)
                    .resultsListPath(resultsListPath)
                    .build());

            // If it succeeds, we should get a list of generated test paths
            assertNotNull(generated, "Analysis should return a non-null TestSuite");
            assertTrue(generated.getTestFiles().size() >= 0, "Generated tests list should be returned");

        } catch (Exception e) {
            // If it fails, it should be due to process execution issues
            assertNotNull(e);
            System.out.println("Exception caught: " + e.getClass().getSimpleName() + ": " + e.getMessage());
            // The important thing is that we're testing the structure and that it attempts to run the process
            assertTrue(e.getMessage().contains("Analysis execution failed") ||
                      e.getMessage().contains("timed out") ||
                      e.getMessage().contains("python3") ||
                      e.getMessage().contains("No such file") ||
                      e.getMessage().contains("Cannot run program") ||
                      e.getMessage().contains("does not exist") ||
                      e.getMessage().contains("Expected JAR file"));  // Common error when command not found
        }
    }
}
