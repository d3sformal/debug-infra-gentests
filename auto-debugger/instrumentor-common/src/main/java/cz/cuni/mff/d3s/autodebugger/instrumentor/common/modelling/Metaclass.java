package cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling;

/**
 * Abstract base class for code generation metaclasses.
 * Provides a StringBuilder-based code emission framework with
 * append and retrieval methods for building code strings incrementally.
 */
public abstract class Metaclass {
  private final StringBuilder builder = new StringBuilder();

    /**
     * Generates the code representation of this metaclass.
     * Subclasses must implement this method to define their code emission logic.
     */
    public abstract String emitCode();

    /**
     * Appends code to the internal string builder.
     * Used by subclasses to incrementally build code strings.
     */
    protected void append(String code) {
        builder.append(code);
    }

    /**
     * Retrieves the accumulated code and clears the internal buffer.
     * Returns the complete code string and resets the builder for reuse.
     */
    protected String getCode() {
        var result = builder.toString();
        builder.setLength(0);
        return result;
    }
}
