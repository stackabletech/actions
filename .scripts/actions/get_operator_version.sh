#!/usr/bin/env bash
set -euo pipefail

# This script takes two arguments:
# 1. The complete repository name, like `stackabletech/zookeeper-operator`
# 2. The operator name, like zookeeper-operator
# 3. The ref name, like `main` or a tag
REPOSITORY="$1"
OPERATOR_NAME="$2"
REF_NAME="$3"

DEFAULT_VERSION="0.0.0-dev"

# Detect well-known ref names first
# Detect if run against the `main` branch
if [ "${REF_NAME}"  == "main" ]; then
  echo "${DEFAULT_VERSION}"
  exit
fi

# Detect release branches in the form `release-YY.MM`
if [[ "${REF_NAME}" =~ ^release-[0-9]{2}\.[0-9]{1,2}$ ]]; then
  VERSION=$(echo "${REF_NAME}" | cut -d - -f 2)
  echo "${VERSION}.0"
  exit
fi

# Detect release tags in the form `YY.MM.X<REST>`
if [[ "${REF_NAME}" =~ ^[0-9]{2}\.[0-9]{1,2}\.[0-9]{1,2} ]]; then
  echo "${REF_NAME}"
  exit
fi

# Now handle unknown ref names.
# Look up the PR number for the ref. If no PR exists, this exists with a non-0
# exit code which we handle below.
if PR_NUMBER=$(gh pr view "${REF_NAME}" --repo "${REPOSITORY}" --json number --jq '.number'); then
  VERSION="0.0.0-pr$PR_NUMBER"
  # Check if the image exists upstream. If not, fall back to 0.0.0-dev
  HTTP_STATUS_CODE=$(curl -o /dev/null -w "%{http_code}" -H "Authorization: Does not matter, but Harbor needs it" "https://oci.stackable.tech/v2/sdp/${OPERATOR_NAME}/manifests/${VERSION}")

  if [ "$HTTP_STATUS_CODE" == "200" ]; then
    echo "$VERSION"
  else
    # The PR version is no (yet) available. Print a warning (to stderr) and fall back to 0.0.0-dev
    echo "::warning title=Operator Version::The operator PR version was not available (yet). Fell back to ${DEFAULT_VERSION}" 1>&2
    echo "${DEFAULT_VERSION}"
  fi
else
  echo "${DEFAULT_VERSION}"
fi
