# IPC experiments

This folder contains various components aimed at implementing a working and viable solution for the problem of IPC between an instrumented application and a "test driver" application.

Aims:

1. receive & process function call information from the instrumented program
2. export of function identifiers selected by the user - these are the targets which we want to test
3. capture of function argument data
4. ability to "send" captured information to the forked tested processes

## Components

* [`ipc-finalizer`](./ipc-finalizer/) - a tiny application aimed at indicating program crashes to the test driver
* [`ipc-hooklib`](./ipc-hooklib/) - hook library that is able to communicate via selected IPC method with the "test driver" (similar to [the first hook library version](../01-llvm-ir/test-pass/hooklib/))
* [`llcap-server`](./llcap-server/) - The "test driver" Rust application, recipient of (function call) information 
* [`example-complex`](./example-complex/) - cmake-managed test "project" to be instrumented (for testing a build system integration as well as multiple-file projects)

