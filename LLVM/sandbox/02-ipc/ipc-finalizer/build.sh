#!/bin/sh

cd ../ipc-hooklib/
cmake ./ -DCFG_MANUAL=ON
make
cd ../ipc-finalizer/

cmake ./
make
