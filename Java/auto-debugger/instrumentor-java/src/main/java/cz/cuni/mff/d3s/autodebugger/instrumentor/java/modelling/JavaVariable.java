package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier;
import lombok.Getter;

@Getter
public class JavaVariable extends JavaValue {
    private final int frameSlot;

    public JavaVariable(int frameSlot, JavaValueIdentifier exportableValue) {
        super(exportableValue);
        this.frameSlot = frameSlot;
    }

    @Override
    public String emitCode() {
        append(exportedValueIdentifier.getType());
        append(" ");
        append(instrumentationVariableIdentifier.getName());
        append(" = ");
        append("di.getLocalVariableValue(");
        append(Integer.toString(frameSlot));
        append(", ");
        append(exportedValueIdentifier.getType());
        append(".class);");
        return getCode();
    }
}
