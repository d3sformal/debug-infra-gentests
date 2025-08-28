package cz.cuni.mff.d3s.autodebugger.model.java.parsing;

import lombok.AllArgsConstructor;
import lombok.Getter;

import java.util.List;

/**
 * Holds the parsed details of a method signature.
 * This is a language-agnostic representation that can be used across different modules.
 */
@Getter
@AllArgsConstructor
public class MethodSignature {
    private final String className;
    private final String methodName;
    private final List<String> parameterTypes;
    private final String originalSignature;
    private final SignatureState state;
    
    /**
     * Extracts the package name from the class name.
     * 
     * @return The package name, or empty string if no package
     */
    public String getPackageName() {
        if (className == null || className.isEmpty()) {
            return "";
        }
        
        int lastDot = className.lastIndexOf('.');
        if (lastDot == -1) {
            return ""; // Default package
        }
        
        return className.substring(0, lastDot);
    }
    
    /**
     * Extracts the simple class name (without package) from the full class name.
     * 
     * @return The simple class name
     */
    public String getSimpleClassName() {
        if (className == null || className.isEmpty()) {
            return "";
        }
        
        int lastDot = className.lastIndexOf('.');
        if (lastDot == -1) {
            return className; // No package, return as-is
        }
        
        return className.substring(lastDot + 1);
    }
    
    /**
     * Checks if this signature represents a complete method signature.
     * 
     * @return true if this is a complete method signature with all required parts
     */
    public boolean isComplete() {
        return state == SignatureState.FULL_METHOD && 
               methodName != null && !methodName.isEmpty();
    }
    
    /**
     * Checks if this signature is in the default package.
     *
     * @return true if the method is in the default package
     */
    public boolean isInDefaultPackage() {
        return getPackageName().isEmpty();
    }

    /**
     * Returns the full class name (including package).
     * This is an alias for getClassName() for compatibility.
     *
     * @return The full class name
     */
    public String getFullClassName() {
        return className;
    }
}
