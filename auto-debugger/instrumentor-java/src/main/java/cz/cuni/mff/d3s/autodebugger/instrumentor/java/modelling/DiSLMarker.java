package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.Metaclass;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.enums.MarkerType;
import lombok.AllArgsConstructor;
import lombok.Getter;

@Getter
@AllArgsConstructor
public class DiSLMarker extends Metaclass {
    private MarkerType markerType;

    @Override
    public String emitCode() {
        append("marker = ");
        append(markerType.markerName);
        append(".class");
        return getCode();
    }
}
