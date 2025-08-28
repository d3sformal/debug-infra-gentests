package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.Metaclass;
import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.MethodIdentifier;
import lombok.Getter;

import java.util.List;

@Getter
public class SerializationDiSLInstrumentationLogic extends DiSLInstrumentationLogic {

    public SerializationDiSLInstrumentationLogic(MethodIdentifier identifier, DiSLAnnotation annotation, List<JavaValue> exports) {
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
        append("FileOutputStream fileOut;ObjectOutputStream out;\n");
        append("try {\n");
        // TODO: Not going to work for multiple values
        for (JavaValue variable : exports) {
            append("fileOut = new FileOutputStream(\"");
            append(variable.instrumentationVariableIdentifier.getName());
            append(".ser\");\n");
            append("out = new ObjectOutputStream(fileOut);\n");
            append("out.writeObject(");
            append(variable.instrumentationVariableIdentifier.getName());
            append(");\n");
            append("out.close();\n");
            append("fileOut.close();\n");
            append("System.out.println(\"");
            append(variable.instrumentationVariableIdentifier.getName());
            append(".ser\");\n");
        }
        append("} catch (IOException e) {\n");
        append("e.printStackTrace();\n");
        append("}\n");
        append("}\n");
        return getCode();
    }
}
