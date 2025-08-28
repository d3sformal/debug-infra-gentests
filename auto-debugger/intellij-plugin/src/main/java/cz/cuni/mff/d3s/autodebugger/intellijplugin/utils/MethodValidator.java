package cz.cuni.mff.d3s.autodebugger.intellijplugin.utils;

import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.psi.JavaPsiFacade;
import com.intellij.psi.PsiClass;
import com.intellij.psi.PsiMethod;
import com.intellij.psi.search.GlobalSearchScope;
import cz.cuni.mff.d3s.autodebugger.model.java.parsing.MethodSignature;
import cz.cuni.mff.d3s.autodebugger.model.java.parsing.SignatureState;
import cz.cuni.mff.d3s.autodebugger.model.java.parsing.JavaMethodSignatureParser;
import cz.cuni.mff.d3s.autodebugger.intellijplugin.model.MethodValidationResult;

/**
 * Testable method validation logic separated from UI concerns
 */
public class MethodValidator {
    private final Project project;
    private static final Logger LOG = Logger.getInstance(MethodValidator.class);

    public MethodValidator(Project project) {
        this.project = project;
    }

    /**
     * Validates a method signature and returns validation result
     */
    public MethodValidationResult validateMethodSignature(String signature) {
        LOG.debug("MethodValidator: Validating signature: '" + signature + "'");

        if (signature == null || signature.trim().isEmpty()) {
            return new MethodValidationResult(false, "Empty signature", null);
        }

        try {
            MethodSignature parsed = JavaMethodSignatureParser.parseMethodSignature(signature.trim());
            if (parsed.getState() == SignatureState.INVALID) {
                return new MethodValidationResult(false, "Invalid signature format", null);
            }

            PsiMethod method = findMethodByParsedSignature(parsed);
            if (method != null) {
                return new MethodValidationResult(true, "Method found", method);
            } else {
                return new MethodValidationResult(false, "Method not found", null);
            }
        } catch (Exception e) {
            LOG.warn("Error validating method signature: " + signature, e);
            return new MethodValidationResult(false, "Validation error: " + e.getMessage(), null);
        }
    }

    /**
     * Finds a method using parsed signature components
     */
    public PsiMethod findMethodByParsedSignature(MethodSignature parsed) {
        if (project == null) {
            LOG.debug("Project is null, cannot find method");
            return null;
        }

        // Find the class using PSI
        JavaPsiFacade facade = JavaPsiFacade.getInstance(project);
        GlobalSearchScope scope = GlobalSearchScope.allScope(project);
        PsiClass psiClass = facade.findClass(parsed.getClassName(), scope);

        if (psiClass == null) {
            LOG.debug("Class not found: " + parsed.getClassName());
            return null;
        }

        LOG.debug("Found class: " + psiClass.getQualifiedName() + ", searching for method: " + parsed.getMethodName());

        // Find the method with matching name and parameters
        int methodCount = 0;
        for (PsiMethod method : psiClass.getAllMethods()) {
            methodCount++;
            if (method.getName().equals(parsed.getMethodName())) {
                String methodSignature = MethodUtils.buildMethodSignature(method);
                LOG.debug("Comparing signatures - Expected: '" + parsed.getOriginalSignature() + "', Found: '" + methodSignature + "'");
                if (methodSignature.equals(parsed.getOriginalSignature())) {
                    LOG.debug("Method match found!");
                    return method;
                }
            }
        }

        LOG.debug("No matching method found. Searched " + methodCount + " methods in class " + parsed.getClassName());
        return null;
    }
}
