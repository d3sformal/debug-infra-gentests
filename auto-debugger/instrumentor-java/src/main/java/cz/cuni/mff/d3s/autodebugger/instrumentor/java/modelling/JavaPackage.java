package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.Metaclass;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.Identifier;
import lombok.AllArgsConstructor;
import lombok.Getter;

/**
 * Represents a Java package declaration in generated code.
 * Extends Metaclass to provide code emission capabilities for
 * generating proper package declaration statements.
 */
@Getter
@AllArgsConstructor
public class JavaPackage extends Metaclass {
    protected Identifier packageIdentifier;

    @Override
    public String emitCode() {
        append("package ");
        append(packageIdentifier.getName());
        append(";");
        return getCode();
    }
}
