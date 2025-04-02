# Clang AST modification

## Setup & build 

* see the root [README](../../README.md)

* the setup of [`llvm-project`](./llvm-project/) is **also** needed for other parts of the project

## Contents

The term `tracing library` refers to a piece of code that enables us to trace function entry & capture teh values of simple function paramters.

* [`cpy-to-llvm-project`](./cpy-to-llvm-project/)
    * contains clang `ASTMatchers`-based tools
    * is supposed to be copied into the `llvm-project/clang-tools-extra` directory (for easier build - see the root [README](../../README.md) or the [`setup-tool.sh`](./setup-tool.sh) script)
    * [`ast-injection`](./cpy-to-llvm-project/clang-tools-extra/ast-injection/) - initial prototype
    * [`ast-injection-with-lib`](./cpy-to-llvm-project/clang-tools-extra/ast-injection-with-lib/) - a refinement of the initial prototype, exploration of injecting calls to our tracing library

* [`inject-w-library`](./inject-w-library/) - a testing skeleton appliaction where our tracing library could be tested in a clean/simple environment
    * [`lib`](./inject-w-library/lib/) - the (trivial) tracing library that is used throughout the AST approach by linking with it
* [`instr-rapid-iter`](./instr-rapid-iter/) - a rapid-iteration environment that combines AST rewriting and execution of instrumented programs with the tracing library

The clang tools and the skeleton library support `C++` only. For more information, visit the folders themselves.
