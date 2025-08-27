# `setup-k8s-tools`

> Manifest: [setup-k8s-tools/action.yml][setup-k8s-tools]

This action downloads and installs Kubernetes tools.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [setup-k8s-tools] action.

### Inputs

| Input             | Required | Description                   |
| ----------------- | -------- | ----------------------------- |
| `kubectl-version` | No       | The version of kubectl*       |
| `kuttl-version`   | No       | The version of kubectl-kuttl* |
| `helm-version`    | No       | The version of helm*          |

\* If no input is set, the tool won't be installed.

### Outputs

None.

[setup-k8s-tools]: ./action.yaml
