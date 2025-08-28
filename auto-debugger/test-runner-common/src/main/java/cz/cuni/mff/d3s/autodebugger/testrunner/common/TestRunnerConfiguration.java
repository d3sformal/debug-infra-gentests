package cz.cuni.mff.d3s.autodebugger.testrunner.common;

import lombok.Builder;
import lombok.Getter;
import lombok.Singular;

import java.nio.file.Path;
import java.time.Duration;
import java.util.List;
import java.util.Map;

/**
 * Configuration for test execution, including environment settings,
 * classpath, and execution parameters.
 */
@Builder
@Getter
public class TestRunnerConfiguration {
    
    /**
     * Working directory for test execution.
     */
    private final Path workingDirectory;
    
    /**
     * Classpath entries for test execution.
     */
    @Singular
    private final List<Path> classpathEntries;
    
    /**
     * Environment variables for test execution.
     */
    @Singular
    private final Map<String, String> environmentVariables;
    
    /**
     * JVM arguments for test execution.
     */
    @Singular
    private final List<String> jvmArguments;
    
    /**
     * Maximum time to wait for test execution to complete.
     */
    private final Duration executionTimeout;
    
    /**
     * Whether to enable instrumentation during test execution.
     */
    @Builder.Default
    private final boolean enableInstrumentation = false;
    
    /**
     * Path to the instrumentation JAR file (if instrumentation is enabled).
     */
    private final Path instrumentationJarPath;
    
    /**
     * Whether to capture detailed execution traces.
     */
    @Builder.Default
    private final boolean captureExecutionTraces = true;
    
    /**
     * Whether to run tests in parallel.
     */
    @Builder.Default
    private final boolean parallelExecution = false;
    
    /**
     * Number of parallel threads for test execution (if parallel execution is enabled).
     */
    @Builder.Default
    private final int parallelThreads = Runtime.getRuntime().availableProcessors();
    
    /**
     * Whether to fail fast on the first test failure.
     */
    @Builder.Default
    private final boolean failFast = false;
    
    /**
     * Additional system properties for test execution.
     */
    @Singular
    private final Map<String, String> systemProperties;
    
    /**
     * Test framework to use (e.g., "junit5", "junit4", "testng").
     */
    @Builder.Default
    private final String testFramework = "junit5";
    
    /**
     * Whether to generate detailed test reports.
     */
    @Builder.Default
    private final boolean generateReports = true;
    
    /**
     * Directory where test reports should be generated.
     */
    private final Path reportOutputDirectory;
}
