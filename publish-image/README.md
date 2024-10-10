# `publish-image`

> Manifest: [publish-image/action.yml][publish-image]

This action signs and publishes a *single* container image to the given registry. It does the
following work:

1. Tag the `source-image-uri` with the specified `image-registry-uti`, `image-repository`, and
   `image-repository`.
2. Push the container image to the specified registry.
3. Sign the container image (which pushes the signature to the specified registry).
4. Generate an SBOM via a syft scan.
5. Attest an image with the SBOM as a predicate (which pushes the attestation to the specified
   registry).

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [publish-image] action.

<!-- markdownlint-disable-next-line MD028 -->
> [!IMPORTANT]
> For multi-arch images, the `image-manifest-tag` should have the `-$ARCH` suffix, as the tag
> without it should be reserved for the image index manifest which will refer to container images
> for each architecture we will push images for.

### Inputs

- `image-registry-uri`
- `image-registry-username`
- `image-registry-password`
- `image-repository`
- `image-manifest-tag`
- `source-image-uri`

### Outputs

None

[publish-image]: ./action.yml
