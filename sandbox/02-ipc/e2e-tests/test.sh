#!/bin/sh
set -e

# Args: <tested binary directory> <index of the function> <timeout in seconds>
DIR=$1; shift
FN_IDX=$1; shift
TIMEOUT_S=$1; shift
CXX_ARGS=$*;

cd "$DIR"
DIR=$(pwd)
BUILD_DIR="$DIR"/build
OUTS_DIR="$DIR"/out
rm -rf "$BUILD_DIR"
mkdir "$BUILD_DIR"
mkdir -p "$OUTS_DIR"

DIR_RELATIVE_LLCAP_SERVER="../../../llcap-server"
LLCAP_BIN="$DIR_RELATIVE_LLCAP_SERVER"/target/debug/llcap-server
TEST_BINARY="$BUILD_DIR"/testbin

cp ../../../01-llvm-ir/llvm-pass/libfn-pass.so "$BUILD_DIR"
cd ../../ipc-hooklib

cmake ./ -DCFG_MANUAL=OFF
make

cd "$BUILD_DIR"

cmake -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  ../

cmake   -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  -D CMAKE_CXX_FLAGS="-mllvm -Call -Xclang -load -Xclang ./libfn-pass.so -Xclang -fpass-plugin=./libfn-pass.so -fplugin=/usr/local/lib/AstMetaAdd.so" \
  ../

rm "$BUILD_DIR"/module-maps/* || true
mkdir -p "$BUILD_DIR"/module-maps
make clean
make

SELECTION="$OUTS_DIR"/selected-fns.bin
MODMAPS="$BUILD_DIR"/module-maps/

echo "!!! Tracing"
echo "$FN_IDX" | "$LLCAP_BIN" --modmap "$MODMAPS" trace-calls -o "$SELECTION" "$TEST_BINARY"

echo "!!! Rebuilding"

cd "$DIR"

cd ../../ipc-hooklib
cmake ./ -DCFG_MANUAL=OFF
make
cd "$BUILD_DIR"

cmake -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  ../

cmake   -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  -DCMAKE_CXX_FLAGS="$CXX_ARGS -mllvm -llcap-verbose -mllvm -Arg -mllvm -llcap-fn-targets-file=$SELECTION -Xclang -load -Xclang ./libfn-pass.so -Xclang -fpass-plugin=./libfn-pass.so -fplugin=/usr/local/lib/AstMetaAdd.so"  \
  ../

make clean
make

ARG_TRACES="$OUTS_DIR"/arg-traces-dir
TEST_OUTPUT_DIR="$OUTS_DIR"/test-outputs

echo "!!! Capturing"

rm -rf "$ARG_TRACES" && "$LLCAP_BIN"\
 --modmap "$MODMAPS" capture-args -s "$SELECTION" -o "$ARG_TRACES" "$TEST_BINARY"

echo "!!! Testing"

mkdir -p "$TEST_OUTPUT_DIR"

OUTPUT=$(rm -rf "${TEST_OUTPUT_DIR:?}"/* && "$LLCAP_BIN"\
 --modmap "$MODMAPS" test -s "$SELECTION" -t "$TIMEOUT_S" -c "$ARG_TRACES"\
     -o "$TEST_OUTPUT_DIR" "$TEST_BINARY")

# transform output to a |-separated table with a single-line header
# ModuleID|FunctionID|Call|Packet|Result
OUTPUT=$(echo "$OUTPUT" | cut -d']' -f 2- | grep ".*|.*|.*" | tr -d '[:blank:]')

echo "!!! Done"
echo "$OUTPUT"

