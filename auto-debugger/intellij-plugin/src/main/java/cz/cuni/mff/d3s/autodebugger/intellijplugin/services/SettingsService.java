package cz.cuni.mff.d3s.autodebugger.intellijplugin.services;

import com.intellij.openapi.components.PersistentStateComponent;
import com.intellij.openapi.components.Service;
import com.intellij.openapi.components.State;
import com.intellij.openapi.components.Storage;
import com.intellij.openapi.project.Project;
import com.intellij.util.xmlb.XmlSerializerUtil;
import org.jetbrains.annotations.NotNull;
import org.jetbrains.annotations.Nullable;

import java.util.HashMap;
import java.util.Map;

/**
 * Service for managing auto-debugger settings.
 * This service persists configuration values across IDE sessions.
 */
@Service(Service.Level.PROJECT)
@State(
    name = "AutoDebuggerSettings",
    storages = @Storage("autoDebuggerSettings.xml")
)
public final class SettingsService implements PersistentStateComponent<SettingsService.State> {
    private final Project project;
    private State state = new State();

    /**
     * Creates a new settings service for the specified project.
     *
     * @param project The project
     */
    public SettingsService(Project project) {
        this.project = project;
    }

    /**
     * Gets the settings service for the specified project.
     *
     * @param project The project
     * @return The settings service
     */
    public static SettingsService getInstance(@NotNull Project project) {
        return project.getService(SettingsService.class);
    }

    /**
     * Gets a configuration value by key.
     *
     * @param key The configuration key
     * @return The configuration value, or null if not set
     */
    @Nullable
    public String getConfigurationValue(String key) {
        return state.configurationValues.get(key);
    }

    /**
     * Sets a configuration value.
     *
     * @param key The configuration key
     * @param value The configuration value
     */
    public void setConfigurationValue(String key, String value) {
        state.configurationValues.put(key, value);
    }

    /**
     * Gets all configuration values.
     *
     * @return Map of all configuration key-value pairs
     */
    public Map<String, String> getAllConfigurationValues() {
        return new HashMap<>(state.configurationValues);
    }

    /**
     * Sets multiple configuration values at once.
     *
     * @param values Map of configuration key-value pairs
     */
    public void setConfigurationValues(Map<String, String> values) {
        state.configurationValues.clear();
        state.configurationValues.putAll(values);
    }

    /**
     * Checks if a configuration value is set.
     *
     * @param key The configuration key
     * @return true if the value is set and not empty, false otherwise
     */
    public boolean isConfigurationSet(String key) {
        String value = getConfigurationValue(key);
        return value != null && !value.trim().isEmpty();
    }

    @Override
    public @Nullable State getState() {
        return state;
    }

    @Override
    public void loadState(@NotNull State state) {
        XmlSerializerUtil.copyBean(state, this.state);
    }

    /**
     * State class for persisting settings.
     */
    public static class State {
        public Map<String, String> configurationValues = new HashMap<>();
    }
}
