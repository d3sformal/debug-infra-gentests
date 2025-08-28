package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import org.junit.jupiter.api.Test;

import static cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.Constants.normalizeVariableNames;
import static org.junit.jupiter.api.Assertions.assertEquals;

class JavaArgumentTests {

    @Test
    void givenValidJavaArgument_whenGeneratingCode_thenCodeIsGenerated() {
        // given
        JavaArgument javaArgument = Constants.javaArgument;

        // when
        String code = javaArgument.emitCode();

        // then
        String expectedCode = "java.lang.String generatedVariable0 = di.getMethodArgumentValue(0, java.lang.String.class);";
        assertEquals(normalizeVariableNames(expectedCode), normalizeVariableNames(code));
    }
}
