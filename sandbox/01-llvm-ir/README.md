# LLVM IR approach

We attempt to instrument the LLVM IR with calls to a tracing library ([`hooklib`](../../README.md#hooklib-or-the-hooking-library)). This has the advantage of decoupling from the source language. It also means that our tracing library should be compiled as a C-compatible library (or that we have to deal with C++ mangling when calling the library).

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
        * [`hooklib`](./test-pass/hooklib/) directory - a simple shared library that the [`LLVM pass`](./llvm-pass/) depends on (the instrumentation injects function calls to the functions of this library)
        * [`working`](./test-pass/working/) directory - end-to-end "build & run environment" for the insturmentation, please refer to the [`HOWTO`](./test-pass/working/HOWTO.md) file for setup & examples
* [`custom-metadata-pass`](./custom-metadata-pass/)
    * contains a [`llvm-project` **patch**](./custom-metadata-pass/custom-metadata.diff) adding custom metadata functionality to the `FunctionDecl` class (later evolved into [the final patch](./clang-ir-mapping-llvm.diff))
        * allows setting key-value *string* pairs that are available in the LLVM IR represenation
        * the final patch is required & forces recompilation of at least 1500 files, **be sure to apply it** before attempting to run related scripts (usually they have `meta` or `meta-plugin` in their name)
    * contains [a clang AST plugin](./custom-metadata-pass/ast-meta-add/README.md) that modifies the metadata based on the AST walkthrough
