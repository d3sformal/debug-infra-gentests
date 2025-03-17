# Paper notes

# MicroExecution
* [source](https://patricegodefroid.github.io/public_psfiles/icse2014.pdf)

- Any code fragment without a test driver or input data
- No source, debug symbols
- NirvanaVM + SAGE 
- Underconstraining of input, “memory policies” to restrict inputs
- In coop with Valgrind etc
- Values injected via memory-mapping and value injection on insn level
- (Section 2, 3 for exact execution example)
- Decomposition into micro ops -> catch memory ops BEFORE they occur (for true architectural emulation)
    - Extension of VM -> PreMemoryAccessCallback
- Ensuring address uniqueness -> malloc(1)
- Replay capabilities
- “Fork” method “mandatory in the presence of some c++ idioms”
    - Mentioned use cases: “Backtracking”, test generation, in-memory fuzzing
- SAGE integration explores all exec paths
- IDNA records all runtime sources of nondeterminism - replay capability
- Thought: look into possible issues with ASLR (working around / disabling for certain processes)
- Mentions limitations of STATIC analysis: imprecision, assuming API conventions, requires source + user interaction 

# Enlighten
* [source](https://www.usenix.org/system/files/conference/usenixsecurity15/sec15-paper-ramos.pdf)

- Statistical Fault Localization 
    - Suspiciousness for each entity computed dynamically, proportional to failing tests on that traverse the entity
    - Enlighten authors point out how users often don’t make good decisions despite presented with useful information
    - SFL -> identify subset of execution that is suspicious (invocation of a method) -> query to developer about the invocation -> repeat
- Dynamic dependency graph(statements, dependencies between them)
- Shorter traces prioritized for query generation
-  “Output includes state of … parameters, modified globals & return value”
- Limited by Java PathFinder 
- Algorithmic Debugging - an older similar concept
- Mentions great engineering effort, unsupported features requiring more effort and possibly significant perf overhead due to dynamic slicing (dependencies?)

# Under-constrained Symbolic execution
* [source](https://dl.acm.org/doi/pdf/10.1145/3180155.3180242)

- Based on KLEE - symbolic VM
    - Considers all execution paths & all inputs (compared to tools like PIN which only explore 1 path, are as good as the tests)
    - Under-constrained - skips the “execution path prefix” (from main to target function) - missing preconditions
- C/C++ compiled to *bitcode*
- Missing preconditions => spurious errors => annotations to help guide the execution (Figure 11. & vicinity)
- Inputs are initialized lazily
- During symbolic execution, constrains on data is piling up with each conditional / write
    - UC-KLEE explores all paths with a limit (e.g. going through a linked list, mentioned as *k-bounding*)
- Does not handle assembly, function pointers without user input, different build configurations, floating point operations, aliasing (?? This could be extreme) and cyclic DSs
- “Find Set of heap object that are reachable from function’s call arguments using a precise approach based on pointer referents”
- To avoid path explosion, library functions were replaced (“we elide further details due to space”)
