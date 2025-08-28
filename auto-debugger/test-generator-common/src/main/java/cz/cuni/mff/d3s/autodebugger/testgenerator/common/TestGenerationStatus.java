package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

/**
 * Status of the test generation process.
 */
public enum TestGenerationStatus {
    /**
     * Test generation completed successfully.
     */
    SUCCESS,
    
    /**
     * Test generation failed due to an error.
     */
    FAILED,
    
    /**
     * Test generation completed with warnings.
     */
    SUCCESS_WITH_WARNINGS,
    
    /**
     * Test generation was cancelled or aborted.
     */
    CANCELLED,
    
    /**
     * Test generation is currently in progress.
     */
    IN_PROGRESS,
    
    /**
     * Test generation has not started yet.
     */
    NOT_STARTED
}
