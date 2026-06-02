#!/usr/bin/env bash

set -euo pipefail
[ -n "$RUNNER_DEBUG" ] && set -x

ARCH=$(uname -m)

if [ "$COMMITTED_VERSION" == "latest" ]; then
  curl -fsSL -o /tmp/committed "https://github.com/crate-ci/committed/releases/latest/download/committed-${ARCH}-unknown-linux-gnu"
else
  curl -fsSL -o /tmp/committed "https://github.com/crate-ci/committed/releases/download/committed-${COMMITTED_VERSION}/committed-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/committed
rm -rf /tmp/committed
