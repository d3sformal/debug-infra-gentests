package cz.cuni.mff.d3s.autodebugger.model.java.factories;

import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.JavaPackageIdentifier;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.PackageIdentifierParameters;
import lombok.Getter;

public class PackageIdentifierFactory extends IdentifierFactory {
    @Getter
    private static final PackageIdentifierFactory instance = new PackageIdentifierFactory();

    private PackageIdentifierFactory() {
    }

    public JavaPackageIdentifier createIdentifier(PackageIdentifierParameters parameters) {
        return new JavaPackageIdentifier(parameters.packageName);
    }

    public JavaPackageIdentifier generateIdentifier() {
        return new JavaPackageIdentifier("generatedPackage" + getNextId());
    }
}
