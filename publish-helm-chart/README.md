# `publish-helm-chart`

> Manifest: [publish-helm-chart/action.yml][publish-helm-chart]

This action packages, publishes, and signs a Helm chart.
It needs the `id-token: write` permission to be able to sign the Helm chart with a GitHub OIDC token.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [publish-helm-chart] action.

### Inputs

| Input                     | Required | Description                                                     |
| ------------------------- | -------- | --------------------------------------------------------------- |
| `chart-registry-uri`      | Yes      | The URI of the Helm Chart registry                              |
| `chart-registry-username` | Yes      | The username used to login to the Helm Chart registry           |
| `chart-registry-password` | Yes      | The password used to login to the Helm Chart registry           |
| `chart-repository`        | Yes      | Path to the Helm chart, for example `sdp-charts/kafka-operator` |
| `chart-directory`         | Yes      | The directory where the Chart.yaml file is located              |
| `chart-version`           | Yes      | The Helm Chart version                                          |
| `app-version`             | Yes      | The app version to set in the Helm Chart                        |
| `helm-version`            | No       | The version of helm                                             |

### Outputs

None.

[publish-helm-chart]: ./action.yaml
