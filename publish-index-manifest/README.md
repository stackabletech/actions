# `publish-index-manifest`

> Manifest: [publish-index-manifest/action.yml][publish-index-manifest]

This action creates an image index manifest, publishes it, and signs it. It does the following work:

1. Create an image index manifest and link to each architecture in `image-architectures`.
2. Push the image index manifest.
3. Sign the image index manifest (which pushes the signature to the specified registry).

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [publish-index-manifest] action.

### Inputs

- `image-registry-uri`(eg: `oci.stackable.tech`)
- `image-registry-username` (required)
- `image-registry-password` (required)
- `image-repository` (eg: `stackable/kafka`)
- `image-index-manifest-tag` (eg: `3.4.1-stackable0.0.0-dev`)
- `image-architectures` (defaults to `["amd64", "arm64"]`)

### Outputs

None

[publish-index-manifest]: .//action.yml
