#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

if [ "$INTERU_VERSION" == "latest" ]; then
  curl -o /tmp/interu "https://github.com/stackabletech/actions/releases/latest/download/interu-${ARCH}-unknown-linux-gnu"
else
  curl -o /tmp/interu "https://github.com/stackabletech/actions/releases/download/interu-${INTERU_VERSION}/interu-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/interu
rm -rf /tmp/interu
