# LLVM IR approach

We attempt to instrument the LLVM IR with calls to a tracing library. This has the advantage of decoupling from the source language. It also means that our tracing library should be compiled
as a C library (or that we have to deal with C++ mangling when calling the library).

## Setup & build 

* requires the `llvm-project` to be built according to the root [README](../../README.md)

## Structure

* [`llvm-pass`](./llvm-pass/)
    * contains a customized LLVM pass that goes through the IR and modifies it
    * compiled as a static library to be passed later to the LLVM toolchain
    * arguments are passed in the following way: `-mllvm -the-llvm-pass-arg` (see comments in [`pass.cpp`](./llvm-pass/src/pass.cpp) or search `-mllvm` throughout the repository)
* [`test-pass`](./test-pass/)
    * a playground for testing the outputs of the custom LLVM pass
    * Contents: 
        * C and C++ `test-program`s containing various language constructs whose IR will be instrumented by the LLVM pass
        * [`hooklib`](./test-pass/hooklib/) directory - a simple shared library that the LLVM pass ([`llvm-pass`](./llvm-pass/)) depends on (the instrumentation relies on this library - injects function calls that will have to be resolved)
        * [`working`](./test-pass/working/) directory - end-to-end build & run "environment" for the insturmentation, please refer to the [`HOWTO`](./test-pass/working/HOWTO.md) file for setup & examples
* [`custom-metadata-pass`](./custom-metadata-pass/)
    * contains a `llvm-project` **patch** that adds custom metadata functionality to the `FunctionDecl` class
        * allows setting key-value string pairs that are written through to the LLVM IR
        * this patch is required & forces recompilation of at least 1500 files, **be sure to apply this patch** before attempting to run related scripts (usually they have `meta` or `meta-plugin` in their name) 
    * contains a clang plugin that modifies the metadata based on the AST walkthrough
        * so far just adds `custom key`-`dummy value` pair to the metadata that indicates whether the function is **not** in a system header file (as opposed to analyzing name mangling)
