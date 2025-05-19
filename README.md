# Research Project: Locating Bugs in C/C++ Programs by Generating Directed Unit Tests 

> [!note]
> This repo requires up to 2 (**large**) LLVM submodules. To perform a shallow clone, run:

    git clone --recurse-submodules --shallow-submodules

> [!tip]
> Currently, the `llvm-project-cir` submodule is **not** required, you can pull even less data by:

    git submodule update --depth=1 ./sandbox/llvm-project

## Setup & Build

> [!note] Prerequisites: 
> `cmake`, C/C++ toolchain, `ninja`, `xargs`

### Setup LLVM

The following sets up the `ninja` build system:

```sh
cd sandbox/
./setup-llvm-build.sh
```

### Building

```sh
cd sandbox/build
ninja
# add -jN for parallelism or
# ninja -j $(nproc)
```

To install:

> [!Warning]
> I have no robust idea how or if the `ninja install` (installation step) is reversible. I recommend setting up a VM for your testing & development environment. (possible "uninstallation" step is running `xargs rm -rf < install_manifest.txt` in the corresponding build directory)


```sh
sudo ninja install
```

To uninstall:

```sh
xargs rm -rf < install_manifest.txt
```

## Organization

Code style in most sources that `#include` LLVM headers is (auto)formatted by `clangd`.
Other files have no code style enforced (so far). Most of the time, running `cmake ./ && make` should result in a successful build. 
 
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
3. Quick preliminary analysis of a [demangling issue encountered](./notes/0x-llvm-demangling.md)
    * majority was a "quick intro" to mangled symbol "syntax"
    * analysis in the [linked document](./notes/0x-llvm-demangling.md) was performed by human
4. Linking of multiple libraries into one
    * generated `Cmake` examples not functional
5. Discussion of ZMQ's buffer flushing - whether it is possible to force-flush them
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