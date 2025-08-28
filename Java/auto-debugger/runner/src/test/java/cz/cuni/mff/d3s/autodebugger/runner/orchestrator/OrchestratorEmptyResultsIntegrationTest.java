package cz.cuni.mff.d3s.autodebugger.runner.orchestrator;

import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

class OrchestratorEmptyResultsIntegrationTest {

  @TempDir Path tempDir;

  private Arguments baseArgs(Path outputDir, Path dislHome, Path appJar, Path sourceDir) {
    Arguments args = new Arguments();
    args.language = cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage.JAVA;
    args.applicationJarPath = appJar.toString();
    args.sourceCodePath = sourceDir.toString();
    args.dislHomePath = dislHome.toString();
    args.targetMethodReference = "com.example.SimpleAdder.add(int, int)";
    args.testGenerationStrategy = "trace-based-basic";
    args.classpath = List.of();
    args.targetParameters = List.of("0:int", "1:int");
    args.targetFields = List.of();
    args.runtimeArguments = List.of();
    return args;
  }

  @Test
  void whenStubDisabled_andNoSignals_thenRunAnalysisThrows() throws Exception {
    // given: no stub and no pre-created results; analysis should produce an empty result

    Path sourceDir = tempDir.resolve("src");
    Path outputDir = tempDir.resolve("output");
    Path dislHome = tempDir.resolve("disl");
    Path appJar = tempDir.resolve("app.jar");
    Files.createDirectories(sourceDir);
    Files.createDirectories(outputDir);
    Files.createDirectories(dislHome.resolve("bin"));
    Files.createDirectories(dislHome.resolve("output").resolve("lib"));
    Files.createFile(dislHome.resolve("bin").resolve("disl.py"));
    Files.createFile(appJar);

    Orchestrator orchestrator = new Orchestrator(baseArgs(outputDir, dislHome, appJar, sourceDir));

    var model = orchestrator.buildInstrumentationModel();
    var instrumentation = orchestrator.createInstrumentation(model);

    // when/then: runAnalysis must throw because results are empty
    IllegalStateException ex = assertThrows(IllegalStateException.class, () -> orchestrator.runAnalysis(instrumentation));
    assertTrue(ex.getMessage().contains("no test files"));
  }
}

