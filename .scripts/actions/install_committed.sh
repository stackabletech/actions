#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"
mkdir /tmp/committed

curl -o /tmp/committed/committed.tar.gz "https://github.com/crate-ci/committed/releases/download/${COMMITTED_VERSION}/committed-${COMMITTED_VERSION}-${ARCH}-unknown-linux-musl.tar.gz"

tar --directory="/tmp/committed" -zxvf /tmp/committed/committed.tar.gz ./committed
sudo install -m 755 -t /usr/local/bin /tmp/committed/committed
rm -rf /tmp/committed
