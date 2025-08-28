package cz.cuni.mff.d3s.autodebugger.model.common;

import lombok.Getter;

/**
 * Enumeration of supported target programming languages for the auto-debugger.
 * This enum provides type safety and prevents runtime errors from invalid language strings.
 */
@Getter
public enum TargetLanguage {
    
    JAVA("java", "Java", ".jar", ".java");
    

    /**
     * The lowercase identifier used in command line arguments and configuration.
     */
    private final String identifier;
    
    /**
     * The human-readable display name for the language.
     */
    private final String displayName;
    
    /**
     * The typical file extension for executable/application files in this language.
     */
    private final String executableExtension;
    
    /**
     * The typical file extension for source code files in this language.
     */
    private final String sourceExtension;
    
    TargetLanguage(String identifier, String displayName, String executableExtension, String sourceExtension) {
        this.identifier = identifier;
        this.displayName = displayName;
        this.executableExtension = executableExtension;
        this.sourceExtension = sourceExtension;
    }
    
    /**
     * Parses a string identifier into a TargetLanguage enum value.
     * The parsing is case-insensitive.
     * 
     * @param identifier The string identifier (e.g., "java", "JAVA", "Java")
     * @return The corresponding TargetLanguage enum value
     * @throws IllegalArgumentException if the identifier is not recognized
     */
    public static TargetLanguage fromIdentifier(String identifier) {
        if (identifier == null || identifier.trim().isEmpty()) {
            throw new IllegalArgumentException("Language identifier cannot be null or empty");
        }
        
        String normalizedIdentifier = identifier.toLowerCase().trim();
        
        for (TargetLanguage language : values()) {
            if (language.identifier.equals(normalizedIdentifier)) {
                return language;
            }
        }
        
        throw new IllegalArgumentException(
            "Unsupported language: '" + identifier + "'. Supported languages: " + getSupportedLanguagesString());
    }
    
    /**
     * Checks if a language identifier is supported.
     * 
     * @param identifier The string identifier to check
     * @return true if the language is supported, false otherwise
     */
    public static boolean isSupported(String identifier) {
        try {
            fromIdentifier(identifier);
            return true;
        } catch (IllegalArgumentException e) {
            return false;
        }
    }
    
    /**
     * Gets a comma-separated string of all supported language identifiers.
     * 
     * @return String containing all supported language identifiers
     */
    public static String getSupportedLanguagesString() {
        StringBuilder sb = new StringBuilder();
        for (TargetLanguage language : values()) {
            if (!sb.isEmpty()) {
                sb.append(", ");
            }
            sb.append(language.identifier);
        }
        return sb.toString();
    }
    
    /**
     * Validates that a file extension matches this language's expected extensions.
     * 
     * @param filename The filename to validate
     * @param isExecutable true if checking executable extension, false for source extension
     * @return true if the extension matches, false otherwise
     */
    public boolean validateFileExtension(String filename, boolean isExecutable) {
        if (filename == null || filename.isEmpty()) {
            return false;
        }
        
        String expectedExtension = isExecutable ? executableExtension : sourceExtension;
        return filename.toLowerCase().endsWith(expectedExtension.toLowerCase());
    }
    
    @Override
    public String toString() {
        return displayName;
    }
}
