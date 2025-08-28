package cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions;

/**
 * Base exception for all LLM test generation related errors.
 * This is the root exception class for the LLM test generation workflow.
 */
public class LLMTestGenerationException extends Exception {

    /**
     * Creates a new LLM test generation exception with the specified message.
     *
     * @param message the detail message
     */
    public LLMTestGenerationException(String message) {
        super(message);
    }

    /**
     * Creates a new LLM test generation exception with the specified message and cause.
     *
     * @param message the detail message
     * @param cause the cause of this exception
     */
    public LLMTestGenerationException(String message, Throwable cause) {
        super(message, cause);
    }

    /**
     * Creates a new LLM test generation exception with the specified cause.
     *
     * @param cause the cause of this exception
     */
    public LLMTestGenerationException(Throwable cause) {
        super(cause);
    }
}
