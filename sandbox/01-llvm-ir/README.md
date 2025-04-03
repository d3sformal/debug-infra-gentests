# LLVM IR approach

We attempt to instrument the LLVM IR with calls to a tracing library. This has the advantage of decoupling from the source language. It also means that our tracing library should be compiled
as a C library (or that we have to deal with C++ mangling when calling the library).

## Setup & build 

* requires the `llvm-project` to be built according to the root [README](../../README.md)

## Structure

* [`llvm-pass`](./llvm-pass/)
    * contains a customized LLVM pass that goes through the IR and modifies it
    * compiled as a static library to be passed later to the LLVM toolchain
* [`test-pass`](./test-pass/)
    * a playground for testing the outputs of the custom LLVM pass
    * Contents: 
        * C and C++ `test-program`s containing various language constructs whose IR will be instrumented by the LLVM pass
        * [`hooklib`](./test-pass/hooklib/) directory - a simple shared library that the LLVM pass ([`llvm-pass`](./llvm-pass/)) depends on (the instrumentation relies on this library - injects function calls that will have to be resolved)
        * [`working`](./test-pass/working/) directory - end-to-end build & run "environment" for the insturmentation, please refer to the [`HOWTO`](./test-pass/working/HOWTO.md) file for setup & examples
