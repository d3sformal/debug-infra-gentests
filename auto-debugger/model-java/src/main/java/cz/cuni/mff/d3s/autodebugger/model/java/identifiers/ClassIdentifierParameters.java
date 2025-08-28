package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import lombok.AllArgsConstructor;
import lombok.Builder;

@Builder
@AllArgsConstructor
public class ClassIdentifierParameters {
    public String className;
    public JavaPackageIdentifier packageIdentifier;
}
