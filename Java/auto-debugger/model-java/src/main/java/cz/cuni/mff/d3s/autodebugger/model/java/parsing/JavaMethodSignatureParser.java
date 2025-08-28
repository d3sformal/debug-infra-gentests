package cz.cuni.mff.d3s.autodebugger.model.java.parsing;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;

/**
 * Parser for Java method signatures.
 * This class is based on the existing implementation from the IntelliJ plugin
 * but moved to a more appropriate location for reuse across modules.
 */
public class JavaMethodSignatureParser {

    /**
     * Parses a method signature string into a MethodSignature object.
     * 
     * @param signature The method signature string to parse
     * @return Parsed MethodSignature object
     */
    public static MethodSignature parseMethodSignature(String signature) {
        SignatureState state = detectSignatureState(signature);
        if (state != SignatureState.FULL_METHOD) {
            // Return a minimal object for non-full signatures
            String className = "";
            String methodName = "";

            if (state == SignatureState.CLASS_ONLY || state == SignatureState.PACKAGE_ONLY) {
                className = signature;
            } else if (state == SignatureState.METHOD_NAME_ONLY) {
                // For method name only, we need to extract the method name part
                // Handle cases like "Class.method" or just "method"
                if (signature.contains(".") && !signature.contains("(")) {
                    // This is like "Class.method" - extract the method name
                    int lastDot = signature.lastIndexOf('.');
                    methodName = signature.substring(lastDot + 1);
                } else {
                    // This is just "method" or "method(" (malformed)
                    int parenIndex = signature.indexOf('(');
                    methodName = parenIndex >= 0 ? signature.substring(0, parenIndex) : signature;
                }
            } else if (state == SignatureState.INVALID) {
                // For invalid signatures, return empty strings
                className = "";
                methodName = "";
            }

            return new MethodSignature(
                    className,
                    methodName,
                    Collections.emptyList(),
                    signature,
                    state
            );
        }

        String sig = signature.trim();
        String beforeParams = sig.substring(0, sig.indexOf('(')).trim();
        String paramsBody = extractParamsBody(sig);

        String className = extractClassName(beforeParams);
        String methodName = extractMethodName(beforeParams);
        List<String> paramTypes = parseParameterTypes(paramsBody);

        return new MethodSignature(
                className,
                methodName,
                paramTypes,
                signature,
                SignatureState.FULL_METHOD
        );
    }

    /**
     * Utility to determine the SignatureState of a given signature string.
     */
    public static SignatureState detectSignatureState(String sig) {
        if (sig == null || sig.trim().isEmpty()) {
            return SignatureState.EMPTY;
        }
        String trimmed = sig.trim();
        int open = trimmed.indexOf('(');
        int close = trimmed.lastIndexOf(')');

        // Check if we have parentheses
        if (open >= 0) {
            // If we have opening parenthesis but no closing, or malformed parentheses
            if (close <= open) {
                // This is a malformed signature, but we can still determine the state
                // based on what comes before the opening parenthesis
                String beforeParen = trimmed.substring(0, open);
                if (beforeParen.contains(".")) {
                    // Has qualified name, so it's a method name
                    return SignatureState.METHOD_NAME_ONLY;
                } else {
                    // No qualified name, just method name
                    return SignatureState.METHOD_NAME_ONLY;
                }
            }

            // Check if closing parenthesis is at the end
            if (close != trimmed.length() - 1) {
                return SignatureState.INVALID;
            }

            // We have well-formed parentheses
            String beforeParams = trimmed.substring(0, open);
            if (!beforeParams.contains(".")) {
                // Method name without qualified class name - this should be METHOD_NAME_ONLY
                return SignatureState.METHOD_NAME_ONLY;
            }
            return SignatureState.FULL_METHOD;
        }

        // No parentheses - determine if it's package, class, or method name
        if (!trimmed.contains(".")) {
            // No dots - could be a simple class name or method name
            // If it starts with uppercase, it's likely a class name
            if (!trimmed.isEmpty() && Character.isUpperCase(trimmed.charAt(0))) {
                return SignatureState.CLASS_ONLY;
            }
            return SignatureState.METHOD_NAME_ONLY;
        }

        // Has dots - need to determine if it's package, class, or method
        return determineStateWithDots(trimmed);
    }

