package cz.cuni.mff.d3s.autodebugger.instrumentor.java;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.List;
import java.util.Optional;
import java.util.jar.JarFile;
import java.util.jar.Manifest;

import static org.junit.jupiter.api.Assertions.*;

class DiSLCompilerStubJarTest {

  @TempDir
  Path tmp;

  @Test
  void givenMissingDiSL_whenCompile_thenCreatesStubJarWithExpectedEntriesAndManifest() throws IOException {
    // given
    Path outJar = tmp.resolve("stub-instrumentation.jar");
    // dislClasspathRoot can be any path that does not contain disl jars; pass tmp
    DiSLCompiler compiler = new DiSLCompiler(outJar, tmp, List.of());

    // Create minimal source directory with expected java files (names only used when real compile path is taken)
    Path srcDir = Files.createDirectory(tmp.resolve("src"));
    Files.writeString(srcDir.resolve("DiSLClass.java"), "public class DiSLClass {}\n");
    Files.writeString(srcDir.resolve("Collector.java"), "public class Collector {}\n");
    Files.writeString(srcDir.resolve("CollectorRE.java"), "public class CollectorRE {}\n");

    // when: compileDiSLClass should detect missing DiSL deps and produce a stub jar
    Optional<Path> jar = compiler.compileDiSLClass(srcDir);

    // then
    assertTrue(jar.isPresent(), "Stub jar should be created");
    assertTrue(Files.exists(jar.get()), "Stub jar file must exist");

    try (JarFile jf = new JarFile(jar.get().toFile())) {
      assertNotNull(jf.getEntry("DiSLClass.class"), "Jar must contain DiSLClass.class");
      assertNotNull(jf.getEntry("Collector.class"), "Jar must contain Collector.class");
      assertNotNull(jf.getEntry("CollectorRE.class"), "Jar must contain CollectorRE.class");
      Manifest mf = jf.getManifest();
      assertNotNull(mf, "Jar must contain a manifest");
      assertEquals("DiSLClass", mf.getMainAttributes().getValue("DiSL-Classes"));
      assertEquals("true", mf.getMainAttributes().getValue("Test-Stub"));
    }
  }
}

