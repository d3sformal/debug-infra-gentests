package cz.cuni.mff.d3s.autodebugger.model.common.identifiers;

import java.io.Serializable;
import java.util.List;

import lombok.Getter;
import lombok.RequiredArgsConstructor;

/**
 * Abstract base class for method identifiers across different programming languages.
 * Provides common method identification properties including name, return type,
 * and parameter types while allowing language-specific extensions.
 */
@Getter
@RequiredArgsConstructor
public abstract class MethodIdentifier implements Identifier, Serializable {
    protected final String methodName;
    protected final String returnType;
    protected final List<String> parameterTypes;

    public String getName() {
        return methodName;
    }

    /**
     * Gets the fully qualified signature of the method.
     * This should include the full class name and method signature.
     *
     * @return The fully qualified method signature
     */
    public abstract String getFullyQualifiedSignature();

    /**
     * Gets the fully qualified class name that contains this method.
     *
     * @return The fully qualified class name
     */
    public abstract String getFullyQualifiedClassName();

    /**
     * Gets the package name of the class that contains this method.
     *
     * @return The package name, or empty string if no package
     */
    public abstract String getPackageName();

    /**
     * Gets the simple class name (without package) that contains this method.
     *
     * @return The simple class name
     */
    public abstract String getClassName();
}
