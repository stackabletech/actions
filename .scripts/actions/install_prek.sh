#!/usr/bin/env bash

set -euo pipefail
[ -n "$GITHUB_DEBUG" ] && set -x

ARCH=$(uname -m)

echo "::group::Install prek"
mkdir /tmp/prek

if [ "$PREK_VERSION" == "latest" ]; then
  curl -fsSL -o /tmp/prek/prek.tar.gz "https://github.com/j178/prek/releases/latest/download/prek-${ARCH}-unknown-linux-gnu.tar.gz"
else
  curl -fsSL -o /tmp/prek/prek.tar.gz "https://github.com/j178/prek/releases/download/${PREK_VERSION}/prek-${ARCH}-unknown-linux-gnu"
fi

tar --directory="/tmp/prek" -zxvf /tmp/prek/prek.tar.gz prek
sudo install -m 755 -t /usr/local/bin /tmp/prek/prek

prek --version
echo "::endgroup::"
