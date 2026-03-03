#!/usr/bin/env bash

# Initially, the retry args are empty. If the command fails, the retry args are
# set to what the user provided via the RETRY_ARGS env var.
retry_args=""

# Retries any command passed to this script a maximum number of $RETRY_COUNT and
# wait $RETRY_TIMEOUT between each try. If the command failed $RETRY_COUNT, this
# script will return with exit code 1.
for TRY in $(seq 1 "$RETRY_COUNT"); do
  if [ -z "$retry_args" ]; then
    "$@"
  else
    "$@" "$retry_args"
  fi

  EXIT_CODE=$?
  # If command ran successfully, exit the loop
  if [ $EXIT_CODE -eq 0 ]; then
    break
  fi

  echo "Command failed $TRY time(s)"

  # Exit if we reached the number if retries and the command didn't run successfully
  if [ "$TRY" == "$RETRY_COUNT" ]; then
    echo "Exiting"
    exit 1
  fi

  retry_args="${RETRY_ARGS:-}"
  echo "Waiting for $RETRY_TIMEOUT to try again"
  sleep "$RETRY_TIMEOUT"
done
