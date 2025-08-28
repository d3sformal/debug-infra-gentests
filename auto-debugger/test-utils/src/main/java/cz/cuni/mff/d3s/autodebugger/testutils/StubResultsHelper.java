package cz.cuni.mff.d3s.autodebugger.testutils;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertNotNull;

/** Utility to create a minimal stub test and write its path into the autodebugger results list. */
public final class StubResultsHelper {
  private StubResultsHelper() {}

  public static Path writeMinimalStubTestAndResults(Path outDir) throws Exception {
    // Results list path: prefer an existing generated-tests-<runId>.lst created by the instrumentor; otherwise fallback
    Path resultsFile = outDir.resolve("generated-tests.lst");
    try {
      var candidates = Files.list(outDir)
          .filter(p -> p.getFileName().toString().startsWith("generated-tests-") && p.getFileName().toString().endsWith(".lst"))
          .sorted((a,b) -> b.getFileName().toString().compareTo(a.getFileName().toString()))
          .toList();
      if (!candidates.isEmpty()) { resultsFile = candidates.get(0); }
    } catch (Exception ignore) {}
    Files.createDirectories(resultsFile.getParent());

    Path stubDir = outDir.resolve("stub-tests");
    Files.createDirectories(stubDir);
    Path stub = stubDir.resolve("StubTest.java");

    String java = """
      import org.junit.jupiter.api.BeforeEach;
      import org.junit.jupiter.api.Test;
      import static org.junit.jupiter.api.Assertions.*;
      public class StubTest {
        @BeforeEach void setup(){ Object o = new Object(); assertNotNull(o); }
        @Test void testExample(){
          // Arrange
          int a = 1;
          // Act
          int b = a + 1;
          // Assert
          assertTrue(b > a);
        }
      }
      """;

    Files.writeString(stub, java);
    Files.write(resultsFile, List.of(stub.toAbsolutePath().toString()));
    return stub;
  }
}

