package cz.cuni.mff.d3s.autodebugger.instrumentor.java.factories;

import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.JavaReturnValue;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaReturnValueIdentifier;
import lombok.Getter;

public class ReturnValueExportFactory extends ExportableValueFactory {
    @Getter
    private static final ReturnValueExportFactory instance = new ReturnValueExportFactory();

    private ReturnValueExportFactory() {
    }

    public JavaReturnValue createExportable(JavaReturnValueIdentifier identifier) {
        return new JavaReturnValue(identifier);
    }
}
