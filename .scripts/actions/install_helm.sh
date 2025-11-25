#!/usr/bin/env bash

set -euo pipefail
[ -n "$GITHUB_DEBUG" ] && set -x

PLATFORM=$("$GITHUB_ACTION_PATH/../.scripts/actions/get_platform.sh")
ARCH=$("$GITHUB_ACTION_PATH/../.scripts/actions/get_architecture.sh")

FILENAME="helm-${HELM_VERSION}-${PLATFORM}-${ARCH}.tar.gz"
# The signature is expired since a couple of years...
VERIFY_SIGNATURE="false"

echo "::group::Install helm"
mkdir /tmp/helm
curl -fsSL -o /tmp/helm/helm.tar.gz "https://get.helm.sh/${FILENAME}"

if [[ "$VERIFY_SIGNATURE" == "true" ]]; then
  curl -fsSL -o /tmp/helm/helm.tar.gz.asc "https://github.com/helm/helm/releases/download/${HELM_VERSION}/${FILENAME}.asc"
  curl https://keybase.io/mattfarina/pgp_keys.asc | gpg --import
  gpg --verify /tmp/helm/helm.tar.gz.asc /tmp/helm/helm.tar.gz
fi

tar --directory="/tmp/helm" --strip-components=1 -zxvf /tmp/helm/helm.tar.gz "${PLATFORM}-${ARCH}"
# Overwrite the existing binary
sudo install -m 755 -t /usr/local/bin /tmp/helm/helm

helm version --short
echo "::endgroup::"
