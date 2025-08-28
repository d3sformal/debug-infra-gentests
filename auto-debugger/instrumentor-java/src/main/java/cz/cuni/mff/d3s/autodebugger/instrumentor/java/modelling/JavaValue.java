package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.model.java.factories.VariableIdentifierFactory;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaValueIdentifier;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaVariableIdentifier;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.VariableIdentifierParameters;
import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.Metaclass;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.CollectorMethodRegistry;

/**
 * Abstract base class for Java values in instrumentation code generation.
 * Bridges between exportable value identifiers and instrumentation variables,
 * providing code emission capabilities for DiSL collector integration.
 */
public abstract class JavaValue extends Metaclass {
    protected final JavaValueIdentifier exportedValueIdentifier;
    protected final JavaVariableIdentifier instrumentationVariableIdentifier;

    public JavaValue(JavaValueIdentifier exportableValue) {
        this.exportedValueIdentifier = exportableValue;
        var parameters = VariableIdentifierParameters.builder()
                .variableType(exportableValue.getType())
                .build();
        this.instrumentationVariableIdentifier = VariableIdentifierFactory.getInstance()
                .generateIdentifier(parameters);
    }

    /**
     * Generates DiSL collector method call code for this value.
     * Combines the collector method name with value ID and variable name
     * to create a complete method invocation statement.
     */
    public String emitCollectorCode() {
        return "CollectorRE." + CollectorMethodRegistry.getCollectorMethodName(exportedValueIdentifier) + "(" + exportedValueIdentifier.getInternalId() + ", " + instrumentationVariableIdentifier.getName() + ");";
    }
}
