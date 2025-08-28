package cz.cuni.mff.d3s.autodebugger.model.java.parsing;

/**
 * Represents the state of a method signature during parsing.
 * This helps determine how complete or valid a signature string is.
 */
public enum SignatureState {
    /**
     * The signature string is empty or null.
     */
    EMPTY,
    
    /**
     * The signature string is malformed or invalid.
     */
    INVALID,
    
    /**
     * The signature contains only package information.
     * Example: "com.example"
     */
    PACKAGE_ONLY,
    
    /**
     * The signature contains package and class information.
     * Example: "com.example.MyClass"
     */
    CLASS_ONLY,
    
    /**
     * The signature contains method name but incomplete parameter information.
     * Example: "com.example.MyClass.method" or "method"
     */
    METHOD_NAME_ONLY,
    
    /**
     * The signature is complete with class, method name, and parameters.
     * Example: "com.example.MyClass.method(String, int)"
     */
    FULL_METHOD
}
