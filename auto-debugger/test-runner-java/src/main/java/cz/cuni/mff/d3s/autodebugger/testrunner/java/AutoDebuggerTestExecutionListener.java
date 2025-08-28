package cz.cuni.mff.d3s.autodebugger.testrunner.java;

import cz.cuni.mff.d3s.autodebugger.testrunner.common.TestResult;
import cz.cuni.mff.d3s.autodebugger.testrunner.common.TestStatus;
import lombok.Getter;
import lombok.extern.slf4j.Slf4j;
import org.junit.platform.engine.TestExecutionResult;
import org.junit.platform.launcher.TestIdentifier;

import java.time.Duration;
import java.time.Instant;
import java.util.ArrayList;
import java.util.List;
import java.util.Map;
import java.util.concurrent.ConcurrentHashMap;

/**
 * JUnit Platform test execution listener that captures test results
 * and converts them to our internal TestResult format.
 */
@Slf4j
public class AutoDebuggerTestExecutionListener implements org.junit.platform.launcher.TestExecutionListener {
    
    @Getter
    private final List<TestResult> testResults = new ArrayList<>();
    
    private final Map<String, Instant> testStartTimes = new ConcurrentHashMap<>();
    
    @Override
    public void executionStarted(TestIdentifier testIdentifier) {
        if (testIdentifier.isTest()) {
            testStartTimes.put(testIdentifier.getUniqueId(), Instant.now());
            log.debug("Test started: {}", testIdentifier.getDisplayName());
        }
    }
    
    @Override
    public void executionFinished(TestIdentifier testIdentifier, TestExecutionResult executionResult) {
        if (testIdentifier.isTest()) {
            Instant startTime = testStartTimes.get(testIdentifier.getUniqueId());
            Duration executionTime = startTime != null ? 
                Duration.between(startTime, Instant.now()) : Duration.ZERO;
            
            TestResult testResult = convertToTestResult(testIdentifier, executionResult, executionTime);
            testResults.add(testResult);
            
            log.debug("Test finished: {} - Status: {}", 
                testIdentifier.getDisplayName(), testResult.getStatus());
        }
    }
    
    @Override
    public void executionSkipped(TestIdentifier testIdentifier, String reason) {
        if (testIdentifier.isTest()) {
            TestResult testResult = TestResult.builder()
                    .testName(extractTestMethodName(testIdentifier))
                    .testClassName(extractTestClassName(testIdentifier))
                    .status(TestStatus.SKIPPED)
                    .executionTime(Duration.ZERO)
                    .details("Skipped: " + reason)
                    .build();
            
            testResults.add(testResult);
            log.debug("Test skipped: {} - Reason: {}", testIdentifier.getDisplayName(), reason);
        }
    }
    
    private TestResult convertToTestResult(TestIdentifier testIdentifier, 
                                         TestExecutionResult executionResult, 
                                         Duration executionTime) {
        
        TestResult.TestResultBuilder builder = TestResult.builder()
                .testName(extractTestMethodName(testIdentifier))
                .testClassName(extractTestClassName(testIdentifier))
                .executionTime(executionTime);
        
        switch (executionResult.getStatus()) {
            case SUCCESSFUL:
                builder.status(TestStatus.PASSED);
                break;
                
            case FAILED:
                builder.status(TestStatus.FAILED);
                
                if (executionResult.getThrowable().isPresent()) {
                    Throwable throwable = executionResult.getThrowable().get();
                    builder.errorMessage(throwable.getMessage());
                    builder.stackTrace(getStackTraceAsString(throwable));
                    
                    // Extract expected/actual values for assertion failures
                    extractAssertionDetails(throwable, builder);
                }
                break;
                
            case ABORTED:
                builder.status(TestStatus.ABORTED);
                if (executionResult.getThrowable().isPresent()) {
                    Throwable throwable = executionResult.getThrowable().get();
                    builder.errorMessage("Test aborted: " + throwable.getMessage());
                    builder.stackTrace(getStackTraceAsString(throwable));
                }
                break;
        }
        
        return builder.build();
    }
    
    private String extractTestMethodName(TestIdentifier testIdentifier) {
        String displayName = testIdentifier.getDisplayName();
        
        // Handle parameterized tests
        if (displayName.contains("(")) {
            displayName = displayName.substring(0, displayName.indexOf("("));
        }
        
        return displayName;
    }
    
    private String extractTestClassName(TestIdentifier testIdentifier) {
        // Navigate up the hierarchy to find the class
        TestIdentifier current = testIdentifier;
        while (current.getParentId().isPresent()) {
            current = testIdentifier; // This would need the full test plan to navigate properly
            if (current.getType().isContainer() && current.getSource().isPresent()) {
                String source = current.getSource().get().toString();
                if (source.contains("ClassSource")) {
                    // Extract class name from source
                    return extractClassNameFromSource(source);
                }
            }
            break; // Simplified for now
        }
        
        // Fallback: extract from unique ID
        String uniqueId = testIdentifier.getUniqueId();
        if (uniqueId.contains("[class:")) {
            int start = uniqueId.indexOf("[class:") + 7;
            int end = uniqueId.indexOf("]", start);
            if (end > start) {
                return uniqueId.substring(start, end);
            }
        }
        
        return "UnknownClass";
    }
    
    private String extractClassNameFromSource(String source) {
        // Extract class name from ClassSource string representation
        // This is a simplified implementation
        if (source.contains("class:")) {
            int start = source.indexOf("class:") + 6;
            int end = source.indexOf(")", start);
            if (end > start) {
                return source.substring(start, end).trim();
            }
        }
        return "UnknownClass";
    }
    
    private String getStackTraceAsString(Throwable throwable) {
        StringBuilder sb = new StringBuilder();
        sb.append(throwable.getClass().getName()).append(": ").append(throwable.getMessage()).append("\n");
        
        for (StackTraceElement element : throwable.getStackTrace()) {
            sb.append("\tat ").append(element.toString()).append("\n");
        }
        
        return sb.toString();
    }
    
    private void extractAssertionDetails(Throwable throwable, TestResult.TestResultBuilder builder) {
        String message = throwable.getMessage();
        if (message != null) {
            // Try to extract expected and actual values from assertion failure messages
            if (message.contains("expected:") && message.contains("but was:")) {
                String[] parts = message.split("expected:|but was:");
                if (parts.length >= 3) {
                    String expected = parts[1].trim();
                    String actual = parts[2].trim();
                    
                    // Clean up the values (remove brackets, etc.)
                    expected = cleanAssertionValue(expected);
                    actual = cleanAssertionValue(actual);
                    
                    builder.expectedValue(expected);
                    builder.actualValue(actual);
                }
            }
        }
    }
    
    private String cleanAssertionValue(String value) {
        if (value.startsWith("<") && value.endsWith(">")) {
            value = value.substring(1, value.length() - 1);
        }
        return value.trim();
    }
}
