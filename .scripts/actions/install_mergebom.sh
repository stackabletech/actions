#!/usr/bin/env bash

# Installs a released mergebom binary after verifying its cosign signature.
# Requires cosign and the MERGEBOM_VERSION environment variable (e.g. "0.1.0").

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
TAG="mergebom-${MERGEBOM_VERSION}"
BINARY="mergebom-${ARCH}-unknown-linux-gnu"
DOWNLOAD_DIR=$(mktemp -d)

cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

curl --config "$XDG_CONFIG_HOME/curlrc" -o "$DOWNLOAD_DIR/$BINARY" \
  "https://github.com/stackabletech/actions/releases/download/${TAG}/${BINARY}"
curl --config "$XDG_CONFIG_HOME/curlrc" -o "$DOWNLOAD_DIR/$BINARY.bundle" \
  "https://github.com/stackabletech/actions/releases/download/${TAG}/${BINARY}.bundle"

# The binary must have been signed by the release workflow of this repository, for this tag.
"$GITHUB_ACTION_PATH/../.scripts/actions/retry.sh" cosign verify-blob \
  --certificate-identity "https://github.com/stackabletech/actions/.github/workflows/release_mergebom.yml@refs/tags/${TAG}" \
  --certificate-oidc-issuer https://token.actions.githubusercontent.com \
  --bundle "$DOWNLOAD_DIR/$BINARY.bundle" \
  "$DOWNLOAD_DIR/$BINARY"

sudo install -m 755 "$DOWNLOAD_DIR/$BINARY" /usr/local/bin/mergebom
rm -rf "$DOWNLOAD_DIR"
