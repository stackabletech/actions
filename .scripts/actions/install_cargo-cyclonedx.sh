#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"
mkdir /tmp/cargo-cyclonedx

if [ "$CARGO_CYCLONEDX_VERSION" == "latest" ]; then
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/cargo-cyclonedx/cargo-cyclonedx.tar.xz "https://github.com/CycloneDX/cyclonedx-rust-cargo/releases/latest/download/cargo-cyclonedx-${ARCH}-unknown-linux-gnu.tar.xz"
else
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/cargo-cyclonedx/cargo-cyclonedx.tar.xz "https://github.com/CycloneDX/cyclonedx-rust-cargo/releases/download/cargo-cyclonedx-${CARGO_CYCLONEDX_VERSION}/cargo-cyclonedx-${ARCH}-unknown-linux-gnu.tar.xz"
fi

tar --directory="/tmp/cargo-cyclonedx" -Jxvf /tmp/cargo-cyclonedx/cargo-cyclonedx.tar.xz "cargo-cyclonedx-${ARCH}-unknown-linux-gnu/cargo-cyclonedx"
sudo install -m 755 -t /usr/local/bin /tmp/cargo-cyclonedx/cargo-cyclonedx
rm -rf /tmp/cargo-cyclonedx
