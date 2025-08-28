package cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions;

/**
 * Exception thrown when attempting to use an LLM client that has not been properly configured.
 * This is a runtime exception since it represents a programming error.
 */
public class LLMClientNotConfiguredException extends RuntimeException {

    private final String clientType;

    /**
     * Creates a new LLM client not configured exception with a default message.
     */
    public LLMClientNotConfiguredException() {
        super("LLM client must be configured before use. Call configure() method first.");
        this.clientType = null;
    }

    /**
     * Creates a new LLM client not configured exception with the specified message.
     *
     * @param message the detail message
     */
    public LLMClientNotConfiguredException(String message) {
        super(message);
        this.clientType = null;
    }

    /**
     * Creates a new LLM client not configured exception with client type information.
     *
     * @param message the detail message
     * @param clientType the type of client that was not configured
     */
    public LLMClientNotConfiguredException(String message, String clientType) {
        super(message);
        this.clientType = clientType;
    }

    /**
     * Gets the type of client that was not configured.
     *
     * @return the client type, or null if not specified
     */
    public String getClientType() {
        return clientType;
    }

    @Override
    public String getMessage() {
        StringBuilder message = new StringBuilder(super.getMessage());
        
        if (clientType != null) {
            message.append(" [Client Type: ").append(clientType).append("]");
        }
        
        return message.toString();
    }
}
