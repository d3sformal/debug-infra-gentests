#!/bin/sh
set -x

cd ../hooklib
cmake ./
make -j4
cp ./libmy-hook.so ../working/build/
