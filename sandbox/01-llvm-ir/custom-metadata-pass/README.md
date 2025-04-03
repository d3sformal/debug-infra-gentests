# Custom Metadata-adding clang plugin

`clang++ -fplugin=../../build/lib/AstMetaAdd.so ./test-program.cpp ./headerimpl.cpp -g -S -emit-llvm`

- `custom-metadata.diff`
