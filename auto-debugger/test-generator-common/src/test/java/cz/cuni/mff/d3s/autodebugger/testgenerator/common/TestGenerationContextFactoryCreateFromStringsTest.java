package cz.cuni.mff.d3s.autodebugger.testgenerator.common;

import static org.junit.jupiter.api.Assertions.*;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.nio.file.Path;

class TestGenerationContextFactoryCreateFromStringsTest {

    @TempDir
    Path tempDir;

    @Test
    void givenStrings_whenCreateFromStrings_withEmptyPackage_thenBuildsDefaultClassNameAndSignature() {
        var ctx = TestGenerationContextFactory.createFromStrings(
            "", "Calculator", "divide(int, int)", "int", tempDir);
        assertNotNull(ctx);
        assertEquals("Calculator", ctx.getTargetClassName());
        assertEquals("Calculator.divide(int, int)", ctx.getTargetMethodSignature());
        assertEquals("", ctx.getPackageName());
    }

    @Test
    void givenStrings_whenCreateFromStrings_withNoParams_thenParsesMethodNameOnly() {
        var ctx = TestGenerationContextFactory.createFromStrings(
            "com.example", "Util", "now()", "long", tempDir);
        assertNotNull(ctx);
        assertEquals("com.example.Util", ctx.getTargetMethod().getFullyQualifiedClassName());
        assertEquals("com.example", ctx.getPackageName());
        assertEquals("com.example.Util.now()", ctx.getTargetMethodSignature());
    }

    @Test
    void givenStrings_whenCreateFromStrings_withSpacing_thenTrimsParameterTypes() {
        var ctx = TestGenerationContextFactory.createFromStrings(
            "a.b", "C", "m( int ,  java.lang.String  ,double)", "void", tempDir);
        assertNotNull(ctx);
        assertEquals("a.b.C.m(int, java.lang.String, double)", ctx.getTargetMethodSignature());
    }
}

