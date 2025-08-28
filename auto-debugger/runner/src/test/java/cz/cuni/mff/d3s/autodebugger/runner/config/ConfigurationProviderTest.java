package cz.cuni.mff.d3s.autodebugger.runner.config;

import org.junit.jupiter.api.Test;

import java.util.List;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Test class for ConfigurationProvider functionality.
 */
public class ConfigurationProviderTest {

    @Test
    public void givenConfigurationProvider_whenGettingRequiredFields_thenReturnsCorrectFields() {
        List<ConfigurationField> requiredFields = ConfigurationProvider.getRequiredConfigurationFields();

        assertNotNull(requiredFields);
        assertEquals(2, requiredFields.size());

        // Check DiSL Path field
        ConfigurationField dislField = requiredFields.stream()
                .filter(field -> "disl.path".equals(field.getKey()))
                .findFirst()
                .orElse(null);

        assertNotNull(dislField);
        assertEquals("DiSL Path", dislField.getDisplayName());
        assertEquals(ConfigurationField.FieldType.DIRECTORY_PATH, dislField.getType());
        assertTrue(dislField.isRequired());

        // Check Anthropic API Key field
        ConfigurationField anthropicField = requiredFields.stream()
                .filter(field -> "anthropic.key".equals(field.getKey()))
                .findFirst()
                .orElse(null);

        assertNotNull(anthropicField);
        assertEquals("Anthropic API Key", anthropicField.getDisplayName());
        assertEquals(ConfigurationField.FieldType.PASSWORD, anthropicField.getType());
        assertFalse(anthropicField.isRequired()); // Now optional since it can come from environment
    }

    @Test
    public void givenConfigurationProvider_whenGettingOptionalFields_thenReturnsCorrectFields() {
        List<ConfigurationField> optionalFields = ConfigurationProvider.getOptionalConfigurationFields();
        
        assertNotNull(optionalFields);
        assertEquals(3, optionalFields.size());
        
        // Check that all optional fields are not required
        for (ConfigurationField field : optionalFields) {
            assertFalse(field.isRequired());
        }
        
        // Check specific fields exist
        assertTrue(optionalFields.stream().anyMatch(field -> "output.directory".equals(field.getKey())));
        assertTrue(optionalFields.stream().anyMatch(field -> "max.analysis.time".equals(field.getKey())));
        assertTrue(optionalFields.stream().anyMatch(field -> "verbose.logging".equals(field.getKey())));
    }

    @Test
    public void givenConfigurationProvider_whenGettingAllFields_thenCombinesRequiredAndOptional() {
        List<ConfigurationField> allFields = ConfigurationProvider.getAllConfigurationFields();
        List<ConfigurationField> requiredFields = ConfigurationProvider.getRequiredConfigurationFields();
        List<ConfigurationField> optionalFields = ConfigurationProvider.getOptionalConfigurationFields();
        
        assertNotNull(allFields);
        assertEquals(requiredFields.size() + optionalFields.size(), allFields.size());
        
        // Check that all required fields are included
        for (ConfigurationField requiredField : requiredFields) {
            assertTrue(allFields.stream().anyMatch(field -> field.getKey().equals(requiredField.getKey())));
        }
        
        // Check that all optional fields are included
        for (ConfigurationField optionalField : optionalFields) {
            assertTrue(allFields.stream().anyMatch(field -> field.getKey().equals(optionalField.getKey())));
        }
    }

    @Test
    public void givenConfigurationField_whenCheckingProperties_thenReturnsCorrectValues() {
        ConfigurationField field = new ConfigurationField(
                "test.key",
                "Test Field",
                "Test description",
                ConfigurationField.FieldType.TEXT,
                true,
                "default value"
        );
        
        assertEquals("test.key", field.getKey());
        assertEquals("Test Field", field.getDisplayName());
        assertEquals("Test description", field.getDescription());
        assertEquals(ConfigurationField.FieldType.TEXT, field.getType());
        assertTrue(field.isRequired());
        assertEquals("default value", field.getDefaultValue());
    }
}
