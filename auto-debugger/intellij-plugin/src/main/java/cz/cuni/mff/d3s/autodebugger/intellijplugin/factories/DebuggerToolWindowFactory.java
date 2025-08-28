package cz.cuni.mff.d3s.autodebugger.intellijplugin.factories;

import com.intellij.openapi.actionSystem.ActionManager;
import com.intellij.openapi.actionSystem.ActionToolbar;
import com.intellij.openapi.actionSystem.DefaultActionGroup;
import com.intellij.openapi.project.DumbAware;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.wm.ToolWindow;
import com.intellij.openapi.wm.ToolWindowFactory;
import com.intellij.ui.content.Content;
import com.intellij.ui.content.ContentFactory;
import cz.cuni.mff.d3s.autodebugger.intellijplugin.DebuggerToolWindowContent;
import cz.cuni.mff.d3s.autodebugger.intellijplugin.actions.OpenSettingsAction;
import org.jetbrains.annotations.NotNull;

/**
 * Factory for creating the auto-debugger tool window in IntelliJ IDEA.
 * Implements ToolWindowFactory to integrate with IntelliJ's tool window system
 * and DumbAware to function during indexing operations.
 */
public final class DebuggerToolWindowFactory implements ToolWindowFactory, DumbAware {

    /**
     * Creates the tool window content and integrates it with IntelliJ's UI.
     * Sets up the main content panel and configures tool window actions.
     */
    @Override
    public void createToolWindowContent(@NotNull Project project, @NotNull ToolWindow toolWindow) {
        DebuggerToolWindowContent toolWindowContent = new DebuggerToolWindowContent(project);
        Content content = ContentFactory.getInstance().createContent(toolWindowContent.getContentPanel(), "", false);
        toolWindow.getContentManager().addContent(content);

        // Add settings action to the tool window header dropdown
        setupToolWindowActions(toolWindow);
    }

    private void setupToolWindowActions(@NotNull ToolWindow toolWindow) {
        // Create action group for the tool window header
        DefaultActionGroup actionGroup = new DefaultActionGroup();
        actionGroup.add(new OpenSettingsAction());

        // Create toolbar and add it to the tool window
        ActionToolbar toolbar = ActionManager.getInstance().createActionToolbar(
                "AutoDebuggerToolWindow",
                actionGroup,
                false
        );
        toolbar.setTargetComponent(null);

        // Set the action group for the tool window header dropdown
        toolWindow.setAdditionalGearActions(actionGroup);
    }
}
