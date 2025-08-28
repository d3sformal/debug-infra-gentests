package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.junit.jupiter.api.Test;

class DiSLScopeTests {

  @Test
  void givenValidDiSLScope_whenGeneratingCode_thenCodeIsGenerated() {
    // given
    DiSLScope dislScope = Constants.dislScope;

    // when
    String code = dislScope.emitCode();

    // then
    assertEquals(
        String.format("scope = \"%s.%s\"", Constants.targetClassName, Constants.targetMethodName),
        code);
  }
}
