package cz.cuni.mff.d3s.autodebugger.runner.config;

/**
 * Represents a configuration field that can be displayed in the settings dialog.
 */
public class ConfigurationField {
    private final String key;
    private final String displayName;
    private final String description;
    private final FieldType type;
    private final boolean required;
    private final String defaultValue;

    public ConfigurationField(String key, String displayName, String description, 
                            FieldType type, boolean required, String defaultValue) {
        this.key = key;
        this.displayName = displayName;
        this.description = description;
        this.type = type;
        this.required = required;
        this.defaultValue = defaultValue;
    }

    public String getKey() {
        return key;
    }

    public String getDisplayName() {
        return displayName;
    }

    public String getDescription() {
        return description;
    }

    public FieldType getType() {
        return type;
    }

    public boolean isRequired() {
        return required;
    }

    public String getDefaultValue() {
        return defaultValue;
    }

    /**
     * Types of configuration fields supported.
     */
    public enum FieldType {
        TEXT,           // Simple text input
        PASSWORD,       // Password input (masked)
        FILE_PATH,      // File path with browse button
        DIRECTORY_PATH, // Directory path with browse button
        NUMBER,         // Numeric input
        BOOLEAN         // Checkbox
    }
}
