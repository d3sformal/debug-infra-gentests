package cz.cuni.mff.d3s.autodebugger.model.java.factories;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaVariableIdentifier;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.VariableIdentifierParameters;
import lombok.Getter;

public class VariableIdentifierFactory extends IdentifierFactory {
    @Getter
    private static final VariableIdentifierFactory instance = new VariableIdentifierFactory();

    private VariableIdentifierFactory() {
    }

    public JavaVariableIdentifier createIdentifier(VariableIdentifierParameters parameters) {
        return new JavaVariableIdentifier(parameters);
    }

    public JavaVariableIdentifier generateIdentifier(VariableIdentifierParameters parameters) {
        parameters.variableName = "generatedVariable" + getNextId();
        return new JavaVariableIdentifier(parameters);
    }
}
