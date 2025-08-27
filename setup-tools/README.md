# `setup-tools`

> Manifest: [setup-tools/action.yml][setup-tools]

This action downloads and installs Stackable tools.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [setup-tools] action.

### Inputs

| Input                  | Required | Description                  |
| ---------------------- | -------- | ---------------------------- |
| `boil-version`         | No       | The version of boil*         |
| `stackablectl-version` | No       | The version of stackablectl* |
| `interu-version`       | No       | The version of interu*       |
| `beku-version`         | No       | The version of beku*         |

\* If no input is set, the tool won't be installed.

### Outputs

None.

[setup-tools]: ./action.yaml
