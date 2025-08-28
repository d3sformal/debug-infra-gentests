package cz.cuni.mff.d3s.autodebugger.runner.factories;

import cz.cuni.mff.d3s.autodebugger.model.common.RunConfiguration;
import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.model.java.JavaRunConfiguration;
import cz.cuni.mff.d3s.autodebugger.testrunner.common.TestRunner;
import cz.cuni.mff.d3s.autodebugger.testrunner.java.JUnitTestRunner;
import cz.cuni.mff.d3s.autodebugger.testrunner.common.TestRunnerConfiguration;
import lombok.extern.slf4j.Slf4j;

@Slf4j
public class TestRunnerFactory {

    public static TestRunner createTestRunner(RunConfiguration runConfiguration) {
        TargetLanguage language = runConfiguration.getLanguage();
        if (language == TargetLanguage.JAVA) {
            return createJavaTestRunner(runConfiguration);
        }

        throw new IllegalArgumentException("Unsupported language: " + language);
    }

    private static TestRunner createJavaTestRunner(RunConfiguration runConfiguration) {
        log.info("Creating Java test runner");

        if (runConfiguration instanceof JavaRunConfiguration javaRunConfiguration) {
            try {
                JUnitTestRunner testRunner = new JUnitTestRunner();

                // Build classpath for test compilation and execution
                TestRunnerConfiguration.TestRunnerConfigurationBuilder configBuilder = TestRunnerConfiguration.builder()
                        .workingDirectory(javaRunConfiguration.getOutputDirectory());

                // Add application JAR to classpath so tests can reference application classes
                configBuilder.classpathEntry(javaRunConfiguration.getApplicationPath());
                log.info("Added application JAR to test classpath: {}", javaRunConfiguration.getApplicationPath());

                // Add additional classpath entries from the run configuration (skip empty entries)
                if (javaRunConfiguration.getClasspathEntries() != null) {
                    for (var classpathEntry : javaRunConfiguration.getClasspathEntries()) {
                        // Skip empty or whitespace-only classpath entries
                        if (classpathEntry != null && !classpathEntry.toString().trim().isEmpty()) {
                            configBuilder.classpathEntry(classpathEntry);
                            log.info("Added classpath entry to test classpath: {}", classpathEntry);
                        } else {
                            log.debug("Skipped empty classpath entry");
                        }
                    }
                }

                TestRunnerConfiguration config = configBuilder.build();
                testRunner.configure(config);
                log.info("Successfully created Java test runner with {} classpath entries",
                        config.getClasspathEntries().size());
                return testRunner;
            } catch (Exception e) {
                log.error("Failed to create Java test runner", e);
                throw new RuntimeException("Failed to create test runner", e);
            }
        }

        throw new IllegalArgumentException("Expected JavaRunConfiguration, got: " + runConfiguration.getClass().getSimpleName());
    }
}
