package cz.cuni.mff.d3s.autodebugger.testrunner.common;

/**
 * Represents the overall status of a test suite execution.
 */
public enum TestSuiteStatus {
    /**
     * All tests in the suite passed successfully.
     */
    PASSED,
    
    /**
     * One or more tests in the suite failed.
     */
    FAILED,
    
    /**
     * The test suite execution was skipped.
     */
    SKIPPED,
    
    /**
     * The test suite execution encountered an error before completion.
     */
    ERROR,
    
    /**
     * The test suite is currently running.
     */
    RUNNING,
    
    /**
     * The test suite execution was aborted or cancelled.
     */
    ABORTED
}
