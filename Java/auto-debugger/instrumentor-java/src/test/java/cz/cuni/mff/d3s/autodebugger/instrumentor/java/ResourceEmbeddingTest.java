package cz.cuni.mff.d3s.autodebugger.instrumentor.java;

import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.io.InputStream;

import static org.junit.jupiter.api.Assertions.*;

class ResourceEmbeddingTest {

  @Test
  void resources_are_packaged_and_accessible() throws IOException {
    try (InputStream jt = getClass().getResourceAsStream("/templates/java/disl-analysis/Collector.jt");
         InputStream re = getClass().getResourceAsStream("/templates/java/disl-analysis/CollectorRE.java")) {
      assertNotNull(jt, "Collector.jt should be on classpath under /templates/java/disl-analysis/");
      assertNotNull(re, "CollectorRE.java should be on classpath under /templates/java/disl-analysis/");
    }
  }
}

