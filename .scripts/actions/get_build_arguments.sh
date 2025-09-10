#!/usr/bin/env bash

# Splits comma-separated values, prepends them with --build-arg, and prints
# them out. Variable values MUST mot contain any whitespace as following commands
# will break.
echo "$1" | awk '{ split($0, args, ","); for (i in args) { printf "--build-arg %s ", args[i] }}'
