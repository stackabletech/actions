# `build-product-image`

> Manifest: [build-product-image/action.yml][build-product-image]

<!-- markdownlint-disable-next-line MD028 -->
> [!NOTE]
> The build step is not concerned with registries, ports, paths to repositories, but still requires
> a name. If the name does not contain a registry, `hub.docker.com` (?) is implied. Therefore,
> `localhost` will be used as the registry so as to avoid accidental interactions with an unintended
> registry.
>
> Ideally, bake should be refactored to use `localhost` as the registry for the previously mentioned
> reason (whether or not that is behind some option).

This action builds a *single* container image using `bake`. It does the following work:

1. Free disk space to avoid running out of disk space during larger builds.
2. Build the image using `bake` which internally uses `docker buildx`.
3. Temporarily retag the image to use `localhost` instead of `docker.stackable.tech/stackable`.
4. Produce output values to be used in later steps.

This action is considered to be the **single** source of truth regarding image index tag and image
manifest tag. All subsequent tasks must use these values to ensure consistency.

Currently, bake provides the following ouput in the `bake-target-tags` file:

```plain
docker.stackable.tech/stackable/kafka:3.4.1-stackable0.0.0-dev-amd64
```

Until bake supports the ability to specify the registry, this action will retag the image as:

```plain
localhost/kafka:3.4.1-stackable0.0.0-dev-amd64
```

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [build-product-image] action.

### Inputs

- `product-name` (eg: `kafka`)
- `product-version` (eg: `3.4.1`)
- `image-tools-version` (eg: `0.0.13`)
- `build-cache-username` (required) <!-- TODO: make the cache optional -->
- `build-cache-password` (required) <!-- TODO: make the cache optional -->
- `bake-config-file` (defaults to `./conf.py`)
- `sdp-version` (defaults to: `0.0.0-dev`)
- `extra-tag-data` (optional, eg. `pr321`)

### Outputs

- `image-manifest-tag` (eg: `3.4.1-stackable0.0.0-dev-amd64`)

[build-product-image]: ./action.yml
