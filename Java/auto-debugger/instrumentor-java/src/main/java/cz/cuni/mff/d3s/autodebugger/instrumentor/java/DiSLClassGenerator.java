package cz.cuni.mff.d3s.autodebugger.instrumentor.java;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.InstrumentationCodeGenerator;

import java.io.File;
import java.io.FileWriter;
import java.nio.file.Path;
import java.util.Optional;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.InstrumentationModel;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public class DiSLClassGenerator extends InstrumentationCodeGenerator {

  public DiSLClassGenerator(Path outputDirectory, InstrumentationModel instrumentationModel) {
    super(outputDirectory, instrumentationModel);
  }

  @Override
  public Optional<Path> generateCode() {
    var code = instrumentationModel.transform();
    try {
      log.info("Generating DiSL class");
      String fileName = "DiSLClass.java";
      File dislClassFile = new File(outputDirectory.toString(), fileName);
      try (FileWriter writer = new FileWriter(dislClassFile)) {
        writer.write(code);
      }
      log.info("DiSL class generated");
      return Optional.of(Path.of(dislClassFile.getParentFile().getAbsolutePath()));
    } catch (Exception e) {
      log.error("Failed to generate DiSL class", e);
      return Optional.empty();
    }
  }
}
