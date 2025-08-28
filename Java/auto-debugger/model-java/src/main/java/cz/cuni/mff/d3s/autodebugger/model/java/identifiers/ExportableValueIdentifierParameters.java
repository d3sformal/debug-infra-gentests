package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import lombok.AllArgsConstructor;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;

@SuperBuilder
@AllArgsConstructor
public abstract class ExportableValueIdentifierParameters {
    public String variableName;
    @NonNull
    public String variableType;

}

