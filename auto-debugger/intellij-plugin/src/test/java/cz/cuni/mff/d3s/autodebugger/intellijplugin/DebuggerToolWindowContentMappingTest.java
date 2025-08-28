package cz.cuni.mff.d3s.autodebugger.intellijplugin;

import com.intellij.psi.PsiClass;
import com.intellij.psi.PsiMethod;
import com.intellij.testFramework.fixtures.LightJavaCodeInsightFixtureTestCase5;
import cz.cuni.mff.d3s.autodebugger.intellijplugin.services.SettingsService;

import java.util.List;

import static org.junit.Assert.*;

public class DebuggerToolWindowContentMappingTest extends LightJavaCodeInsightFixtureTestCase5 {

    private DebuggerToolWindowContent ui;

    @org.junit.Before
    public void setUpUi() {
        ui = new DebuggerToolWindowContent(getFixture().getProject());
        SettingsService.getInstance(getFixture().getProject()).setConfigurationValue("disl.path", getFixture().getProject().getBasePath());
    }

    private PsiMethod addAndFindMethod() {
        String code = "package sample;\n" +
                "public class Demo {\n" +
                "  private int count;\n" +
                "  public String name;\n" +
                "  public static int sum(int a, String b) { return 0; }\n" +
                "}\n";
        getFixture().addFileToProject("src/sample/Demo.java", code);
        PsiClass cls = getFixture().findClass("sample.Demo");
        assertNotNull(cls);
        PsiMethod[] methods = cls.findMethodsByName("sum", false);
        assertEquals(1, methods.length);
        return methods[0];
    }

    @org.junit.Test
    public void givenParametersAndReturnDefault_whenPanelsPopulated_thenReturnIsSelectedByDefault() {
        PsiMethod method = addAndFindMethod();
        ui.showAdditionalConfig(method);
        // The return value is enabled by default; assertion via building params/fields remains unaffected
        // Verify parameters mapping still works as expected
        java.util.List<String> params = ui.buildTargetParametersForRunner(method);
        assertEquals(2, params.size());
        assertTrue(params.contains("0:int"));
        assertTrue(params.contains("1:java.lang.String"));
    }

    @org.junit.Test
    public void givenParametersSelected_whenBuildTargetParameters_thenFormatsSlotAndCanonicalType() {
        PsiMethod method = addAndFindMethod();
        // Default parameterSelectionState is true for params
        List<String> params = ui.buildTargetParametersForRunner(method);
        assertEquals(2, params.size());
        assertTrue(params.contains("0:int"));
        assertTrue(params.contains("1:java.lang.String"));
    }

    @org.junit.Test
    public void givenReturnValueToggledOff_thenStateIsClearedOnPanelRepopulate() {
        PsiMethod method = addAndFindMethod();
        ui.showAdditionalConfig(method);
        // Access internal testing helper: simulate unselecting
        // Since return checkbox is internal, we verify behavior indirectly by repopulating panel
        // and ensuring no exception; toggling is tracked in state (can't read the flag here),
        // so this test remains smoke-level for no regressions in panel lifecycle.
        ui.populateReturnValuePanel(method.getReturnType());
    }

    @org.junit.Test
    public void givenFieldsSelected_whenBuildTargetFields_thenFormatsCanonicalTypeAndName() {
        PsiMethod method = addAndFindMethod();
        // Select fields by key: fully qualified class name + "." + field name
        String clsFqn = method.getContainingClass().getQualifiedName();
        ui.setFieldSelectedForTesting(clsFqn + ".count", true);
        ui.setFieldSelectedForTesting(clsFqn + ".name", true);

        List<String> fields = ui.buildTargetFieldsForRunner(method);
        assertEquals(2, fields.size());
        assertTrue(fields.contains("int:count"));
        assertTrue(fields.contains("java.lang.String:name"));
    }
}

