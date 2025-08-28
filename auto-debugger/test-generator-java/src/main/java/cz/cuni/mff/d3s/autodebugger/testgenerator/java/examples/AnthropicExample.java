package cz.cuni.mff.d3s.autodebugger.testgenerator.java.examples;

import cz.cuni.mff.d3s.autodebugger.testgenerator.common.AnthropicClient;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.LLMConfiguration;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions.AnthropicApiException;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions.LLMConfigurationException;
import cz.cuni.mff.d3s.autodebugger.testgenerator.common.exceptions.LLMResponseException;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.llm.LLMPromptContext;
import cz.cuni.mff.d3s.autodebugger.testgenerator.java.llm.PromptBuilder;

import java.io.IOException;

/**
 * Example demonstrating how to use the Anthropic Claude integration
 * for automated test generation.
 */
public class AnthropicExample {

    public static void main(String[] args) {
        try {
            // Example 1: Basic usage with mock provider
            System.out.println("=== Example 1: Basic Mock Usage ===");
            basicMockExample();

            // Example 2: Advanced usage with runtime data
            System.out.println("\n=== Example 2: Advanced Usage with Runtime Data ===");
            advancedExample();

            // Example 3: Real Anthropic usage (requires API key)
            if (System.getenv("ANTHROPIC_API_KEY") != null) {
                System.out.println("\n=== Example 3: Real Anthropic Usage ===");
                realAnthropicExample();
            } else {
                System.out.println("\n=== Example 3: Skipped (no ANTHROPIC_API_KEY) ===");
            }

        } catch (LLMConfigurationException e) {
            System.err.println("Configuration error: " + e.getMessage());
            if (e.getConfigurationField() != null) {
                System.err.println("Field: " + e.getConfigurationField() + ", Value: " + e.getInvalidValue());
            }
        } catch (AnthropicApiException e) {
            System.err.println("API error: " + e.getMessage());
            if (e.getHttpStatusCode() != null) {
                System.err.println("HTTP Status: " + e.getHttpStatusCode());
            }
        } catch (LLMResponseException e) {
            System.err.println("Response error: " + e.getMessage());
            if (e.getIssueType() != null) {
                System.err.println("Issue Type: " + e.getIssueType());
            }
        } catch (Exception e) {
            System.err.println("Unexpected error: " + e.getMessage());
            e.printStackTrace();
        }
    }

    /**
     * Basic example using the mock provider.
     */
    private static void basicMockExample() throws LLMConfigurationException, AnthropicApiException, LLMResponseException {
        // Configure with mock model (no API key needed)
        LLMConfiguration config = LLMConfiguration.builder()
                .modelName("mock")
                .maxTokens(1000)
                .temperature(0.7)
                .build();

        // Create and configure client
        AnthropicClient client = new AnthropicClient();
        client.configure(config);

        // Simple prompt
        String prompt = """
                Generate JUnit 5 tests for this Calculator class:
                
                public class Calculator {
                    public int add(int a, int b) {
                        return a + b;
                    }
                    
                    public int multiply(int a, int b) {
                        return a * b;
                    }
                }
                """;

        // Generate tests
        String testCode = client.generateCode(prompt);
        
        System.out.println("Generated test code:");
        System.out.println(testCode);
    }

    /**
     * Advanced example using PromptBuilder with runtime data.
     */
    private static void advancedExample() throws LLMConfigurationException, AnthropicApiException, LLMResponseException {
        // Configure mock client
        LLMConfiguration config = LLMConfiguration.builder()
                .modelName("mock")
                .build();

        AnthropicClient client = new AnthropicClient();
        client.configure(config);

        // Build context with runtime data
        LLMPromptContext context = LLMPromptContext.builder()
                .sourceCode("""
                        public class StringUtils {
                            public String reverse(String input) {
                                if (input == null) return null;
                                return new StringBuilder(input).reverse().toString();
                            }
                        }
                        """)
                .targetMethodSignature("reverse(String)")
                .targetClassName("StringUtils")
                .packageName("com.example.utils")
                .testFramework("junit5")
                .generateEdgeCases(true)
                .generateNegativeTests(true)
                .traceData("""
                        reverse("hello") -> "olleh"
                        reverse("") -> ""
                        reverse("a") -> "a"
                        reverse(null) -> null
                        """)
                .additionalInstructions("Focus on null safety and edge cases")
                .build();

        // Generate optimized prompt
        PromptBuilder promptBuilder = new PromptBuilder();
        String prompt = promptBuilder.buildTestGenerationPrompt(context);

        System.out.println("Generated prompt:");
        System.out.println(prompt);
        System.out.println("\n" + "=".repeat(50) + "\n");

        // Generate tests
        String testCode = client.generateCode(prompt);
        System.out.println("Generated test code:");
        System.out.println(testCode);
    }

