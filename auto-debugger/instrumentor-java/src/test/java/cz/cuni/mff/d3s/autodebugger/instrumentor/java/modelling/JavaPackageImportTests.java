package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.assertEquals;

class JavaPackageImportTests {

  @Test
  void givenValidJavaPackageImport_whenGeneratingCode_thenCodeIsGenerated() {
    // given
    JavaPackageImport javaPackageImport = Constants.javaPackageImport;

    // when
    String code = javaPackageImport.emitCode();

    // then
    assertEquals("import java.util.List;", code);
  }
}
