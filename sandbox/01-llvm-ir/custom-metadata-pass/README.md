# Custom Metadata-adding plugin/tool

For the AST plugin, head to the [ast-meta-add directory](./ast-meta-add).

## Demonstration

Requires [patched LLVM](../../../README.md#building).

`clang++ -fplugin=../../build/lib/AstMetaAdd.so ./test-program.cpp ./headerimpl.cpp -g -S -emit-llvm`

Creates `test-program.ll` file where you can inspect the custom metadata inserted.

## Core LLVM patches

* [initial patch adding custom metadata](./custom-metadata.diff)
  * (later evolved into [the final patch](./clang-ir-mapping-llvm.diff))
* [clang-llvm argument index mapping insertion](./clang-llvm-map.diff)

## Archived LLVM patches

* [dead-end patch adding parameter attributes](./metadata-and-unsigned-attributes.diff)
  * failure because the attributes do not shift position if AST and IR number of arguments don't match