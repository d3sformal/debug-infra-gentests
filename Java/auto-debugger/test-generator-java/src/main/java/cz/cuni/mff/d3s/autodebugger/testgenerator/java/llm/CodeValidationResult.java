package cz.cuni.mff.d3s.autodebugger.testgenerator.java.llm;

import lombok.Builder;
import lombok.Getter;
import lombok.Singular;

import java.util.List;

/**
 * Result of code validation, containing validation status and any errors or warnings found.
 */
@Builder
@Getter
public class CodeValidationResult {
    
    /**
     * Whether the code passed validation (no errors).
     */
    private final boolean valid;
    
    /**
     * List of validation errors that prevent compilation.
     */
    @Singular
    private final List<String> errors;
    
    /**
     * List of validation warnings (code will compile but may have issues).
     */
    @Singular
    private final List<String> warnings;
    
    /**
     * Checks if there are any errors.
     */
    public boolean hasErrors() {
        return !errors.isEmpty();
    }
    
    /**
     * Checks if there are any warnings.
     */
    public boolean hasWarnings() {
        return !warnings.isEmpty();
    }
    
    /**
     * Gets the total number of issues (errors + warnings).
     */
    public int getTotalIssueCount() {
        return errors.size() + warnings.size();
    }
    
    /**
     * Gets a summary of the validation result.
     */
    public String getSummary() {
        if (valid && !hasWarnings()) {
            return "Code validation passed with no issues";
        } else if (valid && hasWarnings()) {
            return String.format("Code validation passed with %d warnings", warnings.size());
        } else {
            return String.format("Code validation failed with %d errors and %d warnings", 
                               errors.size(), warnings.size());
        }
    }
}
