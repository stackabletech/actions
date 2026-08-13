#!/usr/bin/env bash
set -uo pipefail

# This script takes two arguments:
# 1. The operator name, like zookeeper-operator
# 2. The ref name, like `main` or a tag
set -e
OPERATOR_NAME="$1"
REF_NAME="$2"
# Disable -e again as otherwise the `PR_NUMBER=$(...)` command below would exit the script on non-0
# exit codes. We however explicitly handle these error cases.
set +e

if [ "${REF_NAME}"  == "main" ]; then
  echo "0.0.0-dev"
  exit
fi

# Look up the PR number for the ref. If no PR exists, this exists with a non-0
# exit code which we handle below.
PR_NUMBER=$(gh pr view "${REF_NAME}" --json number --jq '.number')

if [ "$?" == "0" ]; then
  VERSION="0.0.0-pr$PR_NUMBER"
  # Check if the image exists upstream. If not, fall back to 0.0.0-dev
  HTTP_STATUS_CODE=$(curl -o /dev/null -w "%{http_code}" -H "Authorization: Does not matter, but Harbor needs it" "https://oci.stackable.tech/v2/sdp/${OPERATOR_NAME}/manifests/${VERSION}")

  if [ "$HTTP_STATUS_CODE" == "200" ]; then
    echo "$VERSION"
  else
    # The PR version is no (yet) available. Print a warning (to stderr) and fall back to 0.0.0-dev
    echo "::warning title=Operator Version::The operator PR version was not available (yet). Fell back to 0.0.0-dev" 1>&2
    echo "0.0.0-dev"
  fi
else
  echo "0.0.0-dev"
fi
