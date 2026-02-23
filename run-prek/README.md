# `run-prek`

> Manifest: [run-prek/action.yml][run-prek]

This action sets up the prek tool, and additional tools required for various hooks. It then runs
prek against the changed files. This actions expects checkouts with depth 0. It does the following
work:

1. Installs prek in the specified version.
2. Optionally sets up the Rust toolchain, Hadolint, and Nix.
3. Runs prek on changed files.

Example usage (workflow):

```yaml
---
name: prek

on:
  pull_request:

jobs:
  prek:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout
        with:
          fetch-depth: 0
          submodules: recursive
      - uses: stackabletech/actions/run-prek
```

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [run-prek] action.

### Inputs

- `prek-version` (defaults to `latest`)
- `rust` (eg: `1.80.1`. Disabled if not specified)
- `rust-components` (defaults to `rustfmt,clippy`)
- `hadolint` (eg: `v2.12.0`. Disabled if not specified)
- `nix` (eg: `2.25.2`. Disabled if not specified)
- `nix-github-token` (eg: `secrets.GITHUB_TOKEN`. Required when `nix` is set)

### Outputs

None

[run-prek]: ./action.yaml
