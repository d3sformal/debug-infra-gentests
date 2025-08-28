package cz.cuni.mff.d3s.autodebugger.intellijplugin.actions;

import com.intellij.icons.AllIcons;
import com.intellij.openapi.actionSystem.AnAction;
import com.intellij.openapi.actionSystem.AnActionEvent;
import com.intellij.openapi.project.Project;
import cz.cuni.mff.d3s.autodebugger.intellijplugin.dialogs.SettingsDialog;
import org.jetbrains.annotations.NotNull;

/**
 * Action to open the Auto-Debugger settings dialog.
 * This action is displayed in the tool window header dropdown menu.
 */
public class OpenSettingsAction extends AnAction {

    public OpenSettingsAction() {
        super("Settings...", "Open Auto-Debugger settings", AllIcons.General.Settings);
    }

    @Override
    public void actionPerformed(@NotNull AnActionEvent e) {
        Project project = e.getProject();
        if (project != null) {
            SettingsDialog dialog = new SettingsDialog(project);
            dialog.show();
        }
    }

    @Override
    public void update(@NotNull AnActionEvent e) {
        // Enable the action only when a project is available
        e.getPresentation().setEnabled(e.getProject() != null);
    }
}
