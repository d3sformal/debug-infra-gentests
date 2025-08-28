package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.Metaclass;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;
import java.util.List;
import lombok.Getter;

@Getter
public class ShadowDiSLInstrumentationLogic extends DiSLInstrumentationLogic {

  public ShadowDiSLInstrumentationLogic(
          MethodIdentifier identifier, DiSLAnnotation annotation, List<JavaValue> exports) {
    super(identifier, annotation, exports);
  }

  @Override
  public String emitCode() {
    append(annotation.emitCode());
    append("\npublic static void ");
    append(identifier.getName());
    append("(DynamicContext di) {\n");
    for (Metaclass variable : exports) {
      append(variable.emitCode());
      append("\n");
    }
    append(
        "System.out.println(\"[Instrumentation process] PID: \" + ProcessHandle.current().pid());\n");
    // Start a new event for temporal tracing (no-op in naive mode)
    append("CollectorRE.startEvent();\n");
    for (JavaValue variable : exports) {
      append(variable.emitCollectorCode());
      append("\n");
    }
    append("}\n");
    return getCode();
  }
}
