package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

import lombok.Builder;
import lombok.Getter;

/**
 * Statistics about the test generation process.
 */
@Builder
@Getter
public class TestGenerationStatistics {
    
    /**
     * Total number of test methods generated.
     */
    private final int totalTestMethods;
    
    /**
     * Number of positive test cases (testing normal behavior).
     */
    private final int positiveTestCases;
    
    /**
     * Number of negative test cases (testing error conditions).
     */
    private final int negativeTestCases;
    
    /**
     * Number of edge case tests.
     */
    private final int edgeCaseTests;
    
    /**
     * Number of parameterized tests.
     */
    private final int parameterizedTests;
    
    /**
     * Number of unique input combinations tested.
     */
    private final int uniqueInputCombinations;
    
    /**
     * Estimated code coverage percentage.
     */
    private final double estimatedCodeCoverage;
    
    /**
     * Number of assertions generated.
     */
    private final int totalAssertions;
    
    /**
     * Number of setup/teardown methods generated.
     */
    private final int setupTeardownMethods;
    
    /**
     * Average test method length (in lines of code).
     */
    private final double averageTestMethodLength;
    
    /**
     * Number of external dependencies mocked.
     */
    private final int mockedDependencies;
    
    /**
     * Complexity score of generated tests (0-100).
     */
    private final int complexityScore;
    
    /**
     * Gets the ratio of positive to negative test cases.
     */
    public double getPositiveToNegativeRatio() {
        return negativeTestCases > 0 ? (double) positiveTestCases / negativeTestCases : Double.POSITIVE_INFINITY;
    }
    
    /**
     * Gets the percentage of edge case tests.
     */
    public double getEdgeCasePercentage() {
        return totalTestMethods > 0 ? (double) edgeCaseTests / totalTestMethods * 100.0 : 0.0;
    }
    
    /**
     * Gets the average number of assertions per test method.
     */
    public double getAverageAssertionsPerTest() {
        return totalTestMethods > 0 ? (double) totalAssertions / totalTestMethods : 0.0;
    }
}
