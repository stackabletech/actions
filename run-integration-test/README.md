# `run-integration-test`

> Manifest: [run-integration-test/action.yml][run-integration-test]

This action runs an operator integration test. It does the following work:

1. Create a test cluster on-the-fly using the requested Kubernetes version, distribution and node
   architecture via Replicated. See [Test Platform Triple](#test-platform-triple) for more details.
2. Run the integration test based on the provided test parameters.
3. Delete the cluster of the tests are done and send out a notification on failure.

## Test Platform Triple

The [`test-platform`](#inputs) input expects a test platform triple to select the appropriate node
architecture and Kubernetes distribution & version. The triple format is
`<DISTRIBUTION>-<VERSION>-<ARCHITECTURE>`, eg. `kind-1.31.2-amd64` or `gke-1.31-arm64`.

Each distribution supports different instance types
based on the cloud vendor machine names. This mapping is done via the `instances.yml` file. Based
on this file, the following distributions are supported: `eks`, `gke`, `aks`, `kind`, `k3s`, `rke2`.
There is no mapping for `oke` yet.

Supported Kubernetes version can be inspected on the official Replicated documentation
[page][supported-clusters]. Supported architectures are `amd64` and `arm64`.

## Integration Test Configuration File

Each downstream repository needs a configuration file. This allows customization of various
parameters based on the needs of the operator, or the particular tests. The config file needs to be
placed here: `tests/interu.yaml` to be picked up automatically.

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [run-integration-test] action.

### Inputs

- `test-platform`(required, eg: `kind-1.31.2-amd64`)
- `test-run` (required, `test-suite` or `test`)
- `test-parameter` (defaults to `smoke`)
- `replicated-api-token` (required)

> [!NOTE]
> `test-parameter` maps to a specific test *name*, not to a single test with all dimensions resolved.

### Outputs

- `start-time`
- `end-time`

[supported-clusters]: https://docs.replicated.com/vendor/testing-supported-clusters
[run-integration-test]: ./action.yml
