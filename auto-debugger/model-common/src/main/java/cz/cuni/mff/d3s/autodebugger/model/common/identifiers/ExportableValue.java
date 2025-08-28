package cz.cuni.mff.d3s.autodebugger.model.common.identifiers;

import java.io.Serializable;

/**
 * Interface for values that can be exported during runtime analysis.
 * Extends Serializable to support persistence and provides internal ID
 * for efficient tracking during instrumentation and trace collection.
 */
public interface ExportableValue extends Serializable {
    /**
     * Returns the internal ID used for tracking this value during analysis.
     * @return The internal identifier
     */
    int getInternalId();
}
