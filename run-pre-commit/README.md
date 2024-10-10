# `run-pre-commit`

> Manifest: [run-pre-commit/action.yml][run-pre-commit]

This action runs pre-commit by setting up Python and optionally the Rust toolchain and Hadolint in
the requested version. It requires a checkout with depth 0. It does the following work:

1. Installs Python. The version can be configured via the `python-version` input.
2. Optionally sets up the Rust toolchain and Hadolint.
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

- `python-version`
- `rust`
- `rust-components`
- `hadolint`

### Outputs

None

[run-pre-commit]: ./action.yml
