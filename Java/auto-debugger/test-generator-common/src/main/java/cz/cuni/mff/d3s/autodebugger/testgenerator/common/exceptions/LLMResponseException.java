package cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions;

/**
 * Exception thrown when there are issues with the LLM response content.
 * This includes empty responses, malformed responses, or responses that don't contain expected content.
 */
public class LLMResponseException extends LLMTestGenerationException {

    private final String modelName;
    private final String promptSummary;
    private final String responseSummary;
    private final ResponseIssueType issueType;

    /**
     * Enum representing different types of response issues.
     */
    public enum ResponseIssueType {
        EMPTY_RESPONSE("Response was empty or null"),
        MALFORMED_RESPONSE("Response was malformed or unparseable"),
        MISSING_EXPECTED_CONTENT("Response did not contain expected content"),
        INVALID_CODE_FORMAT("Response did not contain valid code format"),
        UNEXPECTED_CONTENT("Response contained unexpected or invalid content");

        private final String description;

        ResponseIssueType(String description) {
            this.description = description;
        }

        public String getDescription() {
            return description;
        }
    }

    /**
     * Creates a new LLM response exception with the specified message.
     *
     * @param message the detail message
     */
    public LLMResponseException(String message) {
        super(message);
        this.modelName = null;
        this.promptSummary = null;
        this.responseSummary = null;
        this.issueType = null;
    }

    /**
     * Creates a new LLM response exception with the specified message and cause.
     *
     * @param message the detail message
     * @param cause the cause of this exception
     */
    public LLMResponseException(String message, Throwable cause) {
        super(message, cause);
        this.modelName = null;
        this.promptSummary = null;
        this.responseSummary = null;
        this.issueType = null;
    }

    /**
     * Creates a new LLM response exception with detailed response information.
     *
     * @param message the detail message
     * @param modelName the model that generated the response
     * @param promptSummary a summary of the prompt that was sent
     * @param responseSummary a summary of the response that was received
     * @param issueType the type of issue with the response
     */
    public LLMResponseException(String message, String modelName, String promptSummary, String responseSummary, ResponseIssueType issueType) {
        super(message);
        this.modelName = modelName;
        this.promptSummary = promptSummary;
        this.responseSummary = responseSummary;
        this.issueType = issueType;
    }

    /**
     * Creates a new LLM response exception with detailed response information and cause.
     *
     * @param message the detail message
     * @param modelName the model that generated the response
     * @param promptSummary a summary of the prompt that was sent
     * @param responseSummary a summary of the response that was received
     * @param issueType the type of issue with the response
     * @param cause the cause of this exception
     */
    public LLMResponseException(String message, String modelName, String promptSummary, String responseSummary, ResponseIssueType issueType, Throwable cause) {
        super(message, cause);
        this.modelName = modelName;
        this.promptSummary = promptSummary;
        this.responseSummary = responseSummary;
        this.issueType = issueType;
    }

    /**
     * Gets the model that generated the response.
     *
     * @return the model name, or null if not specified
     */
    public String getModelName() {
        return modelName;
    }

    /**
     * Gets a summary of the prompt that was sent.
     *
     * @return the prompt summary, or null if not specified
     */
    public String getPromptSummary() {
        return promptSummary;
    }

    /**
     * Gets a summary of the response that was received.
     *
     * @return the response summary, or null if not specified
     */
    public String getResponseSummary() {
        return responseSummary;
    }

    /**
     * Gets the type of issue with the response.
     *
     * @return the issue type, or null if not specified
     */
    public ResponseIssueType getIssueType() {
        return issueType;
    }

    @Override
    public String getMessage() {
        StringBuilder message = new StringBuilder(super.getMessage());
        
        if (issueType != null) {
            message.append(" [Issue: ").append(issueType.getDescription());
        }
        if (modelName != null) {
            message.append(", Model: ").append(modelName);
        }
        if (promptSummary != null) {
            message.append(", Prompt: ").append(promptSummary);
        }
        if (responseSummary != null) {
            message.append(", Response: ").append(responseSummary);
        }
        if (issueType != null || modelName != null || promptSummary != null || responseSummary != null) {
            message.append("]");
        }
        
        return message.toString();
    }

    /**
     * Creates a summary of a text for inclusion in exception messages.
     * Truncates long text and removes newlines for readability.
     *
     * @param text the text to summarize
     * @param maxLength the maximum length of the summary
     * @return a summary of the text
     */
    public static String createTextSummary(String text, int maxLength) {
        if (text == null) {
            return "null";
        }
        if (text.isEmpty()) {
            return "empty";
        }
        
        String summary = text.replaceAll("\\s+", " ").trim();
        if (summary.length() > maxLength) {
            summary = summary.substring(0, maxLength - 3) + "...";
        }
        
        return summary;
    }
}
