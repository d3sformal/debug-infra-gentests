package cz.cuni.mff.d3s.autodebugger.model.java;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.ExportableValue;
import lombok.extern.slf4j.Slf4j;

import java.util.Map;
import java.util.Set;

/**
 * Adapter class that bridges between the legacy Trace implementation
 * and the new EnhancedTrace implementation following the specification.
 * 
 * This allows gradual migration from the slot-based approach to the
 * event-based approach while maintaining backward compatibility.
 */
@Slf4j
public class TraceAdapter {
    
    /**
     * Converts a legacy Trace to an EnhancedTrace using identifier mapping.
     * 
     * @param legacyTrace The original slot-based trace
     * @param identifierMapping Mapping from slots to ExportableValue identifiers
     * @return EnhancedTrace with converted data
     */
    public static EnhancedTrace convertToEnhanced(Trace legacyTrace, 
                                                 Map<Integer, ExportableValue> identifierMapping) {
        log.info("Converting legacy trace to enhanced trace with {} identifiers", 
                identifierMapping.size());
        
        EnhancedTrace enhancedTrace = new EnhancedTrace();
        int eventIndex = 0;
        
        // Convert each slot's values to the enhanced format
        for (Map.Entry<Integer, ExportableValue> entry : identifierMapping.entrySet()) {
            Integer slot = entry.getKey();
            ExportableValue identifier = entry.getValue();
            
            // Convert values based on the identifier type
            String type = identifier.getType();
            eventIndex = convertSlotValues(legacyTrace, enhancedTrace, slot, identifier, type, eventIndex);
        }
        
        // Add metadata about the conversion
        enhancedTrace.addMetadata("converted_from", "legacy_trace");
        enhancedTrace.addMetadata("conversion_timestamp", System.currentTimeMillis());
        enhancedTrace.addMetadata("original_slot_count", identifierMapping.size());
        
        log.info("Conversion completed. Enhanced trace contains {} variables with {} total events",
                enhancedTrace.getTrackedVariableCount(), enhancedTrace.getTotalEventCount());
        
        return enhancedTrace;
    }
    
    /**
     * Converts values from a specific slot in the legacy trace to the enhanced trace.
     */
    private static int convertSlotValues(Trace legacyTrace, EnhancedTrace enhancedTrace,
                                       Integer slot, ExportableValue identifier, 
                                       String type, int startEventIndex) {
        int currentEventIndex = startEventIndex;
        
        switch (type.toLowerCase()) {
            case "byte":
                Set<Byte> byteValues = legacyTrace.getByteValues(slot);
                for (Byte value : byteValues) {
                    enhancedTrace.addValue(identifier, currentEventIndex++, value);
                }
                break;
                
            case "char":
                Set<Character> charValues = legacyTrace.getCharValues(slot);
                for (Character value : charValues) {
                    enhancedTrace.addValue(identifier, currentEventIndex++, value);
                }
                break;
                
            case "short":
                Set<Short> shortValues = legacyTrace.getShortValues(slot);
                for (Short value : shortValues) {
                    enhancedTrace.addValue(identifier, currentEventIndex++, value);
                }
                break;
                
            case "int":
                Set<Integer> intValues = legacyTrace.getIntValues(slot);
                for (Integer value : intValues) {
                    enhancedTrace.addValue(identifier, currentEventIndex++, value);
                }
                break;
                
            case "long":
                Set<Long> longValues = legacyTrace.getLongValues(slot);
                for (Long value : longValues) {
                    enhancedTrace.addValue(identifier, currentEventIndex++, value);
                }
                break;
                
            case "float":
                Set<Float> floatValues = legacyTrace.getFloatValues(slot);
                for (Float value : floatValues) {
                    enhancedTrace.addValue(identifier, currentEventIndex++, value);
                }
                break;
                
            case "double":
                Set<Double> doubleValues = legacyTrace.getDoubleValues(slot);
                for (Double value : doubleValues) {
                    enhancedTrace.addValue(identifier, currentEventIndex++, value);
                }
                break;
                
            case "boolean":
                Set<Boolean> booleanValues = legacyTrace.getBooleanValues(slot);
                for (Boolean value : booleanValues) {
                    enhancedTrace.addValue(identifier, currentEventIndex++, value);
                }
                break;
                
            default:
                log.warn("Unknown type '{}' for identifier {}, skipping conversion", type, identifier);
                break;
        }
        
        return currentEventIndex;
    }
    
