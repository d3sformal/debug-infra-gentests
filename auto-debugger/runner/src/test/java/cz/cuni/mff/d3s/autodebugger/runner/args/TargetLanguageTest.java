package cz.cuni.mff.d3s.autodebugger.runner.args;

import cz.cuni.mff.d3s.autodebugger.model.common.TargetLanguage;
import org.junit.jupiter.api.Test;

import static org.junit.jupiter.api.Assertions.*;

class TargetLanguageTest {
    
    @Test
    void givenValidIdentifier_whenCreatingFromIdentifier_thenReturnsCorrectLanguage() {
        assertEquals(TargetLanguage.JAVA, TargetLanguage.fromIdentifier("java"));
        assertEquals(TargetLanguage.JAVA, TargetLanguage.fromIdentifier("JAVA"));
        assertEquals(TargetLanguage.JAVA, TargetLanguage.fromIdentifier("Java"));
        assertEquals(TargetLanguage.JAVA, TargetLanguage.fromIdentifier("  java  "));
    }
    
    @Test
    void givenInvalidIdentifier_whenCreatingFromIdentifier_thenThrowsException() {
        assertThrows(IllegalArgumentException.class, () -> {
            TargetLanguage.fromIdentifier("invalid");
        });
        
        assertThrows(IllegalArgumentException.class, () -> {
            TargetLanguage.fromIdentifier(null);
        });
        
        assertThrows(IllegalArgumentException.class, () -> {
            TargetLanguage.fromIdentifier("");
        });
        
        assertThrows(IllegalArgumentException.class, () -> {
            TargetLanguage.fromIdentifier("   ");
        });
    }
    
    @Test
    void givenLanguageIdentifier_whenCheckingSupport_thenReturnsCorrectResult() {
        assertTrue(TargetLanguage.isSupported("java"));
        assertTrue(TargetLanguage.isSupported("JAVA"));

        assertFalse(TargetLanguage.isSupported("invalid"));
        assertFalse(TargetLanguage.isSupported(null));
        assertFalse(TargetLanguage.isSupported(""));
    }
    
    @Test
    void givenTargetLanguage_whenGettingSupportedLanguagesString_thenReturnsValidString() {
        String supported = TargetLanguage.getSupportedLanguagesString();
        
        assertNotNull(supported);
        assertTrue(supported.contains("java"));
    }
    
    @Test
    void givenFileExtensions_whenValidating_thenReturnsCorrectValidation() {
        // Test Java
        assertTrue(TargetLanguage.JAVA.validateFileExtension("app.jar", true));
        assertTrue(TargetLanguage.JAVA.validateFileExtension("Test.java", false));
        assertFalse(TargetLanguage.JAVA.validateFileExtension("app.exe", true));
        assertFalse(TargetLanguage.JAVA.validateFileExtension("test.py", false));
        
        // Test edge cases
        assertFalse(TargetLanguage.JAVA.validateFileExtension(null, true));
        assertFalse(TargetLanguage.JAVA.validateFileExtension("", false));
        
        // Test case insensitive
        assertTrue(TargetLanguage.JAVA.validateFileExtension("APP.JAR", true));
        assertTrue(TargetLanguage.JAVA.validateFileExtension("Test.JAVA", false));
    }
}
