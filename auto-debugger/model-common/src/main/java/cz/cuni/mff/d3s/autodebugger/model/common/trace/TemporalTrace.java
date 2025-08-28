package cz.cuni.mff.d3s.autodebugger.model.common.trace;

import cz.cuni.mff.d3s.autodebugger.model.common.identifiers.ExportableValue;
import lombok.extern.slf4j.Slf4j;

import java.io.Serializable;
import java.util.*;
import java.util.concurrent.ConcurrentHashMap;

/**
 * Temporal Trace stores runtime data as a three-dimensional structure mapping
 * variables (ExportableValue) at specific execution points (event indices) to their values.
 * The design uses the Facade pattern to hide the complexity of the internal
 * map-of-maps structure and provides efficient queries for test generation.
 */
@Slf4j
public class TemporalTrace implements Serializable {
    
    /**
     * Core data structure: Map<ExportableValue, NavigableMap<Integer, Object>>
     * - Outer map: ExportableValue identifier -> variable history
     * - Inner map: Event index -> captured value (TreeMap for efficient floor queries)
     */
    private final Map<ExportableValue, NavigableMap<Integer, Object>> traceData;
    
    /**
     * Global event counter for maintaining execution order
     */
    private int nextEventIndex;
    
    /**
     * Metadata about the trace collection
     */
    private final Map<String, Object> metadata;
    
    public TemporalTrace() {
        this.traceData = new ConcurrentHashMap<>();
        this.nextEventIndex = 0;
        this.metadata = new HashMap<>();
    }
    
    /**
     * Adds a captured runtime value to the trace.
     *
     * @param identifier The unique identifier for the variable being tracked.
     * @param eventIndex The point in the execution timeline when the value was captured.
     * @param value The runtime value of the variable.
     */
    public void addValue(ExportableValue identifier, int eventIndex, Object value) {
        log.debug("Adding value for identifier {} at event {}: {}", 
                 identifier, eventIndex, value);
        
        traceData.computeIfAbsent(identifier, k -> new TreeMap<>())
                 .put(eventIndex, value);
        
        // Update next event index if this is the latest event
        if (eventIndex >= nextEventIndex) {
            nextEventIndex = eventIndex + 1;
        }
    }
    
    /**
     * Convenience method to add a value with auto-generated event index.
     *
     * @param identifier The unique identifier for the variable being tracked.
     * @param value The runtime value of the variable.
     * @return The assigned event index.
     */
    public int addValue(ExportableValue identifier, Object value) {
        int eventIndex = nextEventIndex++;
        addValue(identifier, eventIndex, value);
        return eventIndex;
    }
    
    /**
     * Retrieves the most recent value of a variable at or before a given event index.
     * This is the primary method for reconstructing the state of the application
     * at a specific point in time.
     *
     * @param identifier The identifier of the variable to query.
     * @param eventIndex The upper bound for the event timeline search.
     * @return An Optional containing the value, or an empty Optional if no value was recorded
     *         at or before the specified index.
     */
    public Optional<Object> getLatestValueBefore(ExportableValue identifier, int eventIndex) {
        NavigableMap<Integer, Object> variableHistory = traceData.get(identifier);

        if (variableHistory == null || variableHistory.isEmpty()) {
            log.debug("No history found for identifier: {}", identifier);
            return Optional.empty();
        }

        // Use NavigableMap's floorEntry to find the greatest key <= eventIndex
        Map.Entry<Integer, Object> floorEntry = variableHistory.floorEntry(eventIndex);
        
        if (floorEntry == null) {
            log.debug("No value found at or before event {} for identifier: {}", 
                     eventIndex, identifier);
            return Optional.empty();
        }
        
        log.debug("Found value for identifier {} at event {}: {}", 
                 identifier, floorEntry.getKey(), floorEntry.getValue());
        return Optional.of(floorEntry.getValue());
    }
    
