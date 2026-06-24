#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

echo "::group::Install kubectl-kuttl"
curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/kubectl-kuttl "https://github.com/kudobuilder/kuttl/releases/download/v$KUTTL_VERSION/kubectl-kuttl_${KUTTL_VERSION}_linux_${ARCH}"
sudo install -m 755 -t /usr/local/bin /tmp/kubectl-kuttl
rm -rf /tmp/kubectl-kuttl
echo "::endgroup::"
