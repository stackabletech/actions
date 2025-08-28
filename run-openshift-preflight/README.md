# `run-openshift-preflight`

> Manifest: [run-openshift-preflight/action.yml][run-openshift-preflight]

This action downloads the OpenShift preflight tool, runs it, and then reports if the checks passed.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [run-openshift-preflight] action.

### Inputs

| Input                | Required (Default) | Description                                  |
| -------------------- | ------------------ | -------------------------------------------- |
| `image-index-uri`    | Yes                | The image index URI (eg. oci.stackable.tech/sdp/kafka:3.4.1-stackable0.0.0-dev) of the image to be checked. |
| `image-architecture` | Yes                | The image architecture to be checked.        |
| `preflight-version`  | No (`latest`)      | The version of the OpenShift preflight tool. |

### Outputs

None.

[run-openshift-preflight]: ./action.yaml
