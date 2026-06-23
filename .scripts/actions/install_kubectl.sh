#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$("$GITHUB_ACTION_PATH/../.scripts/actions/get_architecture.sh")
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

echo "::group::Install kubectl"
curl -o /tmp/kubectl "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/${ARCH}/kubectl"
# Overwrite the existing binary
sudo install -m 755 -t /usr/local/bin /tmp/kubectl
rm -rf /tmp/kubectl

kubectl version --client
echo "::endgroup::"
