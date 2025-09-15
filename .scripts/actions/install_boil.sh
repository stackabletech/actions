#!/usr/bin/env bash

set -euo pipefail
[ -n "$GITHUB_DEBUG" ] && set -x

ARCH=$(uname -m)

if [ "$BOIL_VERSION" == "latest" ]; then
  curl -fsSL -o /tmp/boil "https://github.com/stackabletech/docker-images/releases/latest/download/boil-${ARCH}-unknown-linux-gnu"
else
  curl -fsSL -o /tmp/boil "https://github.com/stackabletech/docker-images/releases/download/boil-${BOIL_VERSION}/boil-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/boil
