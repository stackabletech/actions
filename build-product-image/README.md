# `build-product-image`

> Manifest: [build-product-image/action.yml][build-product-image]

<!-- markdownlint-disable-next-line MD028 -->
> [!NOTE]
> The build step is not concerned with registries, ports, paths to repositories, but still requires
> a name. If the name does not contain a registry, `hub.docker.com` (?) is implied. Therefore,
> `localhost` will be used as the registry so as to avoid accidental interactions with an unintended
> registry.

This action builds a *single* container image using `boil`. It does the following work:

1. Free disk space to avoid running out of disk space during larger builds.
2. Build the image using `boil` which internally uses `docker buildx`.
3. Produce output values to be used in later steps.

This action is considered to be the **single** source of truth regarding image index tag and image
manifest tag. All subsequent tasks must use these values to ensure consistency.

boil provides the following output in the `boil-target-tags` file:

```plain
localhost/kafka:3.4.1-stackable0.0.0-dev-amd64
```

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [build-product-image] action.

### Inputs

| Input              | Required (Default) | Description                                             |
| ------------------ | ------------------ | ------------------------------------------------------- |
| `product-name`     | Yes                | The name of the product image to build (eg: `kafka`)    |
| `product-version`  | Yes                | The version of the product image to build (eg: `3.4.1`) |
| `boil-version`     | No (`latest`)      | The version of boil used to build the image             |
| `boil-config-file` | No (`./boil.toml`) | The path to the boil config file                        |
| `sdp-version`      | No (`0.0.0-dev`)   | The SDP version of the image                            |
| `extra-tag-data`   | No                 | Extra data to be included in the tag, eg. `pr321`       |

### Outputs

| Output                               | Example                          | Description                                     |
| ------------------------------------ | -------------------------------- | ----------------------------------------------- |
| `image-manifest-tag`                 | `3.4.1-stackable0.0.0-dev-amd64` | The image manifest tag (including architecture) |
| `suggested-image-index-manifest-tag` | `3.4.1-stackable0.0.0-dev`       | The suggested image index manifest tag          |

[build-product-image]: ./action.yaml
