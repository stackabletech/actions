# Actions

This repository contains various reusable actions which encapsulate series of commands to run a
particular step in a workflow.

## Definitions

| Name                               | Example                                                                |
| ---------------------------------- | ---------------------------------------------------------------------- |
| Image Registry                     | `docker.stackable.tech`                                                |
| Image Repository                   | `stackable/kafka`                                                      |
| Image Index Manifest Tag           | `3.4.1-stackable0.0.0-dev`                                             |
| Image Manifest Tag                 | `3.4.1-stackable0.0.0-dev-amd64`                                       |
| Image Repository URI               | `docker.stackable.tech/stackable/kafka`                                |
| Image Index URI (if multi-arch)    | `docker.stackable.tech/stackable/kafka:3.4.1-stackable0.0.0-dev`       |
| Image Manifest URI (if multi-arch) | `docker.stackable.tech/stackable/kafka:3.4.1-stackable0.0.0-dev-amd64` |
| Image Repo Digest                  | `docker.stackable.tech/stackable/kafka@sha256:917f800259ef4915f976...` |
| Digest                             | `sha256:917f800259ef4915f976e93987b752fd64debf347568610d7f685d2022...` |

## Available Actions

- [build-container-image](./build-container-image/README.md)
- [build-product-image](./build-product-image/README.md)
- [free-disk-space](./free-disk-space/README.md)
- [publish-image](./publish-image/README.md)
- [publish-index-manifest](./publish-index-manifest/README.md)
- [run-pre-commit](./run-pre-commit/README.md)
- [shard](./shard/README.md)
