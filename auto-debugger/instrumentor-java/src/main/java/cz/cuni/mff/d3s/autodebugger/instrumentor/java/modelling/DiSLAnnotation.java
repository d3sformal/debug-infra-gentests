package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.Metaclass;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.enums.ActivationTime;
import lombok.AllArgsConstructor;
import lombok.Getter;

/**
 * Represents a DiSL annotation for instrumentation code generation.
 * Combines activation time, marker, and target method scope to generate
 * proper DiSL annotation syntax for instrumentation methods.
 */
@Getter
@AllArgsConstructor
public class DiSLAnnotation extends Metaclass {
    private ActivationTime activationTime;
    private DiSLMarker marker;
    private DiSLScope targetMethod;

    @Override
    public String emitCode() {
        append("@");
        append(activationTime.annotationCode);
        append("(");
        append(marker.emitCode());
        append(", ");
        append(targetMethod.emitCode());
        append(")");
        return getCode();
    }
}
