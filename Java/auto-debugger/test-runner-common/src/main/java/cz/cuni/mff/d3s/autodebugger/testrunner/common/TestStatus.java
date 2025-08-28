package cz.cuni.mff.d3s.autodebugger.testrunner.common;

/**
 * Represents the execution status of an individual test.
 */
public enum TestStatus {
    /**
     * The test executed successfully and all assertions passed.
     */
    PASSED,
    
    /**
     * The test failed due to assertion failures or exceptions.
     */
    FAILED,
    
    /**
     * The test was skipped and not executed.
     */
    SKIPPED,
    
    /**
     * The test execution was aborted due to timeout or other issues.
     */
    ABORTED,
    
    /**
     * The test is currently running.
     */
    RUNNING
}
