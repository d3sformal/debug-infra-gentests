package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;

public class ReturnValueIdentifierParameters extends ExportableValueIdentifierParameters {
    public final MethodIdentifier methodIdentifier;

    public ReturnValueIdentifierParameters(MethodIdentifier methodIdentifier) {
        super(null, methodIdentifier.getReturnType());
        this.methodIdentifier = methodIdentifier;
    }
}
