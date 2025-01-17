# `run-integration-test`

> Manifest: [run-integration-test/action.yml][run-integration-test]

This action runs an operator integration test. It does the following work:

1. Create a test cluster on-the-fly using the requested Kubernetes version, distribution and node
   architecture via Replicated.
2. Run the integration test based on the provided test parameters.
3. Delete the cluster of the tests are done and send out a notification on failure.

## Integration Test Configuration File

Each downstream repository needs a configuration file. This allows customization of various
parameters based on the needs of the operator, or the particular tests. The config file needs to be
placed here: `tests/interu.yaml` to be picked up automatically.

There are two major components in the config file:

- Definition of `runners`.
- Test `profiles`.

### Runners

The runner configuration selects from the available Kubernetes versions and distributions along with
node groups of desired instance size, architecture, disk size.

```yaml
runners:
  default-amd64:
    platform: rke2-1.31.2
    ttl: 4h
    node-groups:
      - name: default
        arch: amd64
        size: large
        disk: 50
        nodes: 3
```

The platform is specified using a platform pair, which consists of the name of the Kubernetes
distribution and version, eg. `rke2-1.31.2`. Each distribution supports different instance types
based on the cloud vendor machine names. This mapping is done via the `instances.yml` file. Based
on this file, the following distributions are supported: `eks`, `gke`, `aks`, `kind`, `k3s`, `rke2`.
There is no mapping for `oke` yet.

Supported Kubernetes version can be inspected on the official Replicated documentation
[page][supported-clusters]. Supported architectures are `amd64` and `arm64`.

### Profiles

Profiles allow for a variety of pre-configured runners and strategies. A profile can be chosen when
calling interu. For example, the `schedule` profile could be used in CI on the `schedule` event.

The following strategies are currently available:

- `weighted`: allows defining two or more `weights`. Each `weight` defines how often the
  runner specified is used when this profile is used. It should be noted that the weights *don't*
  need to add up to 100, but it is recommended to more easily gauge the probability.
- `use-runner`: uses the specified `runner`.

Each profile can additionally specify test `options`, like `parallelism`, `test-run` and
`test-parameter`.

```yaml
profiles:
  schedule:
    strategy: weighted
    weights:
      - weight: 80
        runner: default-amd64
      - weight: 10
        runner: default-arm64
      - weight: 10
        runner: default-mixed
    options:
      parallelism: 1

  workflow_dispatch:
    strategy: use-runner
    runner: default-amd64
    options:
      test-run: test-suite
      test-parameter: smoke
      parallelism: 2
```

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [run-integration-test] action.

### Inputs

- `test-profile` (required)
- `replicated-api-token` (required)
- `interu-version` (optional)

> [!NOTE]
> `test-parameter` maps to a specific test *name*, not to a single test with all dimensions resolved.

### Outputs

- `start-time`
- `end-time`

[supported-clusters]: https://docs.replicated.com/vendor/testing-supported-clusters
[run-integration-test]: ./action.yml
