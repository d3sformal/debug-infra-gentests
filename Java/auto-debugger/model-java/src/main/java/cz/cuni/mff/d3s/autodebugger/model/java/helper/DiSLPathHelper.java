package cz.cuni.mff.d3s.autodebugger.model.java.helper;

import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;

import java.nio.file.Path;

public class DiSLPathHelper {
    public static Path getDislClassPathRoot(JavaRunConfiguration runConfiguration) {
        return runConfiguration.getDislHomePath().resolve("output/lib/");
    }

    public static Path getDislRunnerPath(JavaRunConfiguration runConfiguration) {
        return runConfiguration.getDislHomePath().resolve("bin/disl.py");
    }

    public static Path getDislHomePath(JavaRunConfiguration runConfiguration) {
        return runConfiguration.getDislHomePath().resolve("output");
    }
}
