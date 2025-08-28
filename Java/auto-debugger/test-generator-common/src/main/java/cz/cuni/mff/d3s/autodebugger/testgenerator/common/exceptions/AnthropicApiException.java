package cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions;

/**
 * Exception thrown when there are issues communicating with the Anthropic Claude API.
 * This includes network errors, API rate limits, authentication failures, and API response errors.
 */
public class AnthropicApiException extends LLMTestGenerationException {

    private final String modelName;
    private final Integer httpStatusCode;
    private final String apiErrorCode;
    private final String requestId;

    /**
     * Creates a new Anthropic API exception with the specified message.
     *
     * @param message the detail message
     */
    public AnthropicApiException(String message) {
        super(message);
        this.modelName = null;
        this.httpStatusCode = null;
        this.apiErrorCode = null;
        this.requestId = null;
    }

    /**
     * Creates a new Anthropic API exception with the specified message and cause.
     *
     * @param message the detail message
     * @param cause the cause of this exception
     */
    public AnthropicApiException(String message, Throwable cause) {
        super(message, cause);
        this.modelName = null;
        this.httpStatusCode = null;
        this.apiErrorCode = null;
        this.requestId = null;
    }

    /**
     * Creates a new Anthropic API exception with detailed API information.
     *
     * @param message the detail message
     * @param modelName the Claude model that was being used
     * @param httpStatusCode the HTTP status code from the API response
     * @param apiErrorCode the specific API error code from Anthropic
     * @param requestId the request ID for debugging purposes
     */
    public AnthropicApiException(String message, String modelName, Integer httpStatusCode, String apiErrorCode, String requestId) {
        super(message);
        this.modelName = modelName;
        this.httpStatusCode = httpStatusCode;
        this.apiErrorCode = apiErrorCode;
        this.requestId = requestId;
    }

    /**
     * Creates a new Anthropic API exception with detailed API information and cause.
     *
     * @param message the detail message
     * @param modelName the Claude model that was being used
     * @param httpStatusCode the HTTP status code from the API response
     * @param apiErrorCode the specific API error code from Anthropic
     * @param requestId the request ID for debugging purposes
     * @param cause the cause of this exception
     */
    public AnthropicApiException(String message, String modelName, Integer httpStatusCode, String apiErrorCode, String requestId, Throwable cause) {
        super(message, cause);
        this.modelName = modelName;
        this.httpStatusCode = httpStatusCode;
        this.apiErrorCode = apiErrorCode;
        this.requestId = requestId;
    }

    /**
     * Gets the Claude model that was being used when the error occurred.
     *
     * @return the model name, or null if not specified
     */
    public String getModelName() {
        return modelName;
    }

    /**
     * Gets the HTTP status code from the API response.
     *
     * @return the HTTP status code, or null if not available
     */
    public Integer getHttpStatusCode() {
        return httpStatusCode;
    }

    /**
     * Gets the specific API error code from Anthropic.
     *
     * @return the API error code, or null if not available
     */
    public String getApiErrorCode() {
        return apiErrorCode;
    }

    /**
     * Gets the request ID for debugging purposes.
     *
     * @return the request ID, or null if not available
     */
    public String getRequestId() {
        return requestId;
    }

    @Override
    public String getMessage() {
        StringBuilder message = new StringBuilder(super.getMessage());
        
        if (modelName != null) {
            message.append(" [Model: ").append(modelName);
        }
        if (httpStatusCode != null) {
            message.append(", HTTP Status: ").append(httpStatusCode);
        }
        if (apiErrorCode != null) {
            message.append(", API Error: ").append(apiErrorCode);
        }
        if (requestId != null) {
            message.append(", Request ID: ").append(requestId);
        }
        if (modelName != null || httpStatusCode != null || apiErrorCode != null || requestId != null) {
            message.append("]");
        }
        
        return message.toString();
    }

    /**
     * Checks if this exception represents a rate limit error.
     *
     * @return true if this is a rate limit error
     */
    public boolean isRateLimitError() {
        return httpStatusCode != null && httpStatusCode == 429;
    }

    /**
     * Checks if this exception represents an authentication error.
     *
     * @return true if this is an authentication error
     */
    public boolean isAuthenticationError() {
        return httpStatusCode != null && httpStatusCode == 401;
    }

    /**
     * Checks if this exception represents a quota exceeded error.
     *
     * @return true if this is a quota exceeded error
     */
    public boolean isQuotaExceededError() {
        return httpStatusCode != null && httpStatusCode == 402;
    }
}
