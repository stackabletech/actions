#!/usr/bin/env bash

set -euo pipefail
[ -n "$GITHUB_DEBUG" ] && set -x

ARCH=$(uname -m)

if [ "$INTERU_VERSION" == "latest" ]; then
  curl -fsSL -o /tmp/interu "https://github.com/stackabletech/actions/releases/latest/download/interu-${ARCH}-unknown-linux-gnu"
else
  curl -fsSL -o /tmp/interu "https://github.com/stackabletech/actions/releases/download/interu-${INTERU_VERSION}/interu-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/interu
