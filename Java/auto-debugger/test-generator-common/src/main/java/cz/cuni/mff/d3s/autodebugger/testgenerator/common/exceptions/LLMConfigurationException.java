package cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions;

/**
 * Exception thrown when there are configuration issues with the LLM client.
 * This includes invalid API keys, unsupported models, invalid parameters, etc.
 */
public class LLMConfigurationException extends LLMTestGenerationException {

    private final String configurationField;
    private final Object invalidValue;

    /**
     * Creates a new LLM configuration exception with the specified message.
     *
     * @param message the detail message
     */
    public LLMConfigurationException(String message) {
        super(message);
        this.configurationField = null;
        this.invalidValue = null;
    }

    /**
     * Creates a new LLM configuration exception with the specified message and cause.
     *
     * @param message the detail message
     * @param cause the cause of this exception
     */
    public LLMConfigurationException(String message, Throwable cause) {
        super(message, cause);
        this.configurationField = null;
        this.invalidValue = null;
    }

    /**
     * Creates a new LLM configuration exception with detailed field information.
     *
     * @param message the detail message
     * @param configurationField the name of the configuration field that caused the error
     * @param invalidValue the invalid value that was provided
     */
    public LLMConfigurationException(String message, String configurationField, Object invalidValue) {
        super(message);
        this.configurationField = configurationField;
        this.invalidValue = invalidValue;
    }

    /**
     * Creates a new LLM configuration exception with detailed field information and cause.
     *
     * @param message the detail message
     * @param configurationField the name of the configuration field that caused the error
     * @param invalidValue the invalid value that was provided
     * @param cause the cause of this exception
     */
    public LLMConfigurationException(String message, String configurationField, Object invalidValue, Throwable cause) {
        super(message, cause);
        this.configurationField = configurationField;
        this.invalidValue = invalidValue;
    }

    /**
     * Gets the name of the configuration field that caused the error.
     *
     * @return the configuration field name, or null if not specified
     */
    public String getConfigurationField() {
        return configurationField;
    }

    /**
     * Gets the invalid value that was provided.
     *
     * @return the invalid value, or null if not specified
     */
    public Object getInvalidValue() {
        return invalidValue;
    }

    @Override
    public String getMessage() {
        StringBuilder message = new StringBuilder(super.getMessage());
        
        if (configurationField != null) {
            message.append(" [Field: ").append(configurationField);
            if (invalidValue != null) {
                message.append(", Value: ").append(invalidValue);
            }
            message.append("]");
        }
        
        return message.toString();
    }
}
