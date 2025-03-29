# Research Project

**This repo requires LLVM (large submodule)**, to perform a shallow clone, run:

    git clone --recurse-submodules --shallow-submodules

## Setup & Build

### Setup LLVM

```sh
cd sandbox/00-clang-ast/
# optionally for AST modification demonstration
# ./setup-tool.sh
./setup-llvm-build.sh
```


### Building


```sh
cd sandbox/00-clang-ast/build
ninja
# if you need to install LLVM again
# sudo ninja install
```

Or build & install via `sandbox/00-clang-ast/build.sh`

### Re-building, modification

Perform those inside `build` folder.
Assumes setup.

**To (re)generate makefiles via CMake**

```sh
cmake \
    -G Ninja ../llvm-project/llvm \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
    -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra" \
    -DCMAKE_BUILD_TYPE=Release \
    -DLLVM_BUILD_TESTS=OFF \
    -DLLDB_INCLUDE_TESTS=OFF \
    -DCLANG_INCLUDE_TESTS=OFF \
    -DLLVM_TARGETS_TO_BUILD=host \
    -DLLVM_INSTALL_UTILS=ON \
    -DLLVM_ENABLE_DUMP=ON
```

## Organization

Code style in most sources that `#include` LLVM headers is (auto)formatted by `clangd`.
Other files have no code style enforced (so far).
 
Folder naming: 

* `working` - a "working folder", a place where most of commands are executed / most files are being changed, gitignored, ...
* `build` - most output artifacts will end up here


1. `notes` subdirectory generally unorganized notes
    * `00-paper-notes` - related papers
    * `00-testing-alternatives` - bulletpoint-style thoughts on, pros/cons of and issues with various methods 
2. `sandbox`
    * `00-clang-ast` - explores source-level modification of the Clang's AST
        * modify source code by inspecting and rewriting the AST
        * recompile modified source code with an instrumentation library
    * `01-llvm-ir` - explores LLVM IR modification
        * compile source into LLVM Bitcode
        * inspect and modify generated Bitcode by adding instructions (mostly just calls into an instrumentation library functions)
        * compile modified Bitcode with an instrumentation library
        * commands and results in `sandbox/01-llvm-ir/test-pass/working` 
