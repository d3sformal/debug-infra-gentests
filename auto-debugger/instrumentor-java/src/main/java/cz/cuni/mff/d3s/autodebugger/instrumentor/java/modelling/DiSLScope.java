package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.Metaclass;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaMethodIdentifier;
import lombok.AllArgsConstructor;
import lombok.Getter;

@Getter
@AllArgsConstructor
public class DiSLScope extends Metaclass {
    private JavaMethodIdentifier methodIdentifier;

    @Override
    public String emitCode() {
        append("scope = \"");
        // Use fully qualified class name for DiSL scope
        var classIdentifier = methodIdentifier.getOwnerClassIdentifier();
        append(classIdentifier.getPackageIdentifier().getPackageName());
        append(".");
        append(classIdentifier.getName());
        append(".");
        append(methodIdentifier.getName());

        // Include parameter types for precise method matching
        if (!methodIdentifier.getParameterTypes().isEmpty()) {
            append("(");
            for (int i = 0; i < methodIdentifier.getParameterTypes().size(); i++) {
                if (i > 0) {
                    append(",");
                }
                append(methodIdentifier.getParameterTypes().get(i));
            }
            append(")");
        }

        append("\"");
        return getCode();
    }
}
