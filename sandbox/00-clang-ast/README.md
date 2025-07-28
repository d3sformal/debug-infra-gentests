# Clang AST modification

## Setup & build 

* requires the `llvm-project` to be built according to the root [README](../../README.md)

## Contents

> [!note]
> The term `tracing library` refers to a piece of code that enables us to trace function entry & capture the values of simple function paramters. (more or less equivalent to the [project-ubiquitous `hooklib`](../../README.md#hooklib-or-the-hooking-library))

* [`cpy-to-llvm-project`](./cpy-to-llvm-project/)
    * contains clang [ASTMatchers](https://clang.llvm.org/docs/LibASTMatchersReference.html)-based tools
    * is supposed to be copied into the [../llvm-project/clang-tools-extra](../llvm-project/clang-tools-extra/) directory (for easier build - see the root [README](../../README.md) or the [`setup-tool.sh`](./setup-tool.sh) script)
    * [`ast-injection`](./cpy-to-llvm-project/clang-tools-extra/ast-injection/) - initial prototype
    * [`ast-injection-with-lib`](./cpy-to-llvm-project/clang-tools-extra/ast-injection-with-lib/) - a refinement of the initial prototype, exploration of injecting calls to our tracing library

* [`inject-w-library`](./inject-w-library/) - a testing skeleton appliaction where our tracing library could be tested in a clean/simple environment
    * [`lib`](./inject-w-library/lib/) - the (trivial) tracing library that is used throughout the AST approach by linking with it
* [`instr-rapid-iter`](./instr-rapid-iter/) - a rapid-iteration environment that combines AST rewriting and execution of instrumented programs with the tracing library

The clang tools and the skeleton library support `C++` only. For more information, visit the folders themselves.
