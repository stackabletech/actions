#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

if [ "$CARGO_AUDITABLE_VERSION" == "latest" ]; then
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/cargo-auditable "https://github.com/rust-secure-code/cargo-auditable/releases/latest/download/cargo-auditable-${ARCH}-unknown-linux-gnu"
else
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/cargo-auditable "https://github.com/rust-secure-code/cargo-auditable/releases/download/cargo-auditable-${CARGO_AUDITABLE_VERSION}/cargo-auditable-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/cargo-auditable
rm -rf /tmp/cargo-auditable
