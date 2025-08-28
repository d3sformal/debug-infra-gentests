package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

/**
 * Strategies for naming generated test methods.
 */
public enum TestNamingStrategy {
    /**
     * Simple naming: test1, test2, etc.
     */
    SIMPLE,
    
    /**
     * Descriptive naming based on input parameters and expected outcomes.
     * Example: testDivide_WithPositiveNumbers_ReturnsCorrectResult
     */
    DESCRIPTIVE,
    
    /**
     * BDD-style naming using given/when/then format.
     * Example: givenPositiveNumbers_whenDividing_thenReturnsCorrectResult
     */
    BDD_STYLE,
    
    /**
     * Parameter-based naming including actual parameter values.
     * Example: testDivide_10_2_Returns5
     */
    PARAMETER_BASED,
    
    /**
     * Scenario-based naming describing the test scenario.
     * Example: testNormalDivisionScenario, testDivisionByZeroScenario
     */
    SCENARIO_BASED
}
