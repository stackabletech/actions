#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

if [ "$BOIL_VERSION" == "latest" ]; then
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/boil "https://github.com/stackabletech/docker-images/releases/latest/download/boil-${ARCH}-unknown-linux-gnu"
else
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/boil "https://github.com/stackabletech/docker-images/releases/download/boil-${BOIL_VERSION}/boil-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/boil
rm -rf /tmp/boil
