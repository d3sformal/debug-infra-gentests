#!/bin/bash
set -e

Out=$(echo "$1" | tail -n+2)
# exception is detected inside the try block, and only inside of it
# we replace 4-th call with index 4 argument (from 5th recorded call)
echo "$Out" | grep "4|4|Exception"

set +e
# no idea why "| not" does not work here
Res=$(echo "$Out" | grep -v "4|4|Exception" | grep "Exception")
if [[ "$Res" != "" ]]
then
  echo "$Res"
  exit 1
fi
