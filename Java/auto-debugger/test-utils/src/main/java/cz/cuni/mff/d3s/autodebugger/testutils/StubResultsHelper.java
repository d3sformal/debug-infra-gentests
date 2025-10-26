package cz.cuni.mff.d3s.autodebugger.testutils;

import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;

import static org.junit.jupiter.api.Assertions.assertNotNull;

/** Utility to create a minimal stub test and write its path into the autodebugger results list. */
public final class StubResultsHelper {
  private StubResultsHelper() {}

  /**
   * Writes a minimal stub test file and records its path in the provided results list file.
   * This is used by tests to simulate the output of the test generation process.
   *
   * @param resultsListPath The exact path where the results list file should be created.
   *                        This should match the path from InstrumentationResult.getResultsListPath()
   * @return Path to the created stub test file
   * @throws Exception if file creation fails
   */
  public static Path writeMinimalStubTestAndResults(Path resultsListPath) throws Exception {
    // Create parent directory for results file
    Files.createDirectories(resultsListPath.getParent());

    // Create stub test directory
    Path stubDir = resultsListPath.getParent().resolve("stub-tests");
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
    Files.write(resultsListPath, List.of(stub.toAbsolutePath().toString()));
    return stub;
  }
}

