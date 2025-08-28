package cz.cuni.mff.d3s.autodebugger.intellijplugin.model;


import com.intellij.psi.PsiMethod;

/**
 * Data class for method validation results
 */
public class MethodValidationResult {
    public final boolean isValid;
    public final String message;
    public final PsiMethod method;

    public MethodValidationResult(boolean isValid, String message, PsiMethod method) {
        this.isValid = isValid;
        this.message = message;
        this.method = method;
    }
}
