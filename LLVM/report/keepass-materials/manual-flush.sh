#!/bin/bash
set -eux

cd ../../sandbox/02-ipc/ipc-finalizer
./build.sh

./ipc-fin /llcap-capture-base-semfull 10

