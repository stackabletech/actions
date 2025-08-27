#!/usr/bin/env bash

# Splits comma-separated values, prepends them with --build-arg, and prints
# them out separated by newlines.
echo "$1" | awk -F',' '{ split($0, args, ","); for (i in args) { printf "--build-arg %s\n", args[i] }}'
