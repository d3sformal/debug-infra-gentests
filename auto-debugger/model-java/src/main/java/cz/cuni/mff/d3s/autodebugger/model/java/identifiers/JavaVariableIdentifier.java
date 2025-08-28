package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import cz.cuni.mff.d3s.autodebugger.model.java.enums.ValueType;
import lombok.Getter;
import lombok.NoArgsConstructor;

@Getter
@NoArgsConstructor
public class JavaVariableIdentifier extends JavaValueIdentifier {
    private String variableName;
    private String variableType;

    public JavaVariableIdentifier(VariableIdentifierParameters parameters) {
        super(ValueType.VARIABLE);
        this.variableName = parameters.variableName;
        this.variableType = parameters.variableType;
    }

    @Override
    public String getName() {
        return variableName;
    }

    @Override
    public int getInternalId() {
        throw new UnsupportedOperationException("Not implemented");
    }

    public String getType() {
        return variableType;
    }
}
