#!/bin/bash
set -eux

path=$(pwd)
fixpath=$path/$1; shift
keepass=$path/$1; shift

cd ../../sandbox/02-ipc/llcap-server

modmaps="$fixpath/modmaps"
selection="$fixpath/selection.bin"
capture="$fixpath/kpass-trcs-dir"
clibin="$keepass/build/src/cli/keepassxc-cli"

cargo r --release -- --modmap "$modmaps" test -s "$selection" -c "$capture" "$clibin" generate --length 13

