# `build-container-image`

> Manifest: [build-container-image/action.yml][build-container-image]

This action builds a *single* container image using `docker buildx build`. It does the following work:

1. Free disk space to avoid running out of disk space during larger builds.
2. Build the image using `docker buildx build`, outputting the architecture specific tag.

This action is considered to be the **single** source of truth regarding the image manifest tag.
All subsequent tasks must use this value to ensure consistency.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [build-container-image] action.

### Inputs

- `image-name` (eg: `kafka`)
- `image-index-manifest-tag` (eg: `3.4.1-stackable0.0.0-dev`)
- `container-file` (defaults to `Dockerfile`)
- `build-context` (defaults to `.`)

### Outputs

- `image-repository-uri` (eg: `localhost/kafka`)
- `image-manifest-tag` (eg: `3.4.1-stackable0.0.0-dev-amd64`)
- `image-manifest-uri` (eg: `localhost/kafka:3.4.1-stackable0.0.0-dev-amd64`)

[build-container-image]: ./action.yaml
