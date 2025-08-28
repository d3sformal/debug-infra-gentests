package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.ExportableValue;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.Identifier;
import cz.cuni.mff.d3s.autodebugger.model.java.enums.ValueType;
import lombok.Getter;
import lombok.NoArgsConstructor;

import java.io.Serializable;

/**
 * Abstract base class for Java value identifiers in instrumentation.
 * Combines Identifier and ExportableValue interfaces to represent
 * Java values that can be tracked during runtime analysis.
 */
@Getter
@NoArgsConstructor
public abstract class JavaValueIdentifier implements Identifier, ExportableValue, Serializable {
    private ValueType valueType;
    protected String type;

    protected JavaValueIdentifier(ValueType valueType) {
        this.valueType = valueType;
    }
}
