#!/bin/bash

# usage
# add_includes.sh file-list ABSOLUTE-path-to-include/funtrace.hpp

cat "$1" | xargs --delimiter=\\n sed -i "1i#include \"$2\""
