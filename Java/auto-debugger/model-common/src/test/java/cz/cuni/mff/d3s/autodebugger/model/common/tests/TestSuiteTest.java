package cz.cuni.mff.d3s.autodebugger.model.common.tests;

import org.junit.jupiter.api.Test;

import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.*;

class TestSuiteTest {

    @Test
    void givenBaseAndFiles_whenBuild_thenImmutableList() {
        var suite = TestSuite.builder()
                .baseDirectory(Path.of("/tmp/out"))
                .testFile(Path.of("/tmp/out/TestA.java"))
                .testFile(Path.of("/tmp/out/TestB.java"))
                .build();
        assertEquals(Path.of("/tmp/out"), suite.getBaseDirectory());
        assertEquals(2, suite.getTestFiles().size());
        assertThrows(UnsupportedOperationException.class, () -> suite.getTestFiles().add(Path.of("X")));
    }
}

