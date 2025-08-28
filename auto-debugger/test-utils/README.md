# test-utilsdd 

Utilities for simplifying tests in the auto-debugger project.

## StubResultsHelper

Prefer configuring JavaRunConfiguration.outputDirectory directly in your tests. When you need to simulate analyzer output (without running DiSL), create a minimal stub test file and results list using StubResultsHelper.

```java
import cz.cuni.mff.d3s.autodebugger.testutils.StubResultsHelper;

Path outputDir = /* your orchestrator or runConfiguration output dir */;
Path stubTest = StubResultsHelper.writeMinimalStubTestAndResults(outputDir);
// Now you can call orchestrator.runAnalysis(...) and expect a non-empty result
```

Notes:
- It writes a basic JUnit 5 test file under `<outputDir>/stub-tests/StubTest.java` and records the absolute path into the results list file (`generated-tests-<runId>.lst` if present, otherwise `generated-tests.lst`).
- No system properties or environment variables are required.

