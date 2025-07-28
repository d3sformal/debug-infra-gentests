# IPC experiments

This folder contains various components aimed at implementing a working and viable solution for the problem of IPC between an instrumented application and the [`llcap-server`](../../README.md#llcap-server) application.

Aims:

1. receive & process function call information from the instrumented program
2. export of function identifiers selected by the user - these are the targets which we want to test
3. capture of function argument data
4. ability to "send" captured information to the forked tested processes

## Components

* [`ipc-finalizer`](./ipc-finalizer/) - a tiny application aimed at indicating program crashes to the `llcap-server` - currently the call-tracing phase does not monitor the instrumented application
* [`ipc-hooklib`](./ipc-hooklib/) - [hook library](../../README.md#hooklib-or-the-hooking-library) that is able to communicate via selected IPC method with the `llcap-server` (vaguely similar to [the first hook library version](../01-llvm-ir/test-pass/hooklib/))
    * currently it uses shared memory, UNIX domain sockets and `socketpair` for all necessary communication
* [`llcap-server`](./llcap-server/)
    * receives function call information
    * captures arguments of selected functions
    * supplies captured arguments to the application (via the `ipc-hooklib`)
* [`example-complex`](./example-complex/) - cmake-managed test "project" to be instrumented (for testing a build system integration as well as multiple-file projects)

## Demo

We provide the podman-backed example in the [`example-arg-replacement`](./example-arg-replacement) directory.

Each demo requires 2 recomplilations of the application - one for function call tracing and the other for all the remaining functionality. 

There is also an example program contained in [`example-complex`](./example-complex/). It accepts arguments which "crash" the program in different parts of its execution. This can help simulate real program crashes as well as limit the amount of data captured/printed in the demo.

You can run that demo by using the [`build-call-trace.sh`](./example-complex/build-call-trace.sh) and [`build-arg-trace.sh`](./example-complex/build-arg-trace.sh) scripts along with `llcap-server` as seen in the [`example-arg-replacement` example](./example-arg-replacement/README.md).
