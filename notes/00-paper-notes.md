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

# Test-Case Reduction for C Compiler Bugs
* [source](https://users.cs.utah.edu/~regehr/papers/pldi12-preprint.pdf)

- kinda interesting ideas when it comes to source code modifications
    - source simplifications like removing fn arguments, replacing aggregates with member scalars
- kinda specific to compiler bugs -> no external inputs, limited to a single binary that "crashes"
- KCC and Frama-C as standard-checking tools / UB-sanitizers

# MIMIC: Locating and Understanding Bugs by Analyzing Mimicked Executions
* [source](https://dl.acm.org/doi/10.1145/2642937.2643014)

- anomaly detection based on passing and failing test cases
- interesting angle of attach: inspect properties during execution (simple integer comparisons, unsigned integer types of over/underflows, no-progress loops, suspiciously many loop iterations, ...)
    - better to integrate this with a debugger?
- expensive (tracking program properties - explosion)
    - even with optimizations, one example program took an entire day to be analyzed
- integrates with many tools (GCOV, GDB)
- authors admit their results might not generalize (pesimism - justified?) 

# Symbolic execution with SymCC: Don't interpret, compile!
* [source](https://www.usenix.org/conference/usenixsecurity20/presentation/poeplau)

- single-execution path symbolic execution (concolic)
- main "breakthrough" is usage of LLVM IR instrumentation as a lightweight (both sLOC footprint and runtime overhead)
    - ~2k sLOC, near-native performance compared to interpretation techniques (not sure if normalized for omni-path vs single-path symbolic execution)
    - compared to techniques above, they handle binary-only libraries & inline assembly "gracefully" (skip but do not harm execution)
- SYMCC generates new input during the symbolic execution, increasing coverage
- mentioned other techniques for IR manipulation/generation:
    - Valgrind's VEX
    - LLVM IR generated from QEMU's internal represenataion
- comparisons with dynamic instrumentation techniques
- nice examples of LLVM IR (wrt. the paper's topic & even as a sort of introductory material)
- calls to a symbolic execution library - the EXACT thing I've proposed (for syntactic version)
    - this (and above) prompted me to implement LLVM IR experiment
    - simple, what remains is the execution forking
# AURORA: Statistical Crash Analysis for Automated Root Cause Explanation
* [source](https://www.usenix.org/conference/usenixsecurity20/presentation/blazytko)
