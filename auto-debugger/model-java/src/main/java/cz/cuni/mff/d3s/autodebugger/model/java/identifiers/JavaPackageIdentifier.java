package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.Identifier;
import lombok.AllArgsConstructor;
import lombok.Getter;

import java.io.Serializable;

/**
 * Java package identifier for organizing classes in namespaces.
 * Implements Identifier interface and provides a default package constant
 * for classes without explicit package declarations.
 */
@Getter
@AllArgsConstructor
public class JavaPackageIdentifier implements Identifier, Serializable {
    private final String packageName;
    public static final JavaPackageIdentifier DEFAULT_PACKAGE = new JavaPackageIdentifier("");

    @Override
    public String getName() {
        return packageName;
    }
}
