package cz.cuni.mff.d3s.autodebugger.intellijplugin.utils;

import com.intellij.psi.PsiClass;
import com.intellij.psi.PsiMethod;
import com.intellij.psi.PsiParameter;
import cz.cuni.mff.d3s.autodebugger.model.java.parsing.MethodSignature;
import cz.cuni.mff.d3s.autodebugger.model.java.parsing.SignatureState;
import cz.cuni.mff.d3s.autodebugger.model.java.parsing.JavaMethodSignatureParser;
import org.jetbrains.annotations.NotNull;

import java.util.List;

public class MethodUtils {

    public static String buildMethodSignature(PsiMethod method) {
        StringBuilder signature = new StringBuilder();

        // Add class name
        PsiClass containingClass = method.getContainingClass();
        if (containingClass != null) {
            signature.append(containingClass.getQualifiedName()).append(".");
        }

        // Add method name
        signature.append(method.getName());

        // Add parameters
        signature.append("(");
        PsiParameter[] parameters = method.getParameterList().getParameters();
        for (int i = 0; i < parameters.length; i++) {
            if (i > 0) signature.append(", ");
            signature.append(parameters[i].getType().getPresentableText());
        }
        signature.append(")");

        return signature.toString();
    }

    public static boolean isFullyQualifiedClassName(@NotNull String prefix) {
        // Check if this looks like a fully qualified class name (has dots and ends with a class-like name)
        if (!prefix.contains(".")) return false;

        int lastDot = prefix.lastIndexOf('.');
        String lastPart = prefix.substring(lastDot + 1);

        // If the last part starts with uppercase, it's likely a class name
        return !lastPart.isEmpty() && Character.isUpperCase(lastPart.charAt(0));
    }

    public static MethodSignature parseMethodSignature(String signature) {
        return JavaMethodSignatureParser.parseMethodSignature(signature);
    }

    public static SignatureState detectSignatureState(String sig) {
        return JavaMethodSignatureParser.detectSignatureState(sig);
    }

    public static String extractClassName(String beforeParams) {
        return JavaMethodSignatureParser.extractClassName(beforeParams);
    }

    public static String extractMethodName(String beforeParams) {
        return JavaMethodSignatureParser.extractMethodName(beforeParams);
    }

    public static String extractParamsBody(String sig) {
        return JavaMethodSignatureParser.extractParamsBody(sig);
    }

    public static List<String> parseParameterTypes(String paramsBody) {
        return JavaMethodSignatureParser.parseParameterTypes(paramsBody);
    }
}
