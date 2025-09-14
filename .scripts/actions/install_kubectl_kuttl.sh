#!/usr/bin/env bash

set -euo pipefail
[ -n "$GITHUB_DEBUG" ] && set -x

ARCH=$(uname -m)

echo "::group::Install kubectl-kuttl"
curl -fsSL -o /tmp/kubectl-kuttl "https://github.com/kudobuilder/kuttl/releases/download/v$KUTTL_VERSION/kubectl-kuttl_${KUTTL_VERSION}_linux_${ARCH}"
sudo install -m 755 -t /usr/local/bin /tmp/kubectl-kuttl
echo "::endgroup::"
