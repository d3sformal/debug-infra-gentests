# TODO:

* ~~prepare examples for function tracing prototype~~
* ~~prepare examples for parameter capture prototype~~
* ~~think about non-deterministic traces~~
    * ~~do we permit relying on function call determinism?~~
* ~~think about C++ objects - capturing inside of them, this, ...~~
* better source organisation, READMEs where possible
* ~~move `llvm-project` somewhere more sensible~~
* add links to commits/READMEs/other files for every "DONE" item in this file
* ~~unify SOLVED vs DONE items~~
* ~~public repo?~~
* ~~remove ZMQ references entirely (too expensive to maintain)~~
    * ensure ALL links to ZMQ functionality actually link to a revision where ZMQ is available
        * ensure the revision is the same, where possible
    * also ensure legacy examples work (test-pass-working)
* more diagrams

# TOPIC: Data Capture Library

## Capturing funciton arguments

* **[DONE]** how to treat **templated** code
    * let `X` be the set of types allowed for recording:
    * `template <class T>`, parameter of type `T`, where all instantiations of `T` in `X` - should be OK with naive source modification
    * not all instantiations have `T` in `X` -> compilation errors
        * how to resolve?
        * can clang provide per-instantiation decisionmaking? how will that affect compile times/feasability?

* **[DONE]** how to treat type aliases?
    * should be doable via clang's APIs (the underlying type should be resolved, right?)

## Capturing execution trace 

* use other tools?

* **[DONE]** C vs C++ function scope tracking
    * for now I only focus on C++ code, this method should allow tracing of even code which throws exceptions
    * C has no "reasonably non-colliding" namespaces
    * on the other hand: no annoying exceptions

* **[DONE]** use of C++'s `auto` keyword
    * limits C heavily (`auto` not used widely)
    * intricacies of C++'s type system & reference semantics
        * ensuring that modified source code makes exactly the same side effects as the unmodified one

## Capturing function arguments

