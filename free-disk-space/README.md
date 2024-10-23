# `free-disk-space`

> Manifest: [free-disk-space/action.yml][free-disk-space]

This action This action frees up disk space on a runner.

It is based on `jlumbroso/free-disk-space`and runs cleanup tasks in parallel where possible, and hides the STDOUT output of each task to reduce noise in workflow runs.

> [!NOTE]
> This action is used by the [build-container-image] and [build-product-image] actions.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [free-disk-space] action.

### Inputs

None

### Outputs

None

[free-disk-space]: ./action.yml
[build-container-image]: ../build-container-image/action.yml
[build-product-image]: ../build-product-image/action.yml
