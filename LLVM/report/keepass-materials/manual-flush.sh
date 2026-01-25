#!/bin/bash
set -eux

cd ../../sandbox/02-ipc/ipc-finalizer
./build.sh

./ipc-fin /llcap-capture-base-semfull 10

# assuming pwd is this file's parent keepassxc is at the relative path ../../../../../keepassxc!
# mkdir ./tmp-tracing && ./make-diffs.sh ./tmp-tracing ../../../../../keepassxc
# ./gui-calltrace.sh ./tmp-tracing ../../../../../keepassxc
# ./gui-capture-test.sh ./tmp-tracing ../../../../../keepassxc

# other useful commands: 
# when keepass complains that only one instance can be running:
# killall -r ".*keepassxc.*" -s KILL
# removing keepassxc/build and rebuilding
# example args (from this directory)