    /**
     * Helper method to determine state when the signature contains dots but no parentheses
     */
    private static SignatureState determineStateWithDots(String trimmed) {
        // Handle trailing dot case (e.g., "com.pkg.Class.")
        if (trimmed.endsWith(".")) {
            String withoutTrailingDot = trimmed.substring(0, trimmed.length() - 1);
            if (withoutTrailingDot.contains(".")) {
                // Check if the last part before the dot looks like a class name
                int lastDot = withoutTrailingDot.lastIndexOf('.');
                String lastPart = withoutTrailingDot.substring(lastDot + 1);
                if (!lastPart.isEmpty() && Character.isUpperCase(lastPart.charAt(0))) {
                    return SignatureState.CLASS_ONLY;
                }
            }
            return SignatureState.PACKAGE_ONLY;
        }

        // Split by dots and analyze the parts
        String[] parts = trimmed.split("\\.");
        if (parts.length < 2) {
            return SignatureState.METHOD_NAME_ONLY;
        }

        // Check the last part to determine the state
        String lastPart = parts[parts.length - 1];
        String secondToLastPart = parts.length > 1 ? parts[parts.length - 2] : "";

        // If last part starts with uppercase, it could be a class name
        if (!lastPart.isEmpty() && Character.isUpperCase(lastPart.charAt(0))) {
            // If the second-to-last part also starts with uppercase,
            // this might be nested classes (e.g., "com.pkg.Outer.Inner")
            if (!secondToLastPart.isEmpty() && Character.isUpperCase(secondToLastPart.charAt(0))) {
                return SignatureState.CLASS_ONLY;
            }

            // Check if this looks like a package.Class pattern
            // If we have at least one lowercase part before the uppercase part, it's likely a class
            for (int i = 0; i < parts.length - 1; i++) {
                if (!parts[i].isEmpty() && Character.isLowerCase(parts[i].charAt(0))) {
                    return SignatureState.CLASS_ONLY;
                }
            }
        }

        // If last part starts with lowercase, we need to be more careful
        if (!lastPart.isEmpty() && Character.isLowerCase(lastPart.charAt(0))) {
            // Check if this looks like a method call pattern
            // If we have a clear class name (uppercase) followed by a lowercase name, it's likely a method
            if (!secondToLastPart.isEmpty() && Character.isUpperCase(secondToLastPart.charAt(0))) {
                return SignatureState.METHOD_NAME_ONLY;
            }

            // If all parts are lowercase, it's likely a package
            // This handles cases like "com.example" where all parts are lowercase
            boolean allLowercase = true;
            for (String part : parts) {
                if (!part.isEmpty() && Character.isUpperCase(part.charAt(0))) {
                    allLowercase = false;
                    break;
                }
            }
            if (allLowercase) {
                return SignatureState.PACKAGE_ONLY;
            }

            // Otherwise, it's likely a method name
            return SignatureState.METHOD_NAME_ONLY;
        }

        // Default to package if we can't determine otherwise
        return SignatureState.PACKAGE_ONLY;
    }

    public static String extractClassName(String beforeParams) {
        int dot = beforeParams.lastIndexOf('.');
        return beforeParams.substring(0, dot);
    }

    public static String extractMethodName(String beforeParams) {
        int dot = beforeParams.lastIndexOf('.');
        return beforeParams.substring(dot + 1);
    }

    public static String extractParamsBody(String sig) {
        int open = sig.indexOf('(');
        int close = sig.lastIndexOf(')');
        return sig.substring(open + 1, close).trim();
    }

    /**
     * Utility to parse a parameter list string into a list of type strings,
     * correctly handling nested generics by tracking angle-bracket depth.
     */
    public static List<String> parseParameterTypes(String paramsBody) {
        if (paramsBody == null || paramsBody.isEmpty()) {
            return Collections.emptyList();
        }
        List<String> types = new ArrayList<>();
        StringBuilder token = new StringBuilder();
        int depth = 0;
        for (int i = 0; i < paramsBody.length(); i++) {
            char c = paramsBody.charAt(i);
            if (c == '<') {
                depth++;
                token.append(c);
            } else if (c == '>') {
                depth--;
                token.append(c);
            } else if (c == ',' && depth == 0) {
                String t = token.toString().trim();
                if (!t.isEmpty()) {
                    types.add(t);
                }
                token.setLength(0);
            } else {
                token.append(c);
            }
        }
        String last = token.toString().trim();
        if (!last.isEmpty()) {
            types.add(last);
        }
        return types;
    }
}
