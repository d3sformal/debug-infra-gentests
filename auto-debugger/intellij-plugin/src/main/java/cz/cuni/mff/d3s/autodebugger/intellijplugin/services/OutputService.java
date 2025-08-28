package cz.cuni.mff.d3s.autodebugger.intellijplugin.services;

import com.intellij.execution.ui.ConsoleView;
import com.intellij.execution.ui.ConsoleViewContentType;
import com.intellij.openapi.components.Service;
import com.intellij.openapi.project.Project;
import com.intellij.openapi.wm.ToolWindow;
import com.intellij.openapi.wm.ToolWindowManager;
import com.intellij.ui.content.Content;
import com.intellij.ui.content.ContentFactory;
import com.intellij.execution.impl.ConsoleViewImpl;
import com.intellij.openapi.diagnostic.Logger;
import org.jetbrains.annotations.NotNull;

import java.util.HashMap;
import java.util.Map;

/**
 * Service for managing output in IntelliJ's output window.
 * Creates separate tabs for different types of output: Tool Output, Analysis Run, and Tests.
 */
@Service(Service.Level.PROJECT)
public final class OutputService {
    private static final Logger LOG = Logger.getInstance(OutputService.class);
    
    private static final String OUTPUT_TOOL_WINDOW_ID = "Output";
    
    // Output tab types
    public enum OutputType {
        TOOL_OUTPUT("Auto-Debugger"),
        ANALYSIS_RUN("Analysis Run"),
        TESTS("Tests");

        private final String displayName;

        OutputType(String displayName) {
            this.displayName = displayName;
        }

        public String getDisplayName() {
            return displayName;
        }
    }
    
    private final Project project;
    private final Map<OutputType, ConsoleView> consoles = new HashMap<>();
    private final Map<OutputType, Content> contents = new HashMap<>();
    
    public OutputService(Project project) {
        this.project = project;
    }
    
    /**
     * Gets the output service for the specified project.
     *
     * @param project The project
     * @return The output service
     */
    public static OutputService getInstance(@NotNull Project project) {
        return project.getService(OutputService.class);
    }
    
    /**
     * Prints a message to the specified output tab.
     *
     * @param outputType The type of output tab
     * @param message The message to print
     */
    public void print(OutputType outputType, String message) {
        print(outputType, message, ConsoleViewContentType.NORMAL_OUTPUT);
    }
    
    /**
     * Prints an error message to the specified output tab.
     *
     * @param outputType The type of output tab
     * @param message The error message to print
     */
    public void printError(OutputType outputType, String message) {
        print(outputType, message, ConsoleViewContentType.ERROR_OUTPUT);
    }
    
    /**
     * Prints a system message to the specified output tab.
     *
     * @param outputType The type of output tab
     * @param message The system message to print
     */
    public void printSystem(OutputType outputType, String message) {
        print(outputType, message, ConsoleViewContentType.SYSTEM_OUTPUT);
    }
    
    /**
     * Prints a message to the specified output tab with the given content type.
     *
     * @param outputType The type of output tab
     * @param message The message to print
     * @param contentType The content type for styling
     */
    public void print(OutputType outputType, String message, ConsoleViewContentType contentType) {
        ConsoleView console = getOrCreateConsole(outputType);
        console.print(message + "\n", contentType);
        
        // Activate the output window and select the appropriate tab
        activateOutputTab(outputType);
    }
    
    /**
     * Clears the specified output tab.
     *
     * @param outputType The type of output tab to clear
     */
    public void clear(OutputType outputType) {
        ConsoleView console = getOrCreateConsole(outputType);
        console.clear();
    }
    
    /**
     * Gets or creates a console for the specified output type.
     *
     * @param outputType The output type
     * @return The console view
     */
    private ConsoleView getOrCreateConsole(OutputType outputType) {
        ConsoleView console = consoles.get(outputType);
        if (console == null) {
            console = createConsole(outputType);
            consoles.put(outputType, console);
        }
        return console;
    }
    
    /**
     * Creates a new console for the specified output type.
     *
     * @param outputType The output type
     * @return The created console view
     */
    private ConsoleView createConsole(OutputType outputType) {
        ConsoleView console = new ConsoleViewImpl(project, false);
        
        // Create content for the output window
        Content content = ContentFactory.getInstance().createContent(
                console.getComponent(), 
                outputType.getDisplayName(), 
                false
        );
        content.setCloseable(false);
        contents.put(outputType, content);
        
        // Add to output tool window
        ToolWindow outputToolWindow = ToolWindowManager.getInstance(project).getToolWindow(OUTPUT_TOOL_WINDOW_ID);
        if (outputToolWindow != null) {
            outputToolWindow.getContentManager().addContent(content);
        } else {
            LOG.warn("Output tool window not found");
        }
        
        return console;
    }
    
    /**
     * Activates the output window and selects the specified tab.
     *
     * @param outputType The output type to activate
     */
    private void activateOutputTab(OutputType outputType) {
        ToolWindow outputToolWindow = ToolWindowManager.getInstance(project).getToolWindow(OUTPUT_TOOL_WINDOW_ID);
        if (outputToolWindow != null) {
            outputToolWindow.activate(() -> {
                Content content = contents.get(outputType);
                if (content != null) {
                    outputToolWindow.getContentManager().setSelectedContent(content);
                }
            });
        }
    }
    
    /**
     * Disposes all consoles and cleans up resources.
     */
    public void dispose() {
        for (ConsoleView console : consoles.values()) {
            console.dispose();
        }
        consoles.clear();
        contents.clear();
    }
}
