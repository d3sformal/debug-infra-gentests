#/bin/sh
set -ex

cd ./build
$1 -fPIC ./bitcode.ll ./libmy-hook.so
cd ../