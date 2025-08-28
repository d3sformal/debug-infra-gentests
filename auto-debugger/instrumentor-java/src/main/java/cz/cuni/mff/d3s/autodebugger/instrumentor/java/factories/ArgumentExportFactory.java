package cz.cuni.mff.d3s.autodebugger.instrumentor.java.factories;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaArgumentIdentifier;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.JavaArgument;
import lombok.Getter;

public class ArgumentExportFactory extends ExportableValueFactory {
  @Getter private static final ArgumentExportFactory instance = new ArgumentExportFactory();

  private ArgumentExportFactory() {}

  public JavaArgument createExportable(JavaArgumentIdentifier identifier) {
    return new JavaArgument(identifier.getArgumentSlot(), identifier);
  }
}
