#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

PLATFORM=$("$GITHUB_ACTION_PATH/../.scripts/actions/get_platform.sh")
ARCH=$("$GITHUB_ACTION_PATH/../.scripts/actions/get_architecture.sh")
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

FILENAME="helm-${HELM_VERSION}-${PLATFORM}-${ARCH}.tar.gz"
VERIFY_SIGNATURE="${VERIFY_SIGNATURE:-true}"

echo "::group::Install helm"
mkdir /tmp/helm
curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/helm/helm.tar.gz "https://get.helm.sh/${FILENAME}"

if [[ "$VERIFY_SIGNATURE" == "true" ]]; then
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/helm/helm.tar.gz.asc "https://github.com/helm/helm/releases/download/${HELM_VERSION}/${FILENAME}.asc"
  curl --config "$XDG_CONFIG_HOME/curlrc" https://keybase.io/mattfarina/pgp_keys.asc | gpg --import
  gpg --verify /tmp/helm/helm.tar.gz.asc /tmp/helm/helm.tar.gz
fi

tar --directory="/tmp/helm" --strip-components=1 -zxvf /tmp/helm/helm.tar.gz "${PLATFORM}-${ARCH}"
# Overwrite the existing binary
sudo install -m 755 -t /usr/local/bin /tmp/helm/helm
rm -rf /tmp/helm

helm version --short
echo "::endgroup::"
