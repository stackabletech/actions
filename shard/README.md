# `shard`

> Manifest: [shard/action.yml][shard]

This action produces a list of versions for a product. This is to be used as a matrix dimension to
parallelize builds. It does the following work:

1. Reads the `conf.py`, filtering versions for the product
2. Write the JSON array of version to `$GITHUB_OUTPUT` for use in a matrix.

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
          product-name: ${{ env.PRODUCT_NAME }}
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

- `product-name` (eg: `kafka`)

### Outputs

- `versions` (eg: `["3.7.1", "3.8.0"]`)

[shard]: ./action.yaml
