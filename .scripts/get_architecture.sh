#!/usr/bin/env bash

uname -m | sed -e "s#x86_64#amd64#" | sed -e "s#aarch64#arm64#"
