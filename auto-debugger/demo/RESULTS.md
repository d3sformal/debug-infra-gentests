# Auto-Debugger Demo Results

## Calculator App (args)

### Input
- **Application**: `CalcApp.java` calling `Calculator.add(int,int)`
- **Method calls**: `add(1,2)`, `add(3,4)`, `add(5,6)`
- **Target parameters**: `0:int`, `1:int`

### Naive Trace
```
Slot 1: {1, 3, 5}
Slot 2: {2, 4, 6}
```

### Temporal Trace
```
Invocation 1: [1, 2]
Invocation 2: [3, 4] 
Invocation 3: [5, 6]
```

### LLM Prompt Context
```
Method: com.example.Calculator.add(int, int)
Source Code:
public int add(int a, int b) {
    return a + b;
}

Trace Data:
- Parameter 'a': values {1, 3, 5}
- Parameter 'b': values {2, 4, 6}
- Cross-product combinations: 9 scenarios

Generate comprehensive JUnit 5 tests covering all parameter combinations.
```

### Generated Test Suite
```java
@Test void testAdd_Scenario1() { calculator.add(1, 2); }
@Test void testAdd_Scenario2() { calculator.add(1, 4); }
@Test void testAdd_Scenario3() { calculator.add(1, 6); }
@Test void testAdd_Scenario4() { calculator.add(3, 2); }
@Test void testAdd_Scenario5() { calculator.add(3, 4); }
@Test void testAdd_Scenario6() { calculator.add(3, 6); }
@Test void testAdd_Scenario7() { calculator.add(5, 2); }
@Test void testAdd_Scenario8() { calculator.add(5, 4); }
@Test void testAdd_Scenario9() { calculator.add(5, 6); }
```
**Coverage**: 9 scenarios (3×3 cross-product)

---

## Counter App (fields)

### Input
- **Application**: `FieldApp.java` calling `Counter.increment()`
- **Method calls**: `increment()` × 3
- **Target fields**: `count:int`

### Naive Trace
```
Field count: {0, 1, 2}
```

### Temporal Trace
```
Invocation 1: count=0
Invocation 2: count=1
Invocation 3: count=2
```

### LLM Prompt Context
```
Method: com.example.Counter.increment()
Source Code:
public void increment() {
    this.count++;
}

Field: private int count
Trace Data:
- Field 'count' states: {0, 1, 2}
- Temporal sequence: 0 → 1 → 2

Generate JUnit 5 tests with field initialization for each captured state.
```

### Generated Test Suite
```java
@Test void testIncrement_Scenario1() { counter.count = 0; counter.increment(); }
@Test void testIncrement_Scenario2() { counter.count = 1; counter.increment(); }
@Test void testIncrement_Scenario3() { counter.count = 2; counter.increment(); }
```
**Coverage**: 3 scenarios

---

## Globals App (static)

### Input
- **Application**: `StaticApp.java` calling `Globals.bump()`
- **Method calls**: `bump()` × 4
- **Target fields**: `value:int`

### Naive Trace
```
Static field value: {0, 10, 20, 30}
```

### Temporal Trace
```
Invocation 1: value=0
Invocation 2: value=10
Invocation 3: value=20
Invocation 4: value=30
```

### LLM Prompt Context
```
Method: com.example.Globals.bump()
Source Code:
public static void bump() {
    value += 10;
}

Static Field: public static int value
Trace Data:
- Static field 'value' states: {0, 10, 20, 30}
- Temporal sequence: 0 → 10 → 20 → 30

Generate JUnit 5 tests with static field initialization for each captured state.
```

### Generated Test Suite
```java
@Test void testBump_Scenario1() { Globals.value = 0; Globals.bump(); }
@Test void testBump_Scenario2() { Globals.value = 10; Globals.bump(); }
@Test void testBump_Scenario3() { Globals.value = 20; Globals.bump(); }
@Test void testBump_Scenario4() { Globals.value = 30; Globals.bump(); }
```
**Coverage**: 4 scenarios

---

## Summary

| App | Method | Invocations | Naive Values | Temporal Sequences | Generated Tests |
|-----|--------|-------------|--------------|-------------------|-----------------|
| Calculator | `add(int,int)` | 3 | 6 unique values | 3 sequences | 9 scenarios |
| Counter | `increment()` | 3 | 3 field states | 3 sequences | 3 scenarios |
| Globals | `bump()` | 4 | 4 field states | 4 sequences | 4 scenarios |

**Total**: 16 test scenarios generated from 10 method invocations across 3 applications.
