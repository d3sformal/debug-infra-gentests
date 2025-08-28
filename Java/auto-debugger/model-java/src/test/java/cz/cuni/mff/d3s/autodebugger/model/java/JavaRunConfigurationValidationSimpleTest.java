package cz.cuni.mff.d3s.autodebugger.model.java;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Simple test to verify JavaRunConfiguration validation compiles correctly.
 */
class JavaRunConfigurationValidationSimpleTest {

    @TempDir
    Path tempDir;

    @Test
    void givenValidConfiguration_whenValidating_thenMethodExists() throws IOException {
        // Create minimal valid files
        Path jarFile = tempDir.resolve("test.jar");
        Files.createFile(jarFile);
        
        Path sourceDir = tempDir.resolve("src");
        Files.createDirectory(sourceDir);
        
        Path dislHome = tempDir.resolve("disl");
        Files.createDirectory(dislHome);
        Files.createDirectories(dislHome.resolve("bin"));
        Files.createFile(dislHome.resolve("bin/disl.py"));
        Files.createDirectories(dislHome.resolve("output/lib"));
        
        // Test that we can create a configuration and call validate
        JavaRunConfiguration config = JavaRunConfiguration.builder()
            .applicationPath(jarFile)
            .sourceCodePath(sourceDir)
            .dislHomePath(dislHome)
            .build();
        
        // This should throw an exception because targetMethod is null
        assertThrows(IllegalStateException.class, config::validate);
    }
}
