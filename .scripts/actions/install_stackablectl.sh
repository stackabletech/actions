#!/usr/bin/env bash

set -euo pipefail
[ -n "$GITHUB_DEBUG" ] && set -x

ARCH=$(uname -m)

if [ "$STACKABLECTL_VERSION" == "latest" ]; then
  curl -fsSL -o /tmp/stackablectl "https://github.com/stackabletech/stackable-cockpit/releases/latest/download/stackablectl-${ARCH}-unknown-linux-gnu"
else
  curl -fsSL -o /tmp/stackablectl "https://github.com/stackabletech/stackable-cockpit/releases/download/stackablectl-${STACKABLECTL_VERSION}/stackablectl-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/stackablectl
