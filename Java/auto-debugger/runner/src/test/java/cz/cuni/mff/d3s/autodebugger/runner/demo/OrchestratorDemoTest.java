package cz.cuni.mff.d3s.autodebugger.runner.demo;

import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class OrchestratorDemoTest {
    
    @Test
    void givenOrchestratorDemo_whenDemonstrateAPI_thenRunsWithoutException() {
        // This test verifies that the orchestrator API demonstration runs without exceptions
        assertDoesNotThrow(() -> {
            OrchestratorDemo.demonstrateOrchestratorAPI();
        });
    }
    
    @Test
    void givenOrchestratorDemo_whenDemonstrateMainWorkflow_thenRunsWithoutException() {
        // This test verifies that the main workflow demonstration runs without exceptions
        assertDoesNotThrow(() -> {
            OrchestratorDemo.demonstrateMainWorkflow();
        });
    }
}