    /**
     * Creates a synthetic EnhancedTrace for testing purposes with realistic data patterns.
     * 
     * @param identifierMapping Mapping of identifiers to use
     * @return EnhancedTrace with synthetic test data
     */
    public static EnhancedTrace createSyntheticTrace(Map<Integer, ExportableValue> identifierMapping) {
        log.info("Creating synthetic enhanced trace for testing");
        
        EnhancedTrace trace = new EnhancedTrace();
        int eventIndex = 0;
        
        // Simulate a realistic execution sequence
        for (Map.Entry<Integer, ExportableValue> entry : identifierMapping.entrySet()) {
            ExportableValue identifier = entry.getValue();
            String type = identifier.getType();
            
            // Add multiple values over time to simulate real execution
            switch (type.toLowerCase()) {
                case "int":
                    trace.addValue(identifier, eventIndex++, 10);
                    trace.addValue(identifier, eventIndex++, 20);
                    trace.addValue(identifier, eventIndex++, 30);
                    break;
                    
                case "boolean":
                    trace.addValue(identifier, eventIndex++, true);
                    trace.addValue(identifier, eventIndex++, false);
                    break;
                    
                case "string":
                    trace.addValue(identifier, eventIndex++, "initial");
                    trace.addValue(identifier, eventIndex++, "modified");
                    break;
                    
                case "double":
                    trace.addValue(identifier, eventIndex++, 3.14);
                    trace.addValue(identifier, eventIndex++, 2.71);
                    break;
                    
                default:
                    // Generic object
                    trace.addValue(identifier, eventIndex++, "value_" + eventIndex);
                    break;
            }
        }
        
        // Add metadata
        trace.addMetadata("synthetic", true);
        trace.addMetadata("creation_timestamp", System.currentTimeMillis());
        trace.addMetadata("purpose", "testing");
        
        log.info("Created synthetic trace with {} variables and {} events",
                trace.getTrackedVariableCount(), trace.getTotalEventCount());
        
        return trace;
    }
    
    /**
     * Merges multiple EnhancedTrace instances into a single trace.
     * Event indices are adjusted to maintain chronological order.
     * 
     * @param traces Array of traces to merge
     * @return Merged EnhancedTrace
     */
    public static EnhancedTrace mergeTraces(EnhancedTrace... traces) {
        log.info("Merging {} traces", traces.length);

        EnhancedTrace mergedTrace = new EnhancedTrace();
        int[] eventOffsetArray = {0}; // Use array to make it effectively final

        for (int i = 0; i < traces.length; i++) {
            EnhancedTrace trace = traces[i];
            final int traceIndex = i; // Make final for lambda

            // Copy all variables from this trace with adjusted event indices
            for (ExportableValue identifier : trace.getTrackedIdentifiers()) {
                var values = trace.getValues(identifier);
                for (Map.Entry<Integer, Object> entry : values.entrySet()) {
                    int adjustedEventIndex = entry.getKey() + eventOffsetArray[0];
                    mergedTrace.addValue(identifier, adjustedEventIndex, entry.getValue());
                }
            }

            // Update offset for next trace
            trace.getEventIndexRange().ifPresent(range -> {
                eventOffsetArray[0] = range[1] + 1; // Start next trace after this one
            });

            // Merge metadata
            trace.getAllMetadata().forEach((key, value) ->
                mergedTrace.addMetadata("trace_" + traceIndex + "_" + key, value));
        }
        
        mergedTrace.addMetadata("merged_from", traces.length + "_traces");
        mergedTrace.addMetadata("merge_timestamp", System.currentTimeMillis());
        
        log.info("Merged trace contains {} variables with {} total events",
                mergedTrace.getTrackedVariableCount(), mergedTrace.getTotalEventCount());
        
        return mergedTrace;
    }
}
