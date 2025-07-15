#!/bin/sh
set -e

# Args: <tested binary directory> <function name> <timeout in seconds> <output-testing script/directory> <clang args>
WorkingDir=$1; shift
TestedFnName=$1; shift
TimeoutSec=$1; shift
OutputTestScriptDir=$1; shift
CppArgs=$*;

cd "$WorkingDir"
WorkingDir=$(pwd)
BuildDir="$WorkingDir"/build
OutputsDir="$WorkingDir"/out
rm -rf "$BuildDir"
mkdir "$BuildDir"
mkdir -p "$OutputsDir"

LlcapSvrBin="$WorkingDir/../../llcap-server/target/debug/llcap-server"
InstrumentedBin="$BuildDir"/testbin

cp ../../../01-llvm-ir/llvm-pass/libfn-pass.so "$BuildDir"
cd ../../ipc-hooklib

cmake ./ -DCFG_MANUAL=OFF
make

cd "$BuildDir"

cmake -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  ../

cmake   -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  -D CMAKE_CXX_FLAGS="-mllvm -Call -Xclang -load -Xclang ./libfn-pass.so -Xclang -fpass-plugin=./libfn-pass.so -fplugin=/usr/local/lib/AstMetaAdd.so" \
  ../

rm "$BuildDir"/module-maps/* || true
mkdir -p "$BuildDir"/module-maps
make clean
make

SelectionPath="$OutputsDir"/selected-fns.bin
ModMapsPath="$BuildDir"/module-maps/

echo "!!! Tracing"
echo "N:$TestedFnName" | "$LlcapSvrBin" --modmap "$ModMapsPath" trace-calls -o "$SelectionPath" "$InstrumentedBin"

echo "!!! Rebuilding"

cd "$WorkingDir"

cd ../../ipc-hooklib
cmake ./ -DCFG_MANUAL=OFF
make
cd "$BuildDir"

cmake -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  ../

cmake   -D CMAKE_C_COMPILER=clang \
  -D CMAKE_CXX_COMPILER=clang++ \
  -DCMAKE_CXX_FLAGS="$CppArgs -mllvm -llcap-verbose -mllvm -Arg -mllvm -llcap-fn-targets-file=$SelectionPath -Xclang -load -Xclang ./libfn-pass.so -Xclang -fpass-plugin=./libfn-pass.so -fplugin=/usr/local/lib/AstMetaAdd.so"  \
  ../

make clean
make

ARG_TRACES="$OutputsDir"/arg-traces-dir
TestOutputsDir="$OutputsDir"/test-outputs

echo "!!! Capturing"

rm -rf "$ARG_TRACES" && "$LlcapSvrBin" --modmap "$ModMapsPath"\
 capture-args -s "$SelectionPath" -o "$ARG_TRACES" "$InstrumentedBin"

echo "!!! Testing"

mkdir -p "$TestOutputsDir"

Output=$(rm -rf "${TestOutputsDir:?}"/* && "$LlcapSvrBin" --modmap "$ModMapsPath"\
 test -s "$SelectionPath" -t "$TimeoutSec" -c "$ARG_TRACES"\
 -o "$TestOutputsDir" "$InstrumentedBin")

# transform output to a |-separated table with a single-line header
# ModuleID|FunctionID|Call|Packet|Result
Output=$(echo "$Output" | cut -d']' -f 2- | grep ".*|.*|.*" | tr -d '[:blank:]')


set -e
if [[ "$OutputTestScriptDir" != "" ]]
then
  echo "!!! Testing Outputs"
  # go through all files and execute them
  if [ -d "$OutputTestScriptDir" ];
  then
    for File in "$OutputTestScriptDir"/tc-*.sh; do
      echo -n "Test $File "
      "$File" "$Output" &1>/dev/null
      echo "OK"
    done
  else
    "$OutputTestScriptDir" "$Output"
  fi
else
  echo "!!! Done"
  echo "$Output"
fi
