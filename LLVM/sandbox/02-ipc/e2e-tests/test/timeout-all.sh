#!/bin/bash

echo "$1" | tail -n+2 | grep -v "|Timeout"

if [[ "$?" == 0 ]]
then
  exit 1;
fi

exit 0;