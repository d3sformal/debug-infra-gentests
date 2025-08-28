package cz.cuni.mff.d3s.autodebugger.testrunner.common;

import java.nio.file.Path;
import java.util.List;

/**
 * Abstract interface for executing test suites and collecting results.
 * Implementations should handle the execution of generated tests and provide
 * detailed execution traces for debugging purposes.
 */
public interface TestRunner {


    /**
     * Executes a test suite from the given test files.
     *
     * @param testFiles List of paths to test files to execute
     * @return TestExecutionResult containing the results and traces
     */
    TestExecutionResult executeTests(List<Path> testFiles);

    /**
     * Executes a single test file.
     *
     * @param testFile Path to the test file to execute
     * @return TestExecutionResult containing the results and traces
     */
    TestExecutionResult executeTest(Path testFile);

    /**
     * Configures the test execution environment.
     * 
     * @param config TestRunnerConfiguration containing execution parameters
     */
    void configure(TestRunnerConfiguration config);
}
