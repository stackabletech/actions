# `run-integration-test`

> Manifest: [run-integration-test/action.yml][run-integration-test]

This action runs an operator integration test. It does the following work:

1. Create a test cluster on-the-fly using the requested Kubernetes version and distribution via
   Replicated.
2. Run the integration test based on the provided test parameters.
3. Delete the cluster of the tests are done and send out a notification on failure.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [run-integration-test] action.

### Inputs

- `test-platform`(required, eg: `kind-1.31.0-amd64`)
- `test-run` (required, `test-suite` or `test`)
- `test-parameter` (defaults to `smoke`)
- `replicated-api-token` (required)

### Outputs

- `start-time`
- `end-time`

[run-integration-test]: ./action.yml