    /**
     * Real Anthropic example using Claude Sonnet.
     * Requires ANTHROPIC_API_KEY environment variable.
     */
    private static void realAnthropicExample() throws LLMConfigurationException, AnthropicApiException, LLMResponseException {
        // Configure with real Anthropic API
        LLMConfiguration config = LLMConfiguration.builder()
                .modelName("claude-sonnet-4-20250514") // Latest and most capable
                .apiKey(System.getenv("ANTHROPIC_API_KEY"))
                .maxTokens(2000)
                .temperature(0.2) // Lower temperature for more deterministic code
                .build();

        // Validate configuration
        config.validate();

        // Create and configure client
        AnthropicClient client = new AnthropicClient();
        client.configure(config);

        // Complex example with runtime data
        LLMPromptContext context = LLMPromptContext.builder()
                .sourceCode("""
                        public class MathUtils {
                            public double calculateCircleArea(double radius) {
                                if (radius < 0) {
                                    throw new IllegalArgumentException("Radius cannot be negative");
                                }
                                return Math.PI * radius * radius;
                            }
                        }
                        """)
                .targetMethodSignature("calculateCircleArea(double)")
                .testFramework("junit5")
                .generateEdgeCases(true)
                .generateNegativeTests(true)
                .traceData("""
                        calculateCircleArea(1.0) -> 3.141592653589793
                        calculateCircleArea(0.0) -> 0.0
                        calculateCircleArea(2.5) -> 19.634954084936208
                        """)
                .build();

        PromptBuilder promptBuilder = new PromptBuilder();
        String prompt = promptBuilder.buildTestGenerationPrompt(context);

        System.out.println("Calling Claude Sonnet 4...");
        long startTime = System.currentTimeMillis();
        
        String testCode = client.generateCode(prompt);
        
        long duration = System.currentTimeMillis() - startTime;
        System.out.println("Response received in " + duration + "ms");
        System.out.println("Generated test code:");
        System.out.println(testCode);
    }

    /**
     * Example showing different Claude models.
     */
    public static void demonstrateModels() throws LLMConfigurationException, AnthropicApiException, LLMResponseException {
        if (System.getenv("ANTHROPIC_API_KEY") == null) {
            System.out.println("Skipping model demonstration (no API key)");
            return;
        }

        String[] models = {
            "claude-sonnet-4-20250514",      // Most capable
            "claude-3-5-sonnet-20241022",    // Fast and capable  
            "claude-3-5-haiku-20241022"      // Fastest
        };

        String simplePrompt = """
                Generate a simple JUnit 5 test for this method:
                
                public int add(int a, int b) {
                    return a + b;
                }
                """;

        for (String model : models) {
            System.out.println("\n=== Testing with " + model + " ===");
            
            LLMConfiguration config = LLMConfiguration.builder()
                    .modelName(model)
                    .apiKey(System.getenv("ANTHROPIC_API_KEY"))
                    .maxTokens(1000)
                    .temperature(0.1)
                    .build();

            AnthropicClient client = new AnthropicClient();
            client.configure(config);

            long startTime = System.currentTimeMillis();
            String result = client.generateCode(simplePrompt);
            long duration = System.currentTimeMillis() - startTime;

            System.out.println("Duration: " + duration + "ms");
            System.out.println("Response length: " + result.length() + " characters");
            System.out.println("First 200 characters: " + 
                result.substring(0, Math.min(200, result.length())) + "...");
        }
    }
}
