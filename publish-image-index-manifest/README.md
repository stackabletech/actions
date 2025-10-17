# `publish-image-index-manifest`

> Manifest: [publish-image-index-manifest/action.yml][publish-image-index-manifest]

This action creates an image index manifest, publishes it, and signs it. It does the following work:

1. Create an image index manifest and link to each architecture in `image-architectures`.
2. Push the image index manifest.
3. Sign the image index manifest (which pushes the signature to the specified registry).

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [publish-image-index-manifest] action.

### Inputs

| Input                      | Required (Default)        | Description                                                                         |
| -------------------------- | ------------------------- | ----------------------------------------------------------------------------------- |
| `image-registry-uri`       | Yes                       | The image registry URI, eg `oci.stackable.tech`                                     |
| `image-registry-username`  | Yes                       | The username used to access the image registry                                      |
| `image-registry-password`  | Yes                       | The password used to access the image registry                                      |
| `image-repository`         | Yes                       | The path to the image, eg `sdp/kafka`                                               |
| `image-index-manifest-tag` | Yes                       | Human-readable tag without architecture information, eg `3.4.1-stackable0.0.0-dev`  |
| `image-architectures`      | No (`["amd64", "arm64"]`) | The list of architectures the to-bo-published image was built for                   |
| `cosign-retries`           | No (3)                    | The number of times cosign operations should be retried                             |
| `cosign-retry-timeout`     | No (30s)                  | Duration to wait before a new cosign operation is retried, format: `NUMBER[SUFFIX]` |

### Outputs

- `image-index-uri`: The final image index URI, eg. `oci.stackable.tech/spd/kafka:3.4.1-stackable0.0.0-dev`.

[publish-image-index-manifest]: ./action.yaml
