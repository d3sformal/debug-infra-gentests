package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class JavaPackageTests {

    @Test
    void givenValidJavaPackage_whenGeneratingCode_thenCodeIsGenerated() {
        // given
        JavaPackage javaPackage = Constants.javaPackage;

        // when
        String code = javaPackage.emitCode();

        // then
        assertEquals("package cz.cuni.mff.d3s.test;", code);
    }
}
