package cz.cuni.mff.d3s.autodebugger.model.common.identifiers;

/**
 * Base interface for all identifier types in the auto-debugger system.
 * Provides a common contract for objects that can be uniquely identified by name.
 */
public interface Identifier {
    /**
     * Returns the name of this identifier.
     * @return The identifier name
     */
    String getName();
}
