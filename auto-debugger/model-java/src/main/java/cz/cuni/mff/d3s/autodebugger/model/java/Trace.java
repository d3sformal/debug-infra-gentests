package cz.cuni.mff.d3s.autodebugger.model.java;

import java.util.*;

public class Trace {
  private final Map<Integer, Set<Byte>> byteValues = new HashMap<>();
  private final Map<Integer, Set<Character>> charValues = new HashMap<>();
  private final Map<Integer, Set<Short>> shortValues = new HashMap<>();
  private final Map<Integer, Set<Integer>> intValues = new HashMap<>();
  private final Map<Integer, Set<Long>> longValues = new HashMap<>();
  private final Map<Integer, Set<Float>> floatValues = new HashMap<>();
  private final Map<Integer, Set<Double>> doubleValues = new HashMap<>();
  private final Map<Integer, Set<Boolean>> booleanValues = new HashMap<>();

  public void addByteValue(int slot, byte value) {
    if (byteValues.containsKey(slot)) {
      byteValues.get(slot).add(value);
    } else {
      byteValues.put(slot, new HashSet<>(List.of(value)));
    }
  }

  public void addCharValue(int slot, char value) {
    if (charValues.containsKey(slot)) {
      charValues.get(slot).add(value);
    } else {
      charValues.put(slot, new HashSet<>(List.of(value)));
    }
  }

  public void addShortValue(int slot, short value) {
    if (shortValues.containsKey(slot)) {
      shortValues.get(slot).add(value);
    } else {
      shortValues.put(slot, new HashSet<>(List.of(value)));
    }
  }

  public void addIntValue(int slot, int value) {
    if (intValues.containsKey(slot)) {
      intValues.get(slot).add(value);
    } else {
      intValues.put(slot, new HashSet<>(List.of(value)));
    }
  }

  public void addLongValue(int slot, long value) {
    if (longValues.containsKey(slot)) {
      longValues.get(slot).add(value);
    } else {
      longValues.put(slot, new HashSet<>(List.of(value)));
    }
  }

  public void addFloatValue(int slot, float value) {
    if (floatValues.containsKey(slot)) {
      floatValues.get(slot).add(value);
    } else {
      floatValues.put(slot, new HashSet<>(List.of(value)));
    }
  }

  public void addDoubleValue(int slot, double value) {
    if (doubleValues.containsKey(slot)) {
      doubleValues.get(slot).add(value);
    } else {
      doubleValues.put(slot, new HashSet<>(List.of(value)));
    }
  }

  public void addBooleanValue(int slot, boolean value) {
    if (booleanValues.containsKey(slot)) {
      booleanValues.get(slot).add(value);
    } else {
      booleanValues.put(slot, new HashSet<>(List.of(value)));
    }
  }

  public void printSlotValues() {
    printSlotValues(byteValues);
    printSlotValues(charValues);
    printSlotValues(shortValues);
    printSlotValues(intValues);
    printSlotValues(longValues);
    printSlotValues(floatValues);
    printSlotValues(doubleValues);
    printSlotValues(booleanValues);
  }

  private <T> void printSlotValues(final Map<Integer, Set<T>> slotValues) {
    for (Map.Entry<Integer, Set<T>> entry : slotValues.entrySet()) {
      System.out.println("Slot: " + entry.getKey() + " values: " + entry.getValue());
    }
  }
}
