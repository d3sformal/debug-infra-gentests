package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.enums;

public enum MarkerType {
    BODY("BodyMarker");

    public final String markerName;

    MarkerType(String markerName) {
        this.markerName = markerName;
    }
}
