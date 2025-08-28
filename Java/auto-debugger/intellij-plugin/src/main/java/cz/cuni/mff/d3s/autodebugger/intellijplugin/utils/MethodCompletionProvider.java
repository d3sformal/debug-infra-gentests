package cz.cuni.mff.d3s.autodebugger.intellijplugin.utils;

import com.intellij.codeInsight.completion.CompletionResultSet;
import com.intellij.codeInsight.lookup.LookupElementBuilder;
import com.intellij.openapi.application.ApplicationManager;
import com.intellij.openapi.diagnostic.Logger;
import com.intellij.openapi.project.Project;
import com.intellij.psi.JavaPsiFacade;
import com.intellij.psi.PsiClass;
import com.intellij.psi.PsiMethod;
import com.intellij.psi.search.GlobalSearchScope;
import com.intellij.psi.search.PsiShortNamesCache;
import com.intellij.psi.search.searches.AllClassesSearch;
import com.intellij.util.Query;
import com.intellij.util.TextFieldCompletionProvider;
import org.jetbrains.annotations.NotNull;
import cz.cuni.mff.d3s.autodebugger.model.java.parsing.SignatureState;
import cz.cuni.mff.d3s.autodebugger.model.java.parsing.JavaMethodSignatureParser;

/**
 * PSI-based completion provider for method signatures with three-stage completion:
 * 1. Package name completion
 * 2. Class name completion (with package)
 * 3. Method name completion from all matching classes
 */
public class MethodCompletionProvider extends TextFieldCompletionProvider {
    private static final Logger LOG = Logger.getInstance(MethodValidator.class);

    private final Project project;

    public MethodCompletionProvider(Project project) {
        this.project = project;
    }

    @Override
    protected void addCompletionVariants(@NotNull String text, int offset, @NotNull String prefix,
                                         @NotNull CompletionResultSet result) {
        SignatureState state = JavaMethodSignatureParser.parseMethodSignature(prefix).getState();
        switch (state) {
            case EMPTY, PACKAGE_ONLY -> ApplicationManager.getApplication().runReadAction(() -> addPackageAndClassCompletions(prefix, result));
            case CLASS_ONLY -> {
                // Nested classes can be present, let's list them alongside methods
                ApplicationManager.getApplication().runReadAction(() -> addPackageAndClassCompletions(prefix, result));
                ApplicationManager.getApplication().runReadAction(() -> addClassNameCompletions(prefix, result));
                ApplicationManager.getApplication().runReadAction(() -> addMethodCompletions(prefix, result));
            }
            case METHOD_NAME_ONLY, FULL_METHOD ->  ApplicationManager.getApplication().runReadAction(() -> addMethodNameCompletions(prefix, result));
            case INVALID ->
                // Do nothing, invalid state
                    LOG.debug("Invalid signature state, not completing: " + prefix);
        }
    }

    // Case 1: Package name completion and Case 2: Class completion within package
    private void addPackageAndClassCompletions(@NotNull String prefix, @NotNull CompletionResultSet result) {
        GlobalSearchScope scope = GlobalSearchScope.projectScope(project);
        Query<PsiClass> query = AllClassesSearch.search(scope, project);

        query.forEach(psiClass -> {
            String qualifiedName = psiClass.getQualifiedName();
            if (qualifiedName != null && qualifiedName.toLowerCase().startsWith(prefix.toLowerCase())) {
                result.addElement(LookupElementBuilder.create(qualifiedName)
                        .withIcon(psiClass.getIcon(0))
                        .withTypeText("Class"));
            }
            return true;
        });
    }

    // Case 2: Class name completion (without a package) - suggest full class names
    private void addClassNameCompletions(@NotNull String prefix, @NotNull CompletionResultSet result) {
        PsiShortNamesCache cache = PsiShortNamesCache.getInstance(project);
        GlobalSearchScope scope = GlobalSearchScope.projectScope(project);

        String[] allClassNames = cache.getAllClassNames();
        for (String className : allClassNames) {
            if (className.toLowerCase().contains(prefix.toLowerCase())) {
                PsiClass[] classes = cache.getClassesByName(className, scope);
                for (PsiClass psiClass : classes) {
                    LOG.debug("Found class: " + psiClass.getQualifiedName());
                    String qualifiedName = psiClass.getQualifiedName();
                    if (qualifiedName != null) {
                        result.addElement(LookupElementBuilder.create(qualifiedName)
                                .withIcon(psiClass.getIcon(0))
                                .withTypeText("Class")
                                .withLookupString(className)); // Allow matching by short name
                    }
                }
            }
        }
    }

    // Case 3: Method name completion - suggest methods from all classes with matching method names
    private void addMethodNameCompletions(@NotNull String prefix, @NotNull CompletionResultSet result) {
        GlobalSearchScope scope = GlobalSearchScope.projectScope(project);
        Query<PsiClass> query = AllClassesSearch.search(scope, project);

        query.forEach(psiClass -> {
            PsiMethod[] methods = psiClass.getAllMethods();
            for (PsiMethod method : methods) {
                LOG.debug("Found method: " + method.getName());
                if (method.getName().toLowerCase().contains(prefix.toLowerCase())) {
                    String methodSignature = MethodUtils.buildMethodSignature(method);
                    result.addElement(LookupElementBuilder.create(methodSignature)
                            .withIcon(method.getIcon(0))
                            .withTypeText(method.getReturnType() != null ?
                                    method.getReturnType().getPresentableText() : "void")
                            .withTailText(" (" + (method.getContainingClass() != null ?
                                    method.getContainingClass().getName() : "Unknown") + ")")
                            .withLookupString(method.getName())); // Allow matching by method name
                }
            }
            return true;
        });
    }

    // Case 3: Method completion for fully qualified class names
    private void addMethodCompletions(@NotNull String prefix, @NotNull CompletionResultSet result) {
        int lastDot = prefix.lastIndexOf('.');
        String className = prefix.substring(0, lastDot);
        String methodPrefix = prefix.substring(lastDot + 1);

        // Find the class using PSI
        JavaPsiFacade facade = JavaPsiFacade.getInstance(project);
        GlobalSearchScope scope = GlobalSearchScope.projectScope(project);
        PsiClass psiClass = facade.findClass(className, scope);

        if (psiClass != null) {
            // Get all methods from the class
            PsiMethod[] methods = psiClass.getAllMethods();
            for (PsiMethod method : methods) {
                if (method.getName().toLowerCase().startsWith(methodPrefix.toLowerCase())) {
                    String methodSignature = MethodUtils.buildMethodSignature(method);
                    result.addElement(LookupElementBuilder.create(methodSignature)
                            .withIcon(method.getIcon(0))
                            .withTypeText(method.getReturnType() != null ?
                                    method.getReturnType().getPresentableText() : "void")
                            .withTailText(" (" + (method.getContainingClass() != null ?
                                    method.getContainingClass().getName() : "Unknown") + ")"));
                }
            }
        }
    }
}

