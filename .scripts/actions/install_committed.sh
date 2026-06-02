#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
mkdir /tmp/committed

if [ "$COMMITTED_VERSION" == "latest" ]; then
  curl -fsSL -o /tmp/committed/committed.tar.gz "https://github.com/crate-ci/committed/releases/latest/download/committed-${ARCH}-unknown-linux-gnu.tar.gz"
else
  curl -fsSL -o /tmp/committed/committed.tar.gz "https://github.com/crate-ci/committed/releases/download/committed-${COMMITTED_VERSION}/committed-${ARCH}-unknown-linux-gnu.tar.gz"
fi

tar --directory="/tmp/committed" -zxvf /tmp/committed/committed.tar.gz ./committed
sudo install -m 755 -t /usr/local/bin /tmp/committed/committed
rm -rf /tmp/committed
