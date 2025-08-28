package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.llm.LLMBasedTestGenerator;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerator;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Files;
import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test class for TestGeneratorFactory functionality.
 * Tests the creation of different test generators with updated LLM configuration.
 */
class TestGeneratorFactoryTest {

    @TempDir
    Path tempDir;

    @Test
    void givenAiAssistedStrategy_whenCreatingTestGenerator_thenReturnsLLMBasedGenerator() throws Exception {
        // given
        Path sourceDir = tempDir.resolve("src");
        Path outputDir = tempDir.resolve("output");
        Path appJar = tempDir.resolve("app.jar");

        Files.createDirectories(sourceDir);
        Files.createDirectories(outputDir);
        Files.createFile(appJar);

        JavaRunConfiguration runConfiguration = JavaRunConfiguration.builder()
                .sourceCodePath(sourceDir)
                .outputDirectory(outputDir)
                .applicationPath(appJar)
                .build();

        // when
        TestGenerator generator = TestGeneratorFactory.createTestGenerator(
                runConfiguration, "ai-assisted", "mock-api-key");

        // then
        assertNotNull(generator);
        assertInstanceOf(LLMBasedTestGenerator.class, generator);
        assertEquals("ai-assisted", generator.getGenerationTechnique());
    }

    @Test
    void givenInvalidStrategy_whenCreatingTestGenerator_thenThrowsException() throws Exception {
        // given
        Path sourceDir = tempDir.resolve("src");
        Path outputDir = tempDir.resolve("output");
        Path appJar = tempDir.resolve("app.jar");

        Files.createDirectories(sourceDir);
        Files.createDirectories(outputDir);
        Files.createFile(appJar);

        JavaRunConfiguration runConfiguration = JavaRunConfiguration.builder()
                .sourceCodePath(sourceDir)
                .outputDirectory(outputDir)
                .applicationPath(appJar)
                .build();

        // when/then
        IllegalArgumentException exception = assertThrows(IllegalArgumentException.class, () -> {
            TestGeneratorFactory.createTestGenerator(runConfiguration, "invalid-strategy");
        });

        assertTrue(exception.getMessage().contains("Unknown test generation strategy"));
    }

    @Test
    void givenAiAssistedStrategyWithMockApiKey_whenCreatingTestGenerator_thenConfiguresCorrectly() throws Exception {
        // given
        Path sourceDir = tempDir.resolve("src");
        Path outputDir = tempDir.resolve("output");
        Path appJar = tempDir.resolve("app.jar");

        Files.createDirectories(sourceDir);
        Files.createDirectories(outputDir);
        Files.createFile(appJar);

        JavaRunConfiguration runConfiguration = JavaRunConfiguration.builder()
                .sourceCodePath(sourceDir)
                .outputDirectory(outputDir)
                .applicationPath(appJar)
                .build();

        // when
        TestGenerator generator = TestGeneratorFactory.createTestGenerator(
                runConfiguration, "ai-assisted", "test-mock-key");

        // then
        assertNotNull(generator);
        assertInstanceOf(LLMBasedTestGenerator.class, generator);

        // Verify the generator was configured properly by checking it doesn't throw on basic operations
        assertDoesNotThrow(() -> {
            assertEquals("ai-assisted", generator.getGenerationTechnique());
        });
    }
}
