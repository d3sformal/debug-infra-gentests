package cz.cuni.mff.d3s.autodebugger.model.common.artifacts;

import org.junit.jupiter.api.Test;

import java.nio.file.Path;

import static org.junit.jupiter.api.Assertions.*;

class InstrumentationResultTest {

    @Test
    void givenMinimal_whenBuild_thenHasPrimaryArtifact() {
        var res = InstrumentationResult.builder()
                .primaryArtifact(Path.of("/tmp/instr.jar"))
                .build();
        assertEquals(Path.of("/tmp/instr.jar"), res.getPrimaryArtifact());
        assertTrue(res.getAdditionalArtifacts().isEmpty());
        assertNull(res.getIdentifiersMappingPath());
        assertNull(res.getResultsListPath());
    }

    @Test
    void givenFull_whenBuild_thenExposesAllFields() {
        var res = InstrumentationResult.builder()
                .primaryArtifact(Path.of("/tmp/instr.jar"))
                .artifact(Path.of("/tmp/a.txt"))
                .identifiersMappingPath(Path.of("/tmp/ids.ser"))
                .resultsListPath(Path.of("/tmp/generated-tests.lst"))
                .build();
        assertEquals(1, res.getAdditionalArtifacts().size());
        assertEquals(Path.of("/tmp/a.txt"), res.getAdditionalArtifacts().get(0));
        assertEquals(Path.of("/tmp/ids.ser"), res.getIdentifiersMappingPath());
        assertEquals(Path.of("/tmp/generated-tests.lst"), res.getResultsListPath());
    }
}

