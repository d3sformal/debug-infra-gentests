package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.runner.strategies.TestGenerationStrategyProvider;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.LLMConfiguration;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.TestGenerator;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.llm.LLMBasedTestGenerator;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.AnthropicClient;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.llm.PromptBuilder;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.llm.CodeValidator;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.trace.NaiveTraceBasedGenerator;
import lombok.extern.slf4j.Slf4j;

import java.nio.file.Path;

@Slf4j
public class TestGeneratorFactory {

    public static TestGenerator createTestGenerator(RunConfiguration runConfiguration, String strategyId) {
        return createTestGenerator(runConfiguration, strategyId, null);
    }

    public static TestGenerator createTestGenerator(RunConfiguration runConfiguration, String strategyId, String apiKey) {
        return createTestGenerator(runConfiguration, strategyId, apiKey, null);
    }

    public static TestGenerator createTestGenerator(RunConfiguration runConfiguration, String strategyId, String apiKey, Path identifierMappingPath) {
        TargetLanguage language = runConfiguration.getLanguage();
        if (language == TargetLanguage.JAVA) {
            return createJavaTestGenerator(runConfiguration, strategyId, apiKey, identifierMappingPath);
        }

        throw new IllegalArgumentException("Unsupported language: " + language);
    }

    private static TestGenerator createJavaTestGenerator(RunConfiguration runConfiguration, String strategyId, String apiKey, Path identifierMappingPath) {
        log.info("Creating Java test generator with strategy: {}", strategyId);

        // Validate that the strategy exists
        if (!TestGenerationStrategyProvider.hasStrategy(strategyId)) {
            throw new IllegalArgumentException("Unknown test generation strategy: " + strategyId);
        }

        if (runConfiguration instanceof JavaRunConfiguration javaRunConfiguration) {
            try {
                if ("ai-assisted".equals(strategyId)) {
                    // Create dependencies for LLM-based test generator
                    AnthropicClient anthropicClient = new AnthropicClient();
                    PromptBuilder promptBuilder = new PromptBuilder();
                    CodeValidator codeValidator = new CodeValidator();

                    // Create LLM-based test generator with dependencies
                    LLMBasedTestGenerator llmGenerator = new LLMBasedTestGenerator(
                            anthropicClient, promptBuilder, codeValidator);

                    // Configure with Anthropic Claude settings
                    String resolvedApiKey = getApiKeyFromEnvironmentOrConfig(apiKey);

                    LLMConfiguration llmConfig = LLMConfiguration.builder()
                            .modelName("claude-sonnet-4-20250514")
                            .apiKey(resolvedApiKey)
                            .maxTokens(4000)
                            .temperature(0.3)
                            .build();

                    llmGenerator.configure(llmConfig);
                    // Set technique label for UI/tests compatibility
                    llmGenerator.setGenerationTechnique("ai-assisted");

                    log.info("Successfully created LLM-based Java test generator with Claude");
                    return llmGenerator;

                } else if (strategyId.startsWith("trace-based")) {
                    // Create the trace-based test generator
                    // Use provided identifier mapping path if available, otherwise fall back to default location
                    Path identifiersPath = identifierMappingPath != null
                            ? identifierMappingPath
                            : javaRunConfiguration.getSourceCodePath().resolve("identifiers");
                    NaiveTraceBasedGenerator generator = new NaiveTraceBasedGenerator(identifiersPath);

                    log.info("Successfully created Java test generator with strategy: {}", strategyId);
                    return generator;
                } else {
                    throw new UnsupportedOperationException("Test generation technique not yet supported: " + strategyId);
                }
            } catch (Exception e) {
                log.error("Failed to create Java test generator", e);
                throw new RuntimeException("Failed to create test generator", e);
            }
        }

        throw new IllegalArgumentException("Expected JavaRunConfiguration, got: " + runConfiguration.getClass().getSimpleName());
    }

    private static String getApiKeyFromEnvironmentOrConfig(String providedApiKey) {
        // First try provided API key from command line
        if (providedApiKey != null && !providedApiKey.trim().isEmpty()) {
            return providedApiKey;
        }

        // Try Anthropic environment variable
        String anthropicKey = System.getenv("ANTHROPIC_API_KEY");
        if (anthropicKey != null && !anthropicKey.trim().isEmpty()) {
            return anthropicKey;
        }

        // Return null to let the LLMConfiguration handle the missing API key
        // The configuration will throw an appropriate exception during validation
        return null;
    }
}
