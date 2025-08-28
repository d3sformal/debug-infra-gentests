package cz.cuni.mff.d3s.autodebugger.intellijplugin;

import com.intellij.execution.jar.JarApplicationConfiguration;
import com.intellij.testFramework.fixtures.LightJavaCodeInsightFixtureTestCase5;
import cz.cuni.mff.d3s.autodebugger.runner.strategies.TestGenerationStrategy;
import cz.cuni.mff.d3s.autodebugger.runner.strategies.TestGenerationStrategyName;

import static org.junit.Assert.*;
import static org.mockito.Mockito.*;

public class JarRunConfigArgumentsExtractionTest extends LightJavaCodeInsightFixtureTestCase5 {

    @org.junit.Test
    public void givenJarRunConfig_whenBuildingArguments_thenClasspathAndArgsExtracted() {
        var project = getFixture().getProject();
        var ui = new DebuggerToolWindowContent(project);

        JarApplicationConfiguration jarCfg = mock(JarApplicationConfiguration.class);
        when(jarCfg.getJarPath()).thenReturn("/path/to/app.jar");
        when(jarCfg.getProgramParameters()).thenReturn("-x 1 --flag");
        when(jarCfg.getWorkingDirectory()).thenReturn(project.getBasePath());

        ui.setSelectedIntellijRunConfigForTesting(jarCfg);

        TestGenerationStrategy strategy = new TestGenerationStrategy(TestGenerationStrategyName.TRACE_BASED_NAIVE, true);
        var args = ui.buildRunnerArgumentsForTesting("/path/to/app.jar",
                java.util.Arrays.asList("-x","1","--flag"),
                "sample.Demo.sum(int,java.lang.String)", strategy);

        assertNotNull(args);
        assertEquals("/path/to/app.jar", args.applicationJarPath);
        assertTrue(args.classpath != null && args.classpath.contains("/path/to/app.jar"));
        assertEquals(java.util.Arrays.asList("-x","1","--flag"), args.runtimeArguments);
    }
}

