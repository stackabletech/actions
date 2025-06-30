#!/usr/bin/env bash

# Get a manifest digest. Example inputs:
# - oci.stackable.tech/sdp/hello-world:0.0.1-SNAPSHOT-stackable0.0.0-dev
# - oci.stackable.tech/sdp/hello-world:0.0.1-SNAPSHOT-stackable0.0.0-dev-amd64
set -euo pipefail

# Note: `docker manifest push` currently outputs the same hash, but `manifest`
# is experimental and the STDOUT is more likely to change than the structured
# output.
docker buildx imagetools inspect --format '{{println .Manifest.Digest}}' "$1"
