package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.Metaclass;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.Identifier;
import lombok.Getter;

@Getter
public class JavaPackageImport extends Metaclass {
    protected JavaPackage importedPackage;

    public JavaPackageImport(Identifier identifier) {
        this.importedPackage = new JavaPackage(identifier);
    }

    @Override
    public String emitCode() {
        append("import ");
        append(importedPackage.getPackageIdentifier().getName());
        append(";");
        return getCode();
    }
}
