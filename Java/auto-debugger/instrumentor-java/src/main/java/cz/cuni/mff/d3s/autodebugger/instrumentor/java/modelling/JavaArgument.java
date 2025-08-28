package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier;
import lombok.Getter;

@Getter
public class JavaArgument extends JavaValue {
    private final int argumentSlot;

    public JavaArgument(int argumentSlot, JavaValueIdentifier exportableValue) {
        super(exportableValue);
        this.argumentSlot = argumentSlot;
    }

    @Override
    public String emitCode() {
        append(exportedValueIdentifier.getType());
        append(" ");
        append(instrumentationVariableIdentifier.getName());
        append(" = ");
        append("di.getMethodArgumentValue(");
        append(Integer.toString(argumentSlot));
        append(", ");
        append(exportedValueIdentifier.getType());
        append(".class);");
        return getCode();
    }
}
