# `run-pre-commit`

> Manifest: [run-pre-commit/action.yml][run-pre-commit]

This action runs pre-commit by setting up Python and optionally installing the Rust toolchain,
Hadolint, and Nix in the requested version. It requires a checkout with depth 0. It does the
following work:

1. Installs Python. The version can be configured via the `python-version` input.
2. Optionally sets up the Rust toolchain, Hadolint, and Nix.
3. Runs pre-commit on changed files.

Example usage (workflow):

```yaml
---
name: pre-commit

on:
  pull_request:

jobs:
  pre-commit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout
        with:
          fetch-depth: 0
          submodules: recursive
      - uses: stackabletech/actions/run-pre-commit
```

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [run-pre-commit] action.

### Inputs

- `python-version` (defaults to `3.12`)
- `pre-commit-version` (defaults to `4.2.0`)
- `rust` (eg: `1.80.1`. Disabled if not specified)
- `rust-components` (defaults to `rustfmt,clippy`)
- `rustup-version` (defaults to `1.28.1`)
- `hadolint` (eg: `v2.12.0`. Disabled if not specified)
- `nix` (eg: `2.25.2`. Disabled if not specified)
- `nix-github-token` (eg: `secrets.GITHUB_TOKEN`. Required when `nix` is set)

### Outputs

None

[run-pre-commit]: ./action.yml
