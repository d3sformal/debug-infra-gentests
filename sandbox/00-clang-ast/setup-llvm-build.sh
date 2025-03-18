mkdir build
cd build
# cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=ON -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra" -DCMAKE_BUILD_TYPE=Release -DLLVM_BUILD_TESTS=ON -DCMAKE_C_COMPILER=clang -DCMAKE_CXX_COMPILER=clang++ ../llvm-project/llvm
cmake \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
    -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_C_COMPILER=clang \
    -DCMAKE_CXX_COMPILER=clang++ \
    -DLLVM_BUILD_TESTS=OFF \
    -DLLDB_INCLUDE_TESTS=OFF \
    -DCLANG_INCLUDE_TESTS=OFF \
    -DCLANG_DEFAULT_CXX_STDLIB=libc++ \
    -DLLVM_TARGETS_TO_BUILD=host \
    -DLLVM_ENABLE_RUNTIMES="libcxx;libcxxabi;libunwind" \
    ../llvm-project/llvm
cd ../

# Use configuration
#  files (https://clang.llvm.org/docs/UsersManual.html#configuration-files) to specify the default --gcc-install-dir= or --gcc-triple=

# cmake $cmakeFlags -DUSE_DEPRECATED_GCC_INSTALL_PREFIX=true ../llvm-project/llvm