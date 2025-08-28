package cz.cuni.mff.d3s.autodebugger.intellijplugin.utils;

import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import cz.cuni.mff.d3s.autodebugger.runner.args.Arguments;
import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

public class RunnerArgumentsBuilderTest {

    @Test
    void givenMinimalInputs_whenBuildingJavaArguments_thenFieldsAreMapped() {
        Arguments a = RunnerArgumentsBuilder.forJava(
                "/app.jar",
                "/src",
                "/disl",
                "/out",
                "com.acme.Main.run(int,String)",
                List.of("0:int", "1:java.lang.String"),
                List.of("int:counter"),
                List.of("--flag"),
                List.of("/lib/a.jar"),
                "trace-based-basic",
                "KEY"
        );

        assertEquals("/app.jar", a.applicationJarPath);
        assertEquals("/src", a.sourceCodePath);
        assertEquals("/disl", a.dislHomePath);
        assertEquals("/out", a.outputDirectory);
        assertEquals("com.acme.Main.run(int,String)", a.targetMethodReference);
        assertEquals(List.of("0:int", "1:java.lang.String"), a.targetParameters);
        assertEquals(List.of("int:counter"), a.targetFields);
        assertEquals(List.of("--flag"), a.runtimeArguments);
        assertEquals(List.of("/lib/a.jar"), a.classpath);
        assertEquals("trace-based-basic", a.testGenerationStrategy);
        assertEquals("KEY", a.apiKey);
        assertNotNull(a.language);
        assertEquals(TargetLanguage.JAVA, a.language);
    }
}

