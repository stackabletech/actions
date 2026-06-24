#!/usr/bin/env bash

set -euo pipefail
[ -n "${RUNNER_DEBUG+set}" ] && set -x

ARCH=$(uname -m)
cp "$GITHUB_ACTION_PATH/../configs/curlrc" "$XDG_CONFIG_HOME/curlrc"

echo "::group::Install prek"
mkdir /tmp/prek

# TODO (@Techassi): Verify checksum
if [ "$PREK_VERSION" == "latest" ]; then
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/prek/prek.tar.gz "https://github.com/j178/prek/releases/latest/download/prek-${ARCH}-unknown-linux-gnu.tar.gz"
else
  curl --config "$XDG_CONFIG_HOME/curlrc" -o /tmp/prek/prek.tar.gz "https://github.com/j178/prek/releases/download/${PREK_VERSION}/prek-${ARCH}-unknown-linux-gnu"
fi

tar --directory="/tmp/prek" --strip-components=1 -zxvf /tmp/prek/prek.tar.gz "prek-${ARCH}-unknown-linux-gnu/prek"
sudo install -m 755 -t /usr/local/bin /tmp/prek/prek
rm -rf /tmp/prek

prek --version
echo "::endgroup::"
