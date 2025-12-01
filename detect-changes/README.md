# `detect-changes`

> Manifest: [detect-changes/action.yml][detect-changes]

This action detects changed files by providing a list of glob patterns.
It can be used in situations where the GHA native path filtering cannot be used.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [detect-changes] action.

### Inputs

| Input      | Required | Description                                                     |
| ---------- | -------- | --------------------------------------------------------------- |
| `patterns` | No       | A list of glob patterns to detect changes in. Defaults to ['*'] |

### Outputs

| Output      | Description                                                                                      |
| ----------- | ------------------------------------------------------------------------------------------------ |
| `detected`  | `'true'` or `'false'` indicating if any changed files were matched by the provided glob patterns |

[detect-changes]: ./action.yaml
