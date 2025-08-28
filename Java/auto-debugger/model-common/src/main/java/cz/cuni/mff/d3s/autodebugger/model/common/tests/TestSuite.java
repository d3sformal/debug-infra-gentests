package cz.cuni.mff.d3s.autodebugger.model.common.tests;

import lombok.Builder;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.Singular;
import lombok.ToString;

import java.nio.file.Path;
import java.util.Collections;
import java.util.List;

/**
 * Represents a set of generated tests to be compiled and executed.
 * Provides a typed shape instead of passing around lists of paths.
 */
@Getter
@Builder
@EqualsAndHashCode
@ToString
public class TestSuite {

    /** Base directory for the test files (for convenient relative resolution). */
    private final Path baseDirectory;

    /** The generated test files (e.g., Java sources). */
    @Singular("testFile")
    private final List<Path> testFiles;

    public List<Path> getTestFiles() {
        return testFiles == null ? List.of() : Collections.unmodifiableList(testFiles);
    }
}

