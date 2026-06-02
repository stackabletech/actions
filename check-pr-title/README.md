# `check-pr-title`

> Manifest: [check-pr-title/action.yml][check-pr-title]

This action checks that the PR title conforms to rules defined by the committed config file.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [check-pr-title] action.

### Inputs

| Input               | Required | Description                                                            |
| ------------------- | -------- | ---------------------------------------------------------------------- |
| `config-file`       | No       | Path to the committed config file. Defaults to `./committed.toml`      |
| `committed-version` | No       | The committed version used to check the PR title. Defaults to `latest` |

### Outputs

None.

[check-pr-title]: ./action.yaml