    /**
     * Retrieves the complete history of values for a specific variable.
     *
     * @param identifier The identifier of the variable to query.
     * @return A SortedMap of event indices to values, or an empty map if the variable
     *         was never tracked.
     */
    public SortedMap<Integer, Object> getValues(ExportableValue identifier) {
        NavigableMap<Integer, Object> history = traceData.get(identifier);
        return history != null ? new TreeMap<>(history) : new TreeMap<>();
    }
    
    /**
     * Retrieves a snapshot of all tracked variables and their most recent values
     * at or before the specified event index.
     *
     * @param eventIndex The point in time for the snapshot.
     * @return A Map of variable identifiers to their last known value.
     */
    public Map<ExportableValue, Object> getStateSnapshotAt(int eventIndex) {
        Map<ExportableValue, Object> snapshot = new HashMap<>();
        
        for (ExportableValue identifier : traceData.keySet()) {
            getLatestValueBefore(identifier, eventIndex)
                .ifPresent(value -> snapshot.put(identifier, value));
        }
        
        log.debug("Created state snapshot at event {} with {} variables", 
                 eventIndex, snapshot.size());
        return snapshot;
    }
    
    /**
     * Gets all tracked variable identifiers.
     *
     * @return Set of all ExportableValue identifiers that have been tracked.
     */
    public Set<ExportableValue> getTrackedIdentifiers() {
        return new HashSet<>(traceData.keySet());
    }
    
    /**
     * Gets the range of event indices in this trace.
     *
     * @return Optional containing [min, max] event indices, or empty if no events.
     */
    public Optional<int[]> getEventIndexRange() {
        int minEvent = Integer.MAX_VALUE;
        int maxEvent = Integer.MIN_VALUE;
        boolean hasEvents = false;

        for (NavigableMap<Integer, Object> history : traceData.values()) {
            if (!history.isEmpty()) {
                hasEvents = true;
                minEvent = Math.min(minEvent, history.firstKey());
                maxEvent = Math.max(maxEvent, history.lastKey());
            }
        }
        
        return hasEvents ? Optional.of(new int[]{minEvent, maxEvent}) : Optional.empty();
    }
    
    /**
     * Gets the total number of recorded events across all variables.
     *
     * @return Total number of value recordings.
     */
    public int getTotalEventCount() {
        return traceData.values().stream()
                       .mapToInt(Map::size)
                       .sum();
    }
    
    /**
     * Gets the number of unique variables tracked.
     *
     * @return Number of tracked variables.
     */
    public int getTrackedVariableCount() {
        return traceData.size();
    }
    
    /**
     * Adds metadata about the trace collection.
     *
     * @param key Metadata key.
     * @param value Metadata value.
     */
    public void addMetadata(String key, Object value) {
        metadata.put(key, value);
    }
    
    /**
     * Gets metadata about the trace collection.
     *
     * @param key Metadata key.
     * @return Metadata value, or null if not found.
     */
    public Object getMetadata(String key) {
        return metadata.get(key);
    }
    
    /**
     * Gets all metadata.
     *
     * @return Copy of all metadata.
     */
    public Map<String, Object> getAllMetadata() {
        return new HashMap<>(metadata);
    }
    
    /**
     * Clears all trace data and metadata.
     */
    public void clear() {
        traceData.clear();
        metadata.clear();
        nextEventIndex = 0;
        log.info("Trace data cleared");
    }
    
    /**
     * Gets a summary of the trace contents for debugging.
     *
     * @return String summary of trace statistics.
     */
    public String getSummary() {
        StringBuilder sb = new StringBuilder();
        sb.append("TemporalTrace Summary:\n");
        sb.append("  - Tracked variables: ").append(getTrackedVariableCount()).append("\n");
        sb.append("  - Total events: ").append(getTotalEventCount()).append("\n");
        
        getEventIndexRange().ifPresentOrElse(
            range -> sb.append("  - Event range: [").append(range[0]).append(", ").append(range[1]).append("]\n"),
            () -> sb.append("  - Event range: empty\n")
        );
        
        sb.append("  - Next event index: ").append(nextEventIndex).append("\n");
        sb.append("  - Metadata entries: ").append(metadata.size());
        
        return sb.toString();
    }
}
