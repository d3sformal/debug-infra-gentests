package cz.cuni.mff.d3s.autodebugger.instrumentor.java;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import lombok.AllArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.javatuples.Pair;

@Slf4j
@AllArgsConstructor
public class JavaTemplateHandler {
  private JavaTemplateTransformer transformer;

  public void transformFile(Path template, Path output, Pair<String, ?>... values) {
    try (var lines = Files.lines(template); var outputWriter = Files.newBufferedWriter(output)) {
      lines.map(line -> transformer.transform(line, values)).forEach(line -> {
        try {
            outputWriter.write(line);
            outputWriter.newLine();
        } catch (IOException e) {
          log.error("Failed to write transformed line to file", e);
          throw new RuntimeException(e);
        }
      });
    } catch (IOException e) {
        log.error("Failed to read template file", e);
        throw new RuntimeException(e);
    }
  }
}
