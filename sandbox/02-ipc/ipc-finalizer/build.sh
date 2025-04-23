#!/bin/sh

cd ../ipc-hooklib/
cmake ./
make
cd ../ipc-finalizer/

cmake ./
make
