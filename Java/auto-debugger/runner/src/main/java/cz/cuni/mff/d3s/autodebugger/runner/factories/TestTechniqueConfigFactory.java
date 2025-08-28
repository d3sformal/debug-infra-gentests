package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.model.common.technique.TestTechniqueConfig;
import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;

/**
 * Builds a TestTechniqueConfig from CLI Arguments and defaults.
 */
public class TestTechniqueConfigFactory {

    public static TestTechniqueConfig fromArguments(Arguments arguments) {
        return TestTechniqueConfig.builder()
                .id(arguments.testGenerationStrategy)
                .apiKey(arguments.apiKey)
                .build();
    }
}
