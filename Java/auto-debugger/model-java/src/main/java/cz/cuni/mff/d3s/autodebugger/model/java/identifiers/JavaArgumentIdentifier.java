package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.ExportableValue;
import cz.cuni.mff.d3s.autodebugger.model.java.enums.ValueType;
import cz.cuni.mff.d3s.autodebugger.model.java.factories.IdentifierFactory;
import lombok.Getter;
import lombok.NoArgsConstructor;

import java.io.Serializable;

@Getter
@NoArgsConstructor
public class JavaArgumentIdentifier extends JavaValueIdentifier implements ExportableValue, Serializable {
    private int internalId;
    private int argumentSlot;

    public JavaArgumentIdentifier(ArgumentIdentifierParameters parameters) {
        super(ValueType.ARGUMENT);
        this.internalId = IdentifierFactory.getNextId();
        this.argumentSlot = parameters.argumentSlot;
        this.type = parameters.variableType;
    }

    @Override
    public String getName() {
        return type + "_" + argumentSlot;
    }
}
