package cz.cuni.mff.d3s.autodebugger.runner.strategies;

import lombok.Getter;

@Getter
public enum TestGenerationStrategyName {
    TRACE_BASED_NAIVE("Trace-based naive", "Generates unit tests based on runtime traces. " +
            "This strategy creates straightforward test cases that replicate the observed behavior."),
    LLM_BASED("LLM-based", "Leverages large language models and AI techniques to generate human-readable, maintainable tests. " +
            "Combines runtime observations with static context in form of the provided code.");

    private final String displayName;
    private final String description;

    TestGenerationStrategyName(String displayName, String description) {
        this.displayName = displayName;
        this.description = description;
    }
}
