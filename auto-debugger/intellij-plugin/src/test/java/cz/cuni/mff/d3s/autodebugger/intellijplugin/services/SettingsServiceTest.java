package cz.cuni.mff.d3s.autodebugger.intellijplugin.services;

import com.intellij.testFramework.fixtures.LightJavaCodeInsightFixtureTestCase5;

import java.util.HashMap;
import java.util.Map;

import static org.junit.Assert.*;

/**
 * Test class for SettingsService functionality.
 */
public class SettingsServiceTest extends LightJavaCodeInsightFixtureTestCase5 {

    private SettingsService settingsService;

    @org.junit.Before
    public void setUp() {
        settingsService = SettingsService.getInstance(getFixture().getProject());
    }

    @org.junit.Test
    public void givenSettingsService_whenPerformingBasicOperations_thenWorksCorrectly() {
        
        // Test setting and getting a single value
        settingsService.setConfigurationValue("test.key", "test.value");
        assertEquals("test.value", settingsService.getConfigurationValue("test.key"));
        
        // Test checking if configuration is set
        assertTrue(settingsService.isConfigurationSet("test.key"));
        assertFalse(settingsService.isConfigurationSet("nonexistent.key"));
        
        // Test setting multiple values
        Map<String, String> values = new HashMap<>();
        values.put("disl.path", "/path/to/disl");
        values.put("anthropic.key", "secret-key");

        settingsService.setConfigurationValues(values);

        assertEquals("/path/to/disl", settingsService.getConfigurationValue("disl.path"));
        assertEquals("secret-key", settingsService.getConfigurationValue("anthropic.key"));

        // Test getting all values
        Map<String, String> allValues = settingsService.getAllConfigurationValues();
        assertTrue(allValues.containsKey("disl.path"));
        assertTrue(allValues.containsKey("anthropic.key"));
        assertEquals("/path/to/disl", allValues.get("disl.path"));
        assertEquals("secret-key", allValues.get("anthropic.key"));
    }

    @org.junit.Test
    public void givenEmptyAndNullValues_whenStoringAndRetrieving_thenHandlesCorrectly() {
        
        // Test null value
        assertNull(settingsService.getConfigurationValue("nonexistent.key"));
        assertFalse(settingsService.isConfigurationSet("nonexistent.key"));
        
        // Test empty value
        settingsService.setConfigurationValue("empty.key", "");
        assertEquals("", settingsService.getConfigurationValue("empty.key"));
        assertFalse(settingsService.isConfigurationSet("empty.key")); // Empty string should be considered as not set
        
        // Test whitespace value
        settingsService.setConfigurationValue("whitespace.key", "   ");
        assertEquals("   ", settingsService.getConfigurationValue("whitespace.key"));
        assertFalse(settingsService.isConfigurationSet("whitespace.key")); // Whitespace should be considered as not set
    }
}
