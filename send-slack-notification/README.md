# `send-slack-notification`

> Manifest: [send-slack-notification/action.yaml][send-slack-notification]

This action sends a Slack message to notify about container image build failures and successes.
Subsequent attempts of the same workflow run are automatically threaded in Slack.
The color of the message is automatically selected based on the provided results.

## Supported notifications

Currently, two types of notifications are supported.

### Container image builds

```yaml
jobs:
  notify:
    name: Failure Notification
    needs: [job_1, job_2]
    runs-on: ubuntu-latest
    if: failure() || github.run_attempt > 1
    steps:
      - name: Send Notification
        uses: stackabletech/actions/send-slack-notification
        with:
          type: container-image-build
          channel-id: DEADBEEF
          build-result: ${{ needs.job_1.result }}
          publish-manifests-result: ${{ needs.job_2.result }}
          slack-token: ${{ secrets.MY_SECRET }}
```

### Integration tests for operators

```yaml
jobs:
  notify:
    name: Failure Notification
    needs: [job_1]
    runs-on: ubuntu-latest
    if: failure() || github.run_attempt > 1
    steps:
      - name: Send Notification
        uses: stackabletech/actions/send-slack-notification
        with:
          type: integration-test
          channel-id: DEADBEEF
          failed-tests: ${{ needs.job_1.failed-tests }}
          test-result: ${{ needs.job_1.result }}
          test-health: ${{ needs.job_1.health }}
          slack-token: ${{ secrets.MY_SECRET }}
```

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [send-slack-notification] action.

### Inputs

- `type` (required, supported values: `container-image-build` and `integration-test`)
- `channel-id` (required)
- `slack-token` (required)
- `build-result` (optional, e.g. `success`)
- `publish-manifests-result` (optional, e.g. `failure`)

### Outputs

None

[send-slack-notification]: ./action.yaml
