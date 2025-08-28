package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class DiSLMarkerTests {

    @Test
    void givenDiSLBodyMarker_whenGeneratingCode_thenCodeIsGenerated() {
        // given
        DiSLMarker dislMarker = Constants.dislMarker;

        // when
        String code = dislMarker.emitCode();

        // then
        assertEquals("marker = BodyMarker.class", code);
    }
}
