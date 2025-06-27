#!/usr/bin/env bash
set -uo pipefail

if [ "$1"  == "main" ]; then
  echo "0.0.0-dev"
  exit
fi

PR_NUMBER=$(gh pr view "$1" --json number --jq '.number')

if [ "$?" != "0" ]; then
  echo "0.0.0-dev"
else
  echo "0.0.0-pr$PR_NUMBER"
fi
