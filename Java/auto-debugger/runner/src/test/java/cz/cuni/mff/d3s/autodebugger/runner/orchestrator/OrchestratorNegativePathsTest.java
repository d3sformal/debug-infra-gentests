package cz.cuni.mff.d3s.autodebugger.runner.orchestrator;

import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class OrchestratorNegativePathsTest {

  @TempDir Path tempDir;

  private Arguments argsWith(Path appJar, Path sourceDir, Path dislHome) {
    Arguments a = new Arguments();
    a.language = TargetLanguage.JAVA;
    a.applicationJarPath = appJar.toString();
    a.sourceCodePath = sourceDir.toString();
    a.dislHomePath = dislHome.toString();
    a.targetMethodReference = "org.example.Calculator.add(int,int)";
    a.targetParameters = List.of("0:int", "1:int");
    a.targetFields = List.of();
    a.classpath = List.of();
    a.runtimeArguments = List.of();
    a.testGenerationStrategy = "trace-based-basic";
    return a;
  }

  @Test
  void invalidDislHome_shouldFailValidation() throws Exception {
    Path appJar = Files.createFile(tempDir.resolve("app.jar"));
    Path source = Files.createDirectories(tempDir.resolve("src"));
    Path dislHome = tempDir.resolve("missing-disl"); // does not exist

    RuntimeException ex = assertThrows(RuntimeException.class, () -> {
      new Orchestrator(argsWith(appJar, source, dislHome)).buildInstrumentationModel();
    });
    assertNotNull(ex.getCause());
    assertTrue(ex.getCause() instanceof IllegalStateException);
    assertTrue(ex.getCause().getMessage().toLowerCase().contains("disl"));
  }

  @Test
  void invalidAppJar_shouldFailValidation() throws Exception {
    Path appJar = tempDir.resolve("missing.jar"); // missing
    Path source = Files.createDirectories(tempDir.resolve("src"));
    Path dislHome = Files.createDirectories(tempDir.resolve("disl"));
    Files.createDirectories(dislHome.resolve("bin"));
    Files.createFile(dislHome.resolve("bin").resolve("disl.py"));
    Files.createDirectories(dislHome.resolve("output").resolve("lib"));

    RuntimeException ex = assertThrows(RuntimeException.class, () -> {
      new Orchestrator(argsWith(appJar, source, dislHome)).buildInstrumentationModel();
    });
    assertNotNull(ex.getCause());
    assertTrue(ex.getCause() instanceof IllegalStateException);
    String msg = ex.getCause().getMessage().toLowerCase();
    assertTrue(msg.contains("application") || msg.contains("jar"));
  }
}

