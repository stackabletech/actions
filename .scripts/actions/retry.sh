#!/usr/bin/env bash

# Retries any command passed to this script a maximum number of $CMD_RETRIES and
# wait $CMD_TIMEOUT between each try. If the command failed $CMD_RETRIES, this
# script will return with exit code 1.
for TRY in $(seq 1 "$CMD_RETRIES"); do
  "$@"

  EXIT_CODE=$?
  # If command ran successfully, exit the loop
  if [ $EXIT_CODE -eq 0 ]; then
    break
  fi

  echo "Command failed $TRY time(s)"

  # Exit if we reached the number if retries and the command didn't run successfully
  if [ "$TRY" == "$CMD_RETRIES" ]; then
    echo "Exiting"
    exit 1
  fi

  echo "Waiting for $CMD_TIMEOUT to try again"
  sleep "$CMD_TIMEOUT"
done
