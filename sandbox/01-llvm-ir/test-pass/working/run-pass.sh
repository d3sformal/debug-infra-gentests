cp ../"$2" ./
mkdir -p ./build
$1 -fpass-plugin=../../llvm-pass/libfn-pass.so "$2" -S -emit-llvm -o ./build/bitcode.ll
