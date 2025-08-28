package cz.cuni.mff.d3s.autodebugger.testrunner.common;

import cz.cuni.mff.d3s.autodebugger.model.common.trace.Trace;
import lombok.Builder;
import lombok.Getter;

import java.time.LocalDateTime;
import java.util.Map;

/**
 * Represents an execution trace captured during test execution.
 * Contains runtime data collected through instrumentation.
 */
@Builder
@Getter
public class ExecutionTrace {
    
    /**
     * Name of the test that generated this trace.
     */
    private final String testName;
    
    /**
     * Fully qualified class name of the test.
     */
    private final String testClassName;
    
    /**
     * The actual trace data collected during execution.
     */
    private final Trace traceData;
    
    /**
     * Timestamp when the trace collection started.
     */
    private final LocalDateTime startTime;
    
    /**
     * Timestamp when the trace collection ended.
     */
    private final LocalDateTime endTime;
    
    /**
     * Method that was being traced.
     */
    private final String tracedMethod;
    
    /**
     * Additional metadata about the trace collection.
     */
    private final Map<String, Object> metadata;
    
    /**
     * Whether the trace collection completed successfully.
     */
    private final boolean successful;
    
    /**
     * Error message if trace collection failed.
     */
    private final String errorMessage;
    
    /**
     * Gets the full test identifier.
     */
    public String getFullTestName() {
        return testClassName + "." + testName;
    }
    
    /**
     * Checks if the trace contains any collected data.
     */
    public boolean hasTraceData() {
        return traceData != null && successful;
    }
}
