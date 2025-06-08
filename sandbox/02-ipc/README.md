# IPC experiments

This folder contains various components aimed at implementing a working and viable solution for the problem of IPC between an instrumented application and a "test driver" application.

Aims:

1. receive & process function call information from the instrumented program
2. export of function identifiers selected by the user - these are the targets which we want to test
3. capture of function argument data
4. ability to "send" captured information to the forked tested processes

## Components

* [`ipc-finalizer`](./ipc-finalizer/) - a tiny application aimed at indicating program crashes to the "test driver" - currently the call-tracing phase does not monitor the instrumented application
* [`ipc-hooklib`](./ipc-hooklib/) - hook library that is able to communicate via selected IPC method with the "test driver" (vaguely similar to [the first hook library version](../01-llvm-ir/test-pass/hooklib/))
    * currently it uses shared memory, UNIX domain sockets and `socketpair` for all necessary communication
* [`llcap-server`](./llcap-server/) - The "test driver" Rust application
    * receives function call information
    * captures arguments of selected functions
    * supplies captured arguments to the application (via the `ipc-hooklib`)
* [`example-complex`](./example-complex/) - cmake-managed test "project" to be instrumented (for testing a build system integration as well as multiple-file projects)

## Demo

The full demo requires 2 recomplilations of the application - one for function call tracing and the other for all the remaining functionality. The example program contained in [`example-complex`](./example-complex/) accepts arguments which "crashes" the program in different parts of its execution. This can help simulate real program crashes as well as limit the amount of data captured/printed in the demo.

### Call tracing

Requires the `AstMetaAdd` plugin (by [building LLVM](../../README.md#building)), as well as the [`llvm-pass`](../01-llvm-ir/llvm-pass/).

To capture function calls from the `example-complex` application, perform in one terminal (**1**):
    
    # generates module mapping
    cd ./example-complex
    ./build-call-trace.sh

In another (or the same) terminal (inside this folder) (**2**):

    # runs the capturing "server"
    cd ./llcap-server
    cargo r -- -vv --modmap ../example-complex/module-maps/ trace-calls

Back in the first terminal (**3**):

    # runs the target binary
    ./example

### Argument capture

Argument capture only needs recompilation of arguments which is based on the **function selection created in the previous step** (path where selection was saved will be denoted `SEL_PATH` from now on):

    cd ./example-complex
    # instruments the target for argument tracing
    ./build-arg-trace.sh SEL_PATH
    ./example

Then, choose any non-existent directory name (denoted `CAP_PATH`)

    cd ../llcap-server
    cargo r -- -vv --modmap ../example-complex/module-maps/ capture-args -s SEL_PATH -o CAP_PATH ../example-complex/example

After executing this command, the `CAP_PATH` should contain folders corresponding to module identifiers containing files corresponding to the (selected) function IDs within that module.

### Test execution

To run tests based on recorded data, run `llcap-server` thus:

        cargo r -- -vv --modmap ../example-complex/module-maps/ test -s SEL_PATH -c CAP_PATH ../example-complex/example
