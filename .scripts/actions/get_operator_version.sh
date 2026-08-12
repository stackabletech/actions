#!/usr/bin/env bash
set -uo pipefail

# This script takes two arguments:
# 1. The operator name, like zookeeper-operator
# 2. The ref name, like `main` or a tag

if [ "$2"  == "main" ]; then
  echo "0.0.0-dev"
  exit
fi

PR_NUMBER=$(gh pr view "$2" --json number --jq '.number')

if [ "$?" == "0" ]; then
  VERSION="0.0.0-pr$PR_NUMBER"
  # Check if the image exists upstream. If not, fall back to 0.0.0-dev
  HTTP_STATUS_CODE=$(curl -o /dev/null -w "%{http_code}" -H "Authorization: Does not matter, but Harbor needs it" "https://oci.stackable.tech/v2/sdp/$1/manifests/${VERSION}")

  if [ "$HTTP_STATUS_CODE" == "200" ]; then
    echo "$VERSION"
  else
    # The PR version is no (yet) available. Print a warning and fall back to 0.0.0-dev
    echo "::warning title=Operator Version::The operator PR version was not available (yet). Fell back to 0.0.0-dev" 1>&2
    echo "0.0.0-dev"
  fi
else
  echo "0.0.0-dev"
fi
