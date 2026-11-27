#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

if [ "$CARGO_CYCLONEDX_VERSION" == "latest" ]; then
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/cargo-cyclonedx "https://github.com/CycloneDX/cyclonedx-rust-cargo/releases/latest/download/cargo-cyclonedx-${ARCH}-unknown-linux-gnu"
else
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/cargo-cyclonedx "https://github.com/CycloneDX/cyclonedx-rust-cargo/releases/download/cargo-cyclonedx-${CARGO_CYCLONEDX_VERSION}/cargo-cyclonedx-${ARCH}-unknown-linux-gnu"
fi

sudo install -m 755 -t /usr/local/bin /tmp/cargo-cyclonedx
rm -rf /tmp/cargo-cyclonedx
