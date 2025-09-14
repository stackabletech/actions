#!/usr/bin/env bash

set -euo pipefail
[ -n "$GITHUB_DEBUG" ] && set -x

ARCH=$("$GITHUB_ACTION_PATH/../.scripts/actions/get_architecture.sh")

echo "::group::Install kubectl"
curl -fsSL -o /tmp/kubectl "https://dl.k8s.io/release/${KUBECTL_VERSION}/bin/linux/${ARCH}/kubectl"
# Overwrite the existing binary
sudo install -m 755 -t /usr/local/bin /tmp/kubectl

kubectl version --client
echo "::endgroup::"
