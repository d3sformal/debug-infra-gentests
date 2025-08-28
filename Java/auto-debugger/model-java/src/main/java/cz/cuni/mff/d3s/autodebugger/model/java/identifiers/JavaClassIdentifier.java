package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.Identifier;
import lombok.Getter;

import java.io.Serializable;

/**
 * Java class identifier combining class name with package information.
 * Implements Identifier interface to provide consistent naming and
 * supports conversion to importable package format for code generation.
 */
@Getter
public class JavaClassIdentifier implements Identifier, Serializable {
  private final String className;
  private final JavaPackageIdentifier packageIdentifier;

  public JavaClassIdentifier(ClassIdentifierParameters parameters) {
    this.className = parameters.className;
    this.packageIdentifier = parameters.packageIdentifier;
  }

  /**
   * Converts this class identifier to an importable package identifier.
   * Combines package name and class name to create a fully qualified import path.
   */
  public JavaPackageIdentifier getAsImportablePackage() {
    return new JavaPackageIdentifier(packageIdentifier.getPackageName() + "." + className);
  }

  @Override
  public String getName() {
    return className;
  }
}
