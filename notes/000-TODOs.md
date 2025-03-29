# TODO:

* prepare examples for function tracing porototype
* prepare examples for parameter capture prototype
* think about non-deterministic traces
    * do we permit relying on function call determinism?
* think about C++ objects - capturing inside of them, this, ...

* ??? inserting log expressions that would report into the framework? (additional info) 
```c++
    RET_T foo(PT p1, PT p2) { 
        /* start of static instrumentation */
        /* user-provided expression that can be serialized/logged */
        // e.g. p1 * p2 / (p1 - p2) / ...
        /* end of static instrumentation */
        // body...
    }
```
# TOPIC: Data Capture Library

## Capturing funciton arguments

* how to treat **templated** code
    * let `X` be the set of types allowed for recording:
    * `template <class T>`, parameter of type `T`, where all instantiations of `T` in `X` - should be OK with naive source modification
    * not all instantiations have `T` in `X` -> compilation errors
        * how to resolve?
        * can clang provide per-instantiation decisionmaking? how will that affect compile times/feasability?

* how to treat type aliases?
    * should be doable via clang's APIs (the underlying type should be resolved, right?)

## Capturing execution trace 

* use other tools?

* C vs C++ function scope tracking
    * for now I only focus on C++ code, this method should allow tracing of even code which throws exceptions
    * C has no "reasonably non-colliding" namespaces
    * on the other hand: no annoying exceptions

* use of C++'s `auto` keyword
    * limits C heavily (`auto` not used widely)
    * intricacies of C++'s type system & reference semantics
        * ensuring that modified source code makes exactly the same side effects as the unmodified one

# TOPIC: C++ support

* methods of objects (TODO)

# TOPIC: LLVM IR approach

* shows more promise than AST modification
* trouble with filtering (non) library funcitons - information not available in the IR
    * **Idea**: add metadata to the functions in the IR that could tell the LLVM pass if the function is "#include"d, library, intrinsic, ...
        * [LLVM Discussion](https://discourse.llvm.org/t/how-to-distinguish-between-user-defined-function-in-a-program-and-library-functions/54401/7)

