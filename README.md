# Research Project: Locating Bugs in C/C++ Programs by Generating Directed Unit Tests 

> [!note]
> This repo requires a (**large**) LLVM submodule. To perform a shallow clone, run:

    git submodule update --depth=1 ./sandbox/llvm-project

## Motivation, context

Suppose we develop a program that contains a bug which causes a crash. In this project, we
try to create tools to help investigate root causes of such bugs by automatically testing the
target program. The tools and plugins developed as a part of this project aim to assist in automating the following:

* capture of calls to non-builtin/library/external functions
* capture of argument values of user-specified functions
* hijacking of the arguments of the user-specified functions
* monitoring of the hijacked program (where the possibly bug-triggering argument sets will stand out)

## Demo

We provide a complete demo in the form of a small C++ program inside the [example-arg-replacement](./sandbox/02-ipc/example-arg-replacement) directory.

## Containers

Demo container is available on [Docker Hub](https://https://hub.docker.com/r/vasutro/llcap-demo-env) (est. 650MB download):

        podman run -it docker.io/vasutro/llcap-demo-env:1.0.2

Last commit of this repository used to build the container and test the demo: 

`8bb815496881db629e3ba5e811d2d719532f3e0c`

For usage, refer to the [example-arg-replacement](./sandbox/02-ipc/example-arg-replacement) directory.

For container internals & development container information, see [podman](./podman/) directory.

## Setup & Build

To **use** the tools, the only thing required is to use our ([patched version of LLVM/clang](./sandbox/01-llvm-ir/clang-ir-mapping-llvm.diff)) (you can use our [demo container](#containers))  and adjusting your build tools to use the patched compiler
and pass [additional](./sandbox/02-ipc/example-arg-replacement/build-arg-trace.sh) [arguments](./sandbox/02-ipc/example-arg-replacement/build-call-trace.sh) to it.

To **build** the tools:

> [!note] Prerequisites: 
> `git`, `cmake`, C/C++ toolchain, `ninja`, `xargs`

### Setup LLVM

```sh
git submodule update --init --depth=1 ./sandbox/llvm-project
# apply our LLVM patch
cd ./sandbox/llvm-project
git apply ../01-llvm-ir/clang-ir-mapping-llvm.diff
cd ../../
```

The following sets up LLVM for the `ninja` build system:

```sh
cd sandbox/
./setup-llvm-build.sh
cd ../
```

### Building

```sh
# this step takes A LONG TIME (and GBs of disk space)
cd sandbox/build
ninja
# add -jN for parallelism or
# ninja -j $(nproc)
```

To install:

> [!Warning]
> the `ninja install` step may not be totally reversible! We recommend setting up a VM for your testing & development environment

```sh
sudo ninja install
# do not clean the build directory yet!
```

We can now build the AST plugin:

```sh
cd ../../
cd ./sandbox/01-llvm-ir/custom-metadata-pass
./setup-tool.sh
cd ../../build

# build and install again (only builds the small AST plugin)
ninja -j $(nproc)
sudo ninja install
```

To uninstall:

```sh
xargs rm -rf < install_manifest.txt
```

Next, you will also need to build (commands shall be executed in the tools' subdirectories):
* [LLVM pass plugin](./sandbox/01-llvm-ir/llvm-pass/) (**depends on** `llvm-project`) - `cmake ./ && make` 
* [hook library](./sandbox/02-ipc/ipc-hooklib/) - `cmake ./ && make` (independent)
* [`llcap-server`](./sandbox/02-ipc/llcap-server/) - `cargo b` or `cargo b --release` (independednt)

## Organization

Code style in most sources that `#include` LLVM headers is (auto)formatted by `clangd`.
Other files have no code style enforced (unless a `.clang-tidy` file is present). Most of the time, running `cmake ./ && make` should result in a successful build. 
 
Folder naming:

* `working` - a "working folder", a place where most of commands are executed / most files are being changed, gitignored, ...
* `build*` - most output artifacts will end up here


1. [`notes`](./notes/) subdirectory generally unorganized notes
    * [`00-paper-notes`](./notes/00-paper-notes.md) - related papers
    * [`00-testing-alternatives`](./notes/00-testing-alternatives.md) - bulletpoint-style thoughts on, pros/cons of and issues with various methods 
2. [`sandbox`](./sandbox/)
    * [`00-clang-ast`](./sandbox/00-clang-ast/) - explores source-level modification of the Clang's AST
        * idea: 
            * modify source code by inspecting and rewriting the AST
            * recompile modified source code with an instrumentation library
    * [`01-llvm-ir`](./sandbox/01-llvm-ir/) - explores LLVM IR modification
        * idea:
            * compile source into LLVM Bitcode
            * inspect and modify generated Bitcode by adding instructions (mostly just calls into instrumentation library functions)
            * compile modified Bitcode with an instrumentation library
    * [`02-ipc`](./sandbox/02-ipc/) - next stage -  experiments based on IPC
        * idea:
            * we need to extract data from the instrumented application
            * instrumentation library's responsibility is to establish connections to "us" and send "us" data
            * also serves as a preparation for the final stage: executing targeted unit tests (i.e. sending data from "us" to unit tests - input data into cloned process)


# AI/LLM usage disclosure

LLMs have been used to consult on **exactly and only** these topics:

## AI/LLM usage that was NOT used

1. Brief overview of the capacities of LLVM to transfer information between individual LLVM passes
    * dead end
2. Quick preliminary analysis of a [demangling issue encountered](./notes/0x-llvm-demangling.md)
    * majority was a "quick intro" to mangled symbol "syntax"
    * analysis in the [linked document](./notes/0x-llvm-demangling.md) was performed by human
3. Linking of multiple libraries into one
    * generated `Cmake` examples not functional
4. Discussion of ZMQ's buffer flushing - whether it is possible to force-flush them
    * last resort option used after studying documentation & searching internet forums
    * results *not used*

## AI/LLM usage used directly in this project

1. Generation of boilerplate code to set up a LLVM pass
    * < 25 lines of code artifacts used, combined with official documentation
    * usage of `llvmGetPassPluginInfo`, `registerPipelineStartEPCallback` in [the LLVM pass](./sandbox/01-llvm-ir/llvm-pass/src/pass.cpp)
2. Translation of CMake directives (in `CMakeLists.txt`) specifying compile time options for LLVM plugins (`-mllvm`) into `cmake` options supplied on the command line
    * plus a discussion of possibilities regarding individual-file setting of such flags (instead of project-wide scope) - *not used*
3. Parsing numbers in C++ without exceptions (e.g. without `std::stoi`, ...)
    * only used as a reference point, usage of `std::from_chars` in this codebase is derived from official documentation