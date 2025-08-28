package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.enums;

public enum ActivationTime {
    BEFORE("Before"),
    AFTER("After"),
    AFTER_RETURNING("AfterReturning"),
    AFTER_THROWING("AfterThrowing");

    public final String annotationCode;

    ActivationTime(String annotationCode) {
        this.annotationCode = annotationCode;
    }
}
