package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

import java.nio.file.Path;
import java.util.List;

public interface UnitTestGenerator {
    List<Path> generateUnitTests();
}
