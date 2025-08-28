package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import static cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.Constants.normalizeVariableNames;
import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

class JavaFieldTests {

  @Test
  void givenValidJavaField_whenGeneratingCode_thenCodeIsGenerated() {
    // given
    JavaField javaField = Constants.javaField;

    // when
    String code = javaField.emitCode();

    // then
    String expectedCode = "java.lang.String generatedVariable1 = di.getInstanceFieldValue(di.getThis(), Test.class, \"testField\", java.lang.String.class);";
    assertEquals(normalizeVariableNames(expectedCode), normalizeVariableNames(code));
  }
}
