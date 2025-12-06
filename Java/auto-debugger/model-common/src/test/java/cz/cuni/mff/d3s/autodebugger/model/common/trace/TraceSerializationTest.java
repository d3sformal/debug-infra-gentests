package cz.cuni.mff.d3s.autodebugger.model.common.trace;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.io.TempDir;

import java.io.*;
import java.nio.file.Path;
import java.util.Set;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Tests for Trace serialization and deserialization.
 */
class TraceSerializationTest {
    
    @TempDir
    Path tempDir;
    
    @Test
    void givenTraceWithIntValues_whenSerializedAndDeserialized_thenValuesPreserved() throws Exception {
        // Given
        Trace original = new Trace();
        original.addIntValue(0, 42);
        original.addIntValue(0, 100);
        original.addIntValue(1, 200);
        
        Path traceFile = tempDir.resolve("trace.ser");
        
        // When - serialize
        try (FileOutputStream fos = new FileOutputStream(traceFile.toFile());
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {
            oos.writeObject(original);
        }
        
        // When - deserialize
        Trace deserialized;
        try (FileInputStream fis = new FileInputStream(traceFile.toFile());
             ObjectInputStream ois = new ObjectInputStream(fis)) {
            deserialized = (Trace) ois.readObject();
        }
        
        // Then
        assertNotNull(deserialized);
        Set<Integer> slot0Values = deserialized.getIntValues(0);
        Set<Integer> slot1Values = deserialized.getIntValues(1);
        
        assertNotNull(slot0Values);
        assertTrue(slot0Values.contains(42));
        assertTrue(slot0Values.contains(100));
        assertEquals(2, slot0Values.size());
        
        assertNotNull(slot1Values);
        assertTrue(slot1Values.contains(200));
        assertEquals(1, slot1Values.size());
    }
    
    @Test
    void givenTraceWithAllPrimitiveTypes_whenSerializedAndDeserialized_thenAllValuesPreserved() throws Exception {
        // Given
        Trace original = new Trace();
        original.addByteValue(0, (byte) 1);
        original.addCharValue(0, 'A');
        original.addShortValue(0, (short) 100);
        original.addIntValue(0, 1000);
        original.addLongValue(0, 10000L);
        original.addFloatValue(0, 1.5f);
        original.addDoubleValue(0, 2.5);
        original.addBooleanValue(0, true);
        
        Path traceFile = tempDir.resolve("trace.ser");
        
        // When - serialize and deserialize
        try (FileOutputStream fos = new FileOutputStream(traceFile.toFile());
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {
            oos.writeObject(original);
        }
        
        Trace deserialized;
        try (FileInputStream fis = new FileInputStream(traceFile.toFile());
             ObjectInputStream ois = new ObjectInputStream(fis)) {
            deserialized = (Trace) ois.readObject();
        }
        
        // Then
        assertNotNull(deserialized);
        assertTrue(deserialized.getByteValues(0).contains((byte) 1));
        assertTrue(deserialized.getCharValues(0).contains('A'));
        assertTrue(deserialized.getShortValues(0).contains((short) 100));
        assertTrue(deserialized.getIntValues(0).contains(1000));
        assertTrue(deserialized.getLongValues(0).contains(10000L));
        assertTrue(deserialized.getFloatValues(0).contains(1.5f));
        assertTrue(deserialized.getDoubleValues(0).contains(2.5));
        assertTrue(deserialized.getBooleanValues(0).contains(true));
    }
    
    @Test
    void givenEmptyTrace_whenSerializedAndDeserialized_thenRemainsEmpty() throws Exception {
        // Given
        Trace original = new Trace();
        Path traceFile = tempDir.resolve("trace.ser");
        
        // When - serialize and deserialize
        try (FileOutputStream fos = new FileOutputStream(traceFile.toFile());
             ObjectOutputStream oos = new ObjectOutputStream(fos)) {
            oos.writeObject(original);
        }
        
        Trace deserialized;
        try (FileInputStream fis = new FileInputStream(traceFile.toFile());
             ObjectInputStream ois = new ObjectInputStream(fis)) {
            deserialized = (Trace) ois.readObject();
        }
        
        // Then
        assertNotNull(deserialized);
        // Empty trace should have no values for any slot
        assertTrue(deserialized.getIntValues(0).isEmpty());
        assertTrue(deserialized.getIntValues(1).isEmpty());
    }
}

