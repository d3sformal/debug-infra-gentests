# AST Metadata plugin

This plugin (compiled as a `clang` tool) injects custom metadata while walking and inspecting the AST of a program.

We later use that metadata to detect some function and parameter features that are otherwise more or less deducible from the IR. The metadata keys used by this plugin ([`llvm-metadata.h`](./llvm-metadata.h)) are also included by the [`llvm-pass`](../../../01-llvm-ir/llvm-pass/).

This plugin requires the [LLVM patch](../clang-llvm-map.diff).

## Building

For some reason, I was unable to build this component completely independently of the `llvm-project` file structure. Building requires moving this folder into `llvm-project/clang/examples/` and adding it into the parent folder's `CMakeLists.txt` file.

These one-time steps can be executed via the [`../setup-tool.sh`](../setup-tool.sh) script.

 cd ../
 ./setup-tool.sh

To build the plugin, you must "rebuild" `llvm-project`. Luckily, `ninja` will take care of the
change tracking and thus only the `AstMetaAdd.cpp` file will be compiled.

 cd ../../../build
 ninja
 sudo ninja install

The installation steps may be required since some scripts rely on the installed path of the plugin.
Otherwise, you can find the plugin on the `build/lib/AstMetaAdd.so` path.

## How it works

We recursively inspect every *declaration*, check for *function* declarations, and for *lambdas*.
If we encounter a *namespace declaration*, we recurse. For *lambda expressions*, we inspect their `operator()` as *function declarations*.

In a *function declaration*, we add metadata only for functions that are not inside a "system header" or similar. This marks functions that are "normally visible" in the project's source code and avoids, e.g., the standard library functions. Apart from the "not system header" marker, we inject metadata informing the IR pass about the presence of the `this` pointer, positions of `std::string` parameters, as well as positions of `unsigned` parameters (currently a bit redundant).

In the code, take a look at `AddMetadataConsumer` class and `addFunctionMetadata` function.
 