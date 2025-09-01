# `shard`

> Manifest: [shard/action.yml][shard]

This action produces a list of versions for an image. This is to be used as a matrix dimension to
parallelize builds. It does the following work:

1. Reads the `<IMAGE_NAME>/boil-config.toml` file, extracting the versions for the image
2. Write the JSON array of versions to `$GITHUB_OUTPUT` for use in a matrix.

Example usage:

```yaml
jobs:
  generate_matrix:
    name: Generate Version List
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout
      - id: shard
        uses: stackabletech/actions/shard
        with:
          image-name: ${{ env.IMAGE_NAME }}
    outputs:
      versions: ${{ steps.shard.outputs.versions }}

  actual_matrix:
    needs: [generate_matrix]
    strategy:
      matrix:
        versions: ${{ fromJson(needs.generate_matrix.outputs.versions) }}
    # ...
```

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [shard] action.

### Inputs

| Input          | Required (Default) | Description                                             |
| -------------- | ------------------ | ------------------------------------------------------- |
| `image-name`   | Yes                | The name of the image, eg: `kafka`                      |
| `boil-version` | No (`latest`)      | The version of boil used to create the list of versions |

### Outputs

| Output     | Example              | Description                                |
| ---------- | -------------------- | ------------------------------------------ |
| `versions` | `["3.7.1", "3.8.0"]` | A JSON array containing the image versions |

[shard]: ./action.yaml
