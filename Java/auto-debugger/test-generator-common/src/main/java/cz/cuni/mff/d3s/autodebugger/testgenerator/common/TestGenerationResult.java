package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

import lombok.Builder;
import lombok.Getter;
import lombok.Singular;

import java.nio.file.Path;
import java.time.Duration;
import java.time.LocalDateTime;
import java.util.List;
import java.util.Map;

/**
 * Result of test generation, including generated test files,
 * statistics, and any errors encountered during generation.
 */
@Builder
@Getter
public class TestGenerationResult {
    
    /**
     * Status of the test generation process.
     */
    private final TestGenerationStatus status;
    
    /**
     * List of generated test file paths.
     */
    @Singular
    private final List<Path> generatedTestFiles;
    
    /**
     * Number of test methods generated.
     */
    private final int testMethodCount;
    
    /**
     * Time taken for test generation.
     */
    private final Duration generationTime;
    
    /**
     * Timestamp when generation started.
     */
    private final LocalDateTime startTime;
    
    /**
     * Timestamp when generation completed.
     */
    private final LocalDateTime endTime;
    
    /**
     * Strategy used for test generation.
     */
    private final String generationStrategy;
    
    /**
     * Error message if generation failed.
     */
    private final String errorMessage;
    
    /**
     * Detailed error information.
     */
    private final String errorDetails;
    
    /**
     * Warnings encountered during generation.
     */
    @Singular
    private final List<String> warnings;
    
    /**
     * Statistics about the generated tests.
     */
    private final TestGenerationStatistics statistics;
    
    /**
     * Additional metadata about the generation process.
     */
    @Singular("metadataEntry")
    private final Map<String, Object> metadata;
    
    /**
     * Raw output from the generation process (for debugging).
     */
    private final String rawOutput;
    
    /**
     * Checks if test generation was successful.
     */
    public boolean isSuccessful() {
        return status == TestGenerationStatus.SUCCESS;
    }
    
    /**
     * Checks if any tests were generated.
     */
    public boolean hasGeneratedTests() {
        return !generatedTestFiles.isEmpty() && testMethodCount > 0;
    }
    
    /**
     * Gets the number of generated test files.
     */
    public int getGeneratedFileCount() {
        return generatedTestFiles.size();
    }
}
