package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.analyzer.common.Analyzer;
import cz.cuni.mff.d3s.autodebugger.analyzer.java.DiSLAnalyzer;
import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import lombok.extern.slf4j.Slf4j;

/**
 * Factory for creating language-specific Analyzer instances.
 * Uses factory pattern to abstract analyzer creation based on
 * target language, configuring appropriate analyzer implementations.
 */
@Slf4j
public class AnalyzerFactory {

    /**
     * Creates a language-specific Analyzer from run configuration.
     * Dispatches to appropriate language-specific factory method based on target language.
     */
    public static Analyzer createAnalyzer(RunConfiguration runConfiguration) {
        TargetLanguage language = runConfiguration.getLanguage();
        if (language == TargetLanguage.JAVA) {
            return createJavaAnalyzer(runConfiguration);
        }

        throw new IllegalArgumentException("Unsupported language: " + language);
    }

    private static DiSLAnalyzer createJavaAnalyzer(RunConfiguration runConfiguration) {
        log.info("Creating DiSL analyzer");

        if (runConfiguration instanceof JavaRunConfiguration javaRunConfiguration) {
            DiSLAnalyzer analyzer = new DiSLAnalyzer(javaRunConfiguration);
            log.info("Successfully created DiSL instrumentor");
            return analyzer;
        }

        throw new IllegalArgumentException("Expected JavaRunConfiguration, got: " + runConfiguration.getClass().getSimpleName());
    }
}
