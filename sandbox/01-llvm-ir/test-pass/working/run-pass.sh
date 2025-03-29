cp ../test-program.cpp ./
clang++ -fpass-plugin=../../llvm-pass/libfn-pass.so ./test-program.cpp -S -emit-llvm -o ./bitcode.ll