* think of how dynamic do we want it to be
    * function sends `fnid, modid` + `t1` + `data1` + `t2` + `data2` (`t1,t2` predefined, encode the size of subsequent `data`)
    * function sends `fnid, modid` + `len` + `packet` (`len` is lenght of the `packet`, no type information, relies on the encoding/decoding parity per a function)
    * function sends `cap_id` + `packet` (`cap_id` encodes both function's id and `packet` length)
    * 1st and 3rd approach require custom specifications (external)
        * external `t1` -> length, `t2` -> length mapping (generated in early phases)
        * external `cap_id` -> `fnid, modid, length` mapping (again, gen in early phases)
    * 2nd approach seems the best in terms of external input information required? although not the best in terms of overhead...
        * a tradeoff for the 2nd approach: `fnid, modid` -> `len` mapping
    * 1st approach presents significant overhead and hurdles to extensibility
    * comes down to 2nd vs 3rd - we're already creating a mapping (if tradeoff above implemented)
        * if not mapping, we introduce at least 8B of overhead per call more than 3rd (which for functions with small arguments becomes significant)
            * `fnid` + `modid` (8B so far) + `len` vs `cap_id` (probably less than 256)
    * seems like the 3rd option is the best (spoiler: it is not)
        * mapping ~~can~~ can't be created as part of the "modmap" generation (because of the separation of module compilation)
    * `fnid, modid` -> `len` mapping provides the most flexibility (arugment packet begins with `fnid` and `modid`)
        * in case the instrumentation gets *merged* to avoid recompilation (runtime check for which kind of instrumentation to execute), the `cap_id` approach would most likely force recompilation (generation of `cap_id` mapping)
        * compared to function call tracing, where many calls are instrumented, there should be much less overhead for argument tracing on average which is why the overhead should be acceptable
    * additionally, encoding special types (e.g. custom types or variable-length types) should be possible!
        * imagine dumping a `std::string`: we certainly need `cstr` data, maybe `size` and `capacity` - although the `size` determines the `cstr` payload size, the current model does not identify the need to interpret received payloads as such (e.g. to read different amount of data than `len` specifies)
        * therefore we need **capture map** like `fnid, modid` -> `typeid-list` where `typeid-list` lists IDs of types that are going to be received via argument capture 
            * we use `typeid-list` to **read** properly from the argument capture stream
        * all type IDs in `typeid-list` must have these components **in sync**:
            * hooklib serialization (to send arguments to `llcap-server`)
            * hooklib deserialization (to receive arguments to execute tests)
            * LLVM-pass mapping generation
            * `llcap-server` being able to parse the format
                * can be partially hadnled in the **capture map** for basic types
                    * e.g. "`typeid` 0 (representing e.g. `int`) has size `4`"
                    * or   "`typeid` 23 is a custom type with custom key `501-CustomStruct`" in which case the `llcap-server` must support the "custom key"

# TOPIC: C++ support

*  **[DUPLICATE]** ~~methods of objects with AST modification (abandoned?)~~

# TOPIC: LLVM IR approach

* shows more promise than AST modification
* trouble with filtering (non) library funcitons - information not available in the IR
    * **[DONE]** **Idea**: add metadata to the functions in the IR that could tell the LLVM pass if the function is `#include`d, library, ...
        * [LLVM Discussion](https://discourse.llvm.org/t/how-to-distinguish-between-user-defined-function-in-a-program-and-library-functions/54401/7)
    * more metadata could help with a GUI integration later (emitting line informatio metadata - *using debug information metadata*, ...)
    * **[DONE]** inject custom metadata regarding (non)library/builtin functions
* **[DONE]** capturing inside a lambda, overall lambda instrumentation
* **[DONE]** LLVM [intrinsics](http://llvm.org/docs/LangRef.html#intrinsics) - can they be used?
    * **warning, can affect code generation**
    * lack of documentation (e.g. `llvm.memcpy`)
    * even then, seem inconsequential

* **[DONE]** **important for custom IR metadata**: `isExpansionInMainFile` vs `isExpansionInSystemHeader`
    * Does `isExpansionInMainFile` exclude functions in **any** header files?
        * If yes, does `isExpansionInSystemHeader` solve this?

* **[DONE]** is the custom metadata approach outlined in [](./01-llvm-ir-metadata-emission.md#custom-llvm-ir-metadata) correct?
* **[DONE]** is it possible to inject metadata in the AST Matching phase?
    * e.g. visit every `FunctionDecl` and add data (key-value string pairs) that would later be injected as LLVM IR metadata? (this is a question because the AST itself (and `FunctionDecl`) seemed to be "`const`ant" throughout my experimentation)

* **[DONE]** investigate [Clang IR (CIR)](https://llvm.github.io/clangir/)
    * relatively new feature (doesn't seem to be used with my build of LLVM) - *ClangIR upstreaming RFC was accepted in Feb 2024,*
    * conclusion: probably useful in far future (dialect augmentation)

* **[DONE]** add an option to metadata export plugin to use mangled name approach via a plugin option
    * supply LLVM pass & use `-mllvm -llcap-filter-by-mangled` when compiling

* `custom-metadata-pass` - document build steps

# TOPIC: Typed value capture
* **[DONE]** document hurdles / bypasses / alternatives to LLVM metadata approach
    * `this` pointer
    * ABIs that return structures by value in registers (`sret`, `structureReturn`, `StructRet`)
    * **?maybe?** - dive deeper into the ABI specs and cause runtime error within the LLVM plugin if the target ABI is unknown/unhandled
* **[DONE]** document current approach
* **[DONE]** discuss possible breakage points
    * structures fitting 64 bits passed in registers
    * changes in argument order

# TOPIC: Testing / Isolation
* isolation inspiration - to investigate? - (`clone`/`fork` used): [mem-isolate](https://github.com/brannondorsey/mem-isolate)

# TOPIC: Papers

* investigate memory instruction instrumentation via Pin from the AURORA tool
    * [repo](https://github.com/RUB-SysSec/aurora)

# TOPIC: Modification of incoming arguments
* are there assumptions in the IR, that would break if we replaced arguments mid-function?
    * the idea is to "introduce new `alloca`d variable (`%x`)" for each `%n` argument and replace each `%n` occurence with the new `%x`  
* `const`ness vs `IR`-level modifications
    * does `-O3`, etc. impact the number of arguments in IR? (e.g. via more agressive const propagation?)
    * if prohibitive for argument "hijacking", can we at least "remove" `const`
* validate approach that modifies the IR

# TOPIC: Extras
* **[CLOSED]** report mangling bug (after rebuild of latest llvm project)
    * actually a [larger issue](https://discourse.llvm.org/t/rfc-clang-diagnostic-for-demangling-failures/82835) ([pull request for a diagnostic](https://github.com/llvm/llvm-project/pull/111391))

* so far only on x64 linux ABI - try more?
* attachment to external tools
    * Valgrind, GDB - obvious candidates (instrument to conditionally trigger a breakpoint?)
    * property-checking tools (unsinged over/underflow, no-progress loops, suspicious iteration counts, recording & analysis of comparisons at particular location)
    * GCOV - coverage
* aggregate mode of the tools - run test cases with multiple failing instances to allow cross-checking (by the user)
* custom "log expressions" that would report into the framework? (additional info) 
    * ! compatibility with LLVM IR
```c++
    RET_T foo(PT p1, PT p2) { 

        /* start of static instrumentation */
        
        /* user-provided expression that can be serialized/logged 
           e.g. p1 * p2 / (p1 - p2) / ... */
        /* end of static instrumentation */
        
        // function body...
    }
```
