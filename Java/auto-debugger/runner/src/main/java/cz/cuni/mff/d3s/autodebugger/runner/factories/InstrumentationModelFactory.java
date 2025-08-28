package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.instrumentor.common.modelling.InstrumentationModel;
import cz.cuni.mff.d3s.autodebugger.instrumentor.java.modelling.DiSLModel;
import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public class InstrumentationModelFactory {

    public static InstrumentationModel buildInstrumentationModel(RunConfiguration runConfiguration) {
        TargetLanguage language = runConfiguration.getLanguage();
        if (language == TargetLanguage.JAVA) {
            return createJavaInstrumentationModel(runConfiguration);
        }

        throw new IllegalArgumentException("Unsupported language: " + language);
    }

    private static DiSLModel createJavaInstrumentationModel(RunConfiguration runConfiguration) {
        log.info("Building DiSL instrumentation model");

        if (runConfiguration instanceof JavaRunConfiguration javaRunConfiguration) {
            DiSLModel model = new DiSLModel(javaRunConfiguration.getTargetMethod(), javaRunConfiguration.getExportableValues());
            log.info("Successfully built DiSL instrumentation model");
            return model;
        }

        throw new IllegalArgumentException("Expected JavaRunConfiguration, got: " + runConfiguration.getClass().getSimpleName());
    }
}
