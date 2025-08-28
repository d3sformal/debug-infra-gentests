package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.ExportableValue;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;
import cz.cuni.mff.d3s.autodebugger.model.java.enums.ValueType;
import cz.cuni.mff.d3s.autodebugger.model.java.factories.IdentifierFactory;
import lombok.Getter;
import lombok.NoArgsConstructor;

import java.io.Serializable;

@Getter
@NoArgsConstructor
public class JavaReturnValueIdentifier extends JavaValueIdentifier implements ExportableValue, Serializable {
    private int internalId;
    private MethodIdentifier methodIdentifier;

    public JavaReturnValueIdentifier(ReturnValueIdentifierParameters parameters) {
        super(ValueType.RETURN_VALUE);
        this.methodIdentifier = parameters.methodIdentifier;
        this.internalId = IdentifierFactory.getNextId();
        this.type = parameters.variableType;
    }

    @Override
    public String getName() {
        return "return_" + methodIdentifier.getName();
    }

    @Override
    public int getInternalId() {
        return internalId;
    }

    @Override
    public String getType() {
        return type;
    }
}
