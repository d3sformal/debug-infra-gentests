package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.Instrumentor;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.DiSLInstrumentor;
import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.common.technique.TestTechniqueConfig;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public class InstrumentorFactory {

    public static Instrumentor createInstrumentor(RunConfiguration runConfiguration, TestTechniqueConfig technique) {
        TargetLanguage language = runConfiguration.getLanguage();
        if (language == TargetLanguage.JAVA) {
            return createJavaInstrumentor(runConfiguration, technique);
        }
        throw new IllegalArgumentException("Unsupported language: " + language);
    }

    private static DiSLInstrumentor createJavaInstrumentor(RunConfiguration runConfiguration, TestTechniqueConfig technique) {
        log.info("Building DiSL instrumentor");
        if (runConfiguration instanceof JavaRunConfiguration javaRunConfiguration) {
            DiSLInstrumentor instrumentor = DiSLInstrumentor.builder()
                .runConfiguration(javaRunConfiguration)
                .strategyId(technique != null ? technique.getId() : null)
                .apiKey(technique != null ? technique.getApiKey() : null)
                .build();
            log.info("Successfully built DiSL instrumentor");
            return instrumentor;
        }
        throw new IllegalArgumentException("Expected JavaRunConfiguration, got: " + runConfiguration.getClass().getSimpleName());
    }
}
