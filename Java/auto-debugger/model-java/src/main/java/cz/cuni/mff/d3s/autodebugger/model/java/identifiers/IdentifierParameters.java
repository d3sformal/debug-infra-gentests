package cz.cuni.mff.d3s.autodebugger.model.java.identifiers;

import lombok.Getter;

import java.util.Optional;

@Getter
public class IdentifierParameters {
    private final Optional<PackageIdentifierParameters> packageParameters;
    private final Optional<VariableIdentifierParameters> variableParameters;
    private final Optional<MethodIdentifierParameters> methodParameters;

    public IdentifierParameters(PackageIdentifierParameters parameters) {
        this.packageParameters = Optional.of(parameters);
        this.variableParameters = Optional.empty();
        this.methodParameters = Optional.empty();
    }

    public IdentifierParameters(VariableIdentifierParameters parameters) {
        this.packageParameters = Optional.empty();
        this.variableParameters = Optional.of(parameters);
        this.methodParameters = Optional.empty();
    }

    public IdentifierParameters(MethodIdentifierParameters parameters) {
        this.packageParameters = Optional.empty();
        this.variableParameters = Optional.empty();
        this.methodParameters = Optional.of(parameters);
    }
}
