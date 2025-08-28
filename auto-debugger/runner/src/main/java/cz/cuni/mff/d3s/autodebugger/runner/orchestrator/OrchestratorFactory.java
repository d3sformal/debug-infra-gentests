package cz.cuni.mff.d3s.autodebugger.runner.orchestrator;

import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;

/**
 * Factory for creating orchestrator instances based on target language.
 * This is a simplified factory that creates the unified Orchestrator class
 * but provides the interface expected by tests.
 */
public class OrchestratorFactory {

    /**
     * Creates an orchestrator for the specified language using enum.
     *
     * @param language The target language
     * @return Orchestrator instance
     * @throws IllegalArgumentException if language is null or unsupported
     */
    public static Orchestrator create(TargetLanguage language) {
        if (language == null) {
            throw new IllegalArgumentException("Language cannot be null");
        }

        if (!isLanguageSupported(language)) {
            throw new IllegalArgumentException("Unsupported language: " + language);
        }

        // Create language-specific orchestrator
        if (language == TargetLanguage.JAVA) {
            return new JavaOrchestrator();
        }

        throw new IllegalArgumentException("Unsupported language: " + language);
    }

    /**
     * Creates an orchestrator for the specified language using arguments.
     *
     * @param arguments The arguments containing language and configuration
     * @return Orchestrator instance
     * @throws IllegalArgumentException if arguments or language is null or unsupported
     */
    public static Orchestrator create(Arguments arguments) {
        if (arguments == null) {
            throw new IllegalArgumentException("Arguments cannot be null");
        }

        if (arguments.language == null) {
            throw new IllegalArgumentException("Language cannot be null");
        }

        if (!isLanguageSupported(arguments.language)) {
            throw new IllegalArgumentException("Unsupported language: " + arguments.language);
        }

        // Create language-specific orchestrator
        if (arguments.language == TargetLanguage.JAVA) {
            return new JavaOrchestrator(arguments);
        }

        throw new IllegalArgumentException("Unsupported language: " + arguments.language);
    }

    /**
     * Creates an orchestrator for the specified language using string identifier.
     *
     * @param languageId The language identifier (case-insensitive)
     * @return Orchestrator instance
     * @throws IllegalArgumentException if languageId is null, empty, or unsupported
     */
    public static Orchestrator create(String languageId) {
        if (languageId == null || languageId.trim().isEmpty()) {
            throw new IllegalArgumentException("Language identifier cannot be null or empty");
        }

        TargetLanguage language = TargetLanguage.fromIdentifier(languageId);
        return create(language);
    }

    /**
     * Checks if the specified language is supported.
     *
     * @param language The target language
     * @return true if supported, false otherwise
     */
    public static boolean isLanguageSupported(TargetLanguage language) {
        return language == TargetLanguage.JAVA;
    }

    /**
     * Checks if the specified language identifier is supported.
     *
     * @param languageId The language identifier
     * @return true if supported, false otherwise
     */
    public static boolean isLanguageSupported(String languageId) {
        if (languageId == null || languageId.trim().isEmpty()) {
            return false;
        }

        try {
            TargetLanguage language = TargetLanguage.fromIdentifier(languageId);
            return isLanguageSupported(language);
        } catch (IllegalArgumentException e) {
            return false;
        }
    }

    /**
     * Gets a string representation of all supported languages.
     *
     * @return Comma-separated list of supported languages
     */
    public static String getSupportedLanguages() {
        return "java";
    }

    /**
     * Creates dummy arguments for testing purposes.
     */
    private static Arguments createDummyArguments(TargetLanguage language) {
        Arguments args = new Arguments();
        args.language = language;
        args.applicationJarPath = "dummy.jar";
        args.sourceCodePath = "src";
        args.dislHomePath = "disl";
        args.targetMethodReference = "org.example.Test.method()";
        args.testGenerationStrategy = "trace-based-basic";
        args.classpath = java.util.List.of();
        args.targetParameters = java.util.List.of();
        args.targetFields = java.util.List.of();
        args.runtimeArguments = java.util.List.of();
        return args;
    }
}
