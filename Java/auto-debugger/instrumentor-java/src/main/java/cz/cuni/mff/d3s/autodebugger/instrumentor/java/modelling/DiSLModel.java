package cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.InstrumentationModel;
import cz.cuni.mff.d3s.autodebugger.model.java.factories.IdentifierFactory;
import cz.cuni.mff.d3s.autodebugger.model.java.factories.MethodIdentifierFactory;
import cz.cuni.mff.d3s.autodebugger.model.java.identifiers.*;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.factories.ExportableValueFactory;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.enums.ActivationTime;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.enums.MarkerType;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.stream.Collectors;
import java.util.stream.Stream;

import lombok.Getter;
import lombok.extern.slf4j.Slf4j;

/**
 * DiSL-specific instrumentation model for Java applications.
 * Extends the base InstrumentationModel to provide DiSL-specific code generation
 * with predefined imports, annotations, and instrumentation logic patterns.
 */
@Slf4j
public class DiSLModel extends InstrumentationModel {

  @Getter
  private final JavaMethodIdentifier targetMethod;

  private final String DEFAULT_PACKAGE_NAME = "cz.cuni.mff.d3s.autodebugger.analyzer.disl";
  private final JavaPackage DEFAULT_PACKAGE =
      new JavaPackage(
          IdentifierFactory.createFrom(
              new IdentifierParameters(new PackageIdentifierParameters(DEFAULT_PACKAGE_NAME))));

  private final List<String> DISL_LIBRARY_IMPORTS =
      List.of(
          "ch.usi.dag.disl.annotation.After",
          "ch.usi.dag.disl.annotation.Before",
          "ch.usi.dag.disl.dynamiccontext.DynamicContext",
          "ch.usi.dag.disl.marker.BodyMarker");

  private final List<String> JAVA_IMPORTS =
      List.of(
          "java.io.FileNotFoundException",
          "java.io.IOException",
          "java.io.FileOutputStream",
          "java.io.ObjectOutputStream");

  public DiSLModel(JavaMethodIdentifier targetMethod, List<JavaValueIdentifier> exportedValues) {
    this.targetMethod = targetMethod;
    var classBuilder = DiSLClass.builder();
    List<JavaPackageImport> imports =
        Stream.concat(DISL_LIBRARY_IMPORTS.stream(), JAVA_IMPORTS.stream())
            .map(PackageIdentifierParameters::new)
            .map(IdentifierParameters::new)
            .map(IdentifierFactory::createFrom)
            .map(JavaPackageImport::new)
            .collect(Collectors.toList());
    classBuilder.imports(imports);
    List<JavaValue> exports = new ArrayList<>();
    for (var identifier : exportedValues) {
      if (identifier instanceof JavaValueIdentifier valueIdentifier) {
        exports.add(ExportableValueFactory.createFrom(valueIdentifier));
        getImport(valueIdentifier)
            .ifPresent(i -> imports.add(new JavaPackageImport(i)));
      } else {
        log.error("Variable {} is not a ExportableIdentifier", identifier);
      }
    }
    var parameters =
        MethodIdentifierParameters.builder()
            .returnType("void")
            .ownerClassIdentifier(
                new JavaClassIdentifier(
                    ClassIdentifierParameters.builder()
                        .className("DiSLClass")
                        .packageIdentifier(JavaPackageIdentifier.DEFAULT_PACKAGE)
                        .build()))
            .parameterTypes(List.of("DynamicContext"))
            .build();
    var beforeAnnotation =
        new DiSLAnnotation(
            ActivationTime.BEFORE, new DiSLMarker(MarkerType.BODY), new DiSLScope(targetMethod));
    var afterAnnotation =
        new DiSLAnnotation(
            ActivationTime.AFTER, new DiSLMarker(MarkerType.BODY), new DiSLScope(targetMethod));
    classBuilder.instrumentationMethods(
        List.of(
            new ShadowDiSLInstrumentationLogic(
                MethodIdentifierFactory.getInstance().generateIdentifier(parameters),
                beforeAnnotation,
                exports),
            new ShadowDiSLInstrumentationLogic(
                MethodIdentifierFactory.getInstance().generateIdentifier(parameters),
                afterAnnotation,
                exports)));
    rootClass = classBuilder.build();
  }

  @Override
  public String transform() {
    var result = super.transform();
    return addIndentation(result);
  }

  private Optional<JavaPackageIdentifier> getImport(
      JavaValueIdentifier valueIdentifier) {
    if (valueIdentifier instanceof JavaFieldIdentifier fieldIdentifier) {
      return Optional.of(fieldIdentifier.getOwnerClassIdentifier().getAsImportablePackage());
    }
    return Optional.empty();
  }

  private String addIndentation(String code) {
    StringBuilder indentedCode = new StringBuilder();
    int indentLevel = 0;
    for (char c : code.toCharArray()) {
      if (c == '{') {
        indentLevel++;
      }
      if (c == '}') {
        indentLevel--;
        assert (indentLevel >= 0);
        assert (indentedCode.length() - 1 >= 0);
        if (indentedCode.charAt(indentedCode.length() - 1) == '\t') {
          indentedCode.deleteCharAt(indentedCode.length() - 1);
        }
      }
      if (c == '\n') {
        indentedCode.append(c);
        indentedCode.append("\t".repeat(indentLevel));
        continue;
      }
      indentedCode.append(c);
    }
    return indentedCode.toString();
  }
}
