#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

if [ "$STACKABLECTL_VERSION" == "latest" ]; then
  curl -o /tmp/stackablectl "https://github.com/stackabletech/stackable-cockpit/releases/latest/download/stackablectl-${ARCH}-unknown-linux-gnu"
else
  curl -o /tmp/stackablectl "https://github.com/stackabletech/stackable-cockpit/releases/download/stackablectl-${STACKABLECTL_VERSION}/stackablectl-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/stackablectl
rm -rf /tmp/stackablectl
