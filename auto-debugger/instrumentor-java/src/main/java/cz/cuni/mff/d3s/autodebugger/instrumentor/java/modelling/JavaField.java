package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier;
import lombok.Getter;

@Getter
public class JavaField extends JavaValue {
    private final String name;
    private final String ownerType;

    public JavaField(String name, String ownerType, JavaValueIdentifier exportableValue) {
        super(exportableValue);
        this.name = name;
        // TODO: Use fully qualified name for ownerType
        this.ownerType = ownerType;
    }

    // TODO: Static fields
    @Override
    public String emitCode() {
    // dc.getThis (), TargetClass.class, "instName", String.class
        append(exportedValueIdentifier.getType());
        append(" ");
        append(instrumentationVariableIdentifier.getName());
        append(" = ");
        append("di.getInstanceFieldValue(di.getThis(), ");
        append(ownerType);
        append(".class, \"");
        append(name);
        append("\", ");
        append(exportedValueIdentifier.getType());
        append(".class);");
        return getCode();
    }
}
