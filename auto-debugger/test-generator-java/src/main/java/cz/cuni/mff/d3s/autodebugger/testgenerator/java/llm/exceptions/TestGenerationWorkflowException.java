package cz.cuni.mff.d3s.autodebugger.testgenerator.java.llm.exceptions;

import cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions.LLMTestGenerationException;

/**
 * Exception thrown when there are issues in the test generation workflow.
 * This includes problems with source code processing, test file writing, code validation, etc.
 * This is a runtime exception since workflow errors are typically unrecoverable.
 */
public class TestGenerationWorkflowException extends RuntimeException {

    private final String workflowStage;
    private final String sourceFilePath;
    private final String targetMethod;

    /**
     * Enum representing different stages of the test generation workflow.
     */
    public enum WorkflowStage {
        SOURCE_CODE_READING("Reading source code"),
        PROMPT_BUILDING("Building LLM prompt"),
        CODE_GENERATION("Generating test code"),
        CODE_VALIDATION("Validating generated code"),
        CODE_REFINEMENT("Refining generated code"),
        TEST_FILE_WRITING("Writing test file"),
        POST_PROCESSING("Post-processing generated tests");

        private final String description;

        WorkflowStage(String description) {
            this.description = description;
        }

        public String getDescription() {
            return description;
        }
    }

    /**
     * Creates a new test generation workflow exception with the specified message.
     *
     * @param message the detail message
     */
    public TestGenerationWorkflowException(String message) {
        super(message);
        this.workflowStage = null;
        this.sourceFilePath = null;
        this.targetMethod = null;
    }

    /**
     * Creates a new test generation workflow exception with the specified message and cause.
     *
     * @param message the detail message
     * @param cause the cause of this exception
     */
    public TestGenerationWorkflowException(String message, Throwable cause) {
        super(message, cause);
        this.workflowStage = null;
        this.sourceFilePath = null;
        this.targetMethod = null;
    }

    /**
     * Creates a new test generation workflow exception with workflow context.
     *
     * @param message the detail message
     * @param workflowStage the stage of the workflow where the error occurred
     * @param sourceFilePath the path to the source file being processed
     * @param targetMethod the target method being tested
     */
    public TestGenerationWorkflowException(String message, String workflowStage, String sourceFilePath, String targetMethod) {
        super(message);
        this.workflowStage = workflowStage;
        this.sourceFilePath = sourceFilePath;
        this.targetMethod = targetMethod;
    }

    /**
     * Creates a new test generation workflow exception with workflow context and cause.
     *
     * @param message the detail message
     * @param workflowStage the stage of the workflow where the error occurred
     * @param sourceFilePath the path to the source file being processed
     * @param targetMethod the target method being tested
     * @param cause the cause of this exception
     */
    public TestGenerationWorkflowException(String message, String workflowStage, String sourceFilePath, String targetMethod, Throwable cause) {
        super(message, cause);
        this.workflowStage = workflowStage;
        this.sourceFilePath = sourceFilePath;
        this.targetMethod = targetMethod;
    }

    /**
     * Gets the stage of the workflow where the error occurred.
     *
     * @return the workflow stage, or null if not specified
     */
    public String getWorkflowStage() {
        return workflowStage;
    }

    /**
     * Gets the path to the source file being processed.
     *
     * @return the source file path, or null if not specified
     */
    public String getSourceFilePath() {
        return sourceFilePath;
    }

    /**
     * Gets the target method being tested.
     *
     * @return the target method, or null if not specified
     */
    public String getTargetMethod() {
        return targetMethod;
    }

    @Override
    public String getMessage() {
        StringBuilder message = new StringBuilder(super.getMessage());
        
        if (workflowStage != null) {
            message.append(" [Stage: ").append(workflowStage);
        }
        if (sourceFilePath != null) {
            message.append(", Source: ").append(sourceFilePath);
        }
        if (targetMethod != null) {
            message.append(", Method: ").append(targetMethod);
        }
        if (workflowStage != null || sourceFilePath != null || targetMethod != null) {
            message.append("]");
        }
        
        return message.toString();
    }
}
