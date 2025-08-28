package cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling;

import lombok.Getter;

/**
 * Abstract base class for instrumentation models.
 * Provides a common structure for representing instrumentation code
 * with a root metaclass that can be transformed into concrete code.
 */
@Getter
public abstract class InstrumentationModel {
  protected Metaclass rootClass;

  /**
   * Transforms the instrumentation model into concrete code.
   * Delegates to the root metaclass to generate the final code output.
   */
  public String transform() {
    return rootClass.emitCode();
  }
}
