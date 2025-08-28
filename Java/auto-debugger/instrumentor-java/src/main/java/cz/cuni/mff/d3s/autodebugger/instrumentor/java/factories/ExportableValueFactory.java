package cz.cuni.mff.d3s.autodebugger.instrumentor.java.factories;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.*;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.JavaValue;

public abstract class ExportableValueFactory {

  public static JavaValue createFrom(JavaValueIdentifier identifier) {
    return switch (identifier.getValueType()) {
      case ARGUMENT ->
          ArgumentExportFactory.getInstance().createExportable((JavaArgumentIdentifier) identifier);
      case FIELD ->
          FieldExportFactory.getInstance().createExportable((JavaFieldIdentifier) identifier);
      case VARIABLE ->
          VariableExportFactory.getInstance().createExportable((JavaVariableIdentifier) identifier);
      case RETURN_VALUE ->
          ReturnValueExportFactory.getInstance().createExportable((JavaReturnValueIdentifier) identifier);
      default ->
          throw new IllegalArgumentException(
              "Unknown exportable type: " + identifier.getValueType());
    };
  }
}
