package cz.cuni.mff.d3s.autodebugger.testrunner.java;

import cz.cuni.mff.d3s.autodebugger.testrunner.common.TestRunnerConfiguration;
import lombok.Getter;
import lombok.extern.slf4j.Slf4j;

import java.nio.file.Files;
import java.nio.file.Path;

/**
 * Manages instrumentation setup for test execution.
 * Handles DiSL agent configuration and trace collection setup.
 */
@Slf4j
public class InstrumentationManager {
    
    private boolean instrumentationSetup = false;

    @Getter
    private Path instrumentationJarPath;
    
    /**
     * Sets up instrumentation for test execution based on the configuration.
     * 
     * @param configuration Test runner configuration containing instrumentation settings
     */
    public void setupInstrumentation(TestRunnerConfiguration configuration) {
        log.info("Setting up instrumentation for test execution");
        
        if (!configuration.isEnableInstrumentation()) {
            log.debug("Instrumentation is disabled in configuration");
            return;
        }
        
        instrumentationJarPath = configuration.getInstrumentationJarPath();
        
        if (instrumentationJarPath == null) {
            log.warn("Instrumentation enabled but no instrumentation JAR path provided");
            return;
        }
        
        if (!Files.exists(instrumentationJarPath)) {
            log.error("Instrumentation JAR not found: {}", instrumentationJarPath);
            throw new RuntimeException("Instrumentation JAR not found: " + instrumentationJarPath);
        }
        
        log.info("Instrumentation JAR found: {}", instrumentationJarPath);

        // TODO

        instrumentationSetup = true;
        log.info("Instrumentation setup completed");
    }
    
    /**
     * Checks if instrumentation has been properly set up.
     * 
     * @return true if instrumentation is ready, false otherwise
     */
    public boolean isInstrumentationReady() {
        return instrumentationSetup;
    }

    /**
     * Cleans up instrumentation resources.
     */
    public void cleanup() {
        log.info("Cleaning up instrumentation resources");
        instrumentationSetup = false;
        instrumentationJarPath = null;
    }
}
