# `send-slack-notification`

> Manifest: [send-slack-notification/action.yaml][send-slack-notification]

This action sends a Slack message to notify about container image build failures and successes.
Subsequent attempts of the same workflow run are automatically threaded in Slack.
The color of the message is automatically selected based on the provided results.

Example usage (workflow):

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
          build-result: ${{ needs.job_2.result }}
          publish-manifests-result: ${{ needs.job_2.result }}
          slack-token: ${{ secrets.MY_SECRET }}
```

## Inputs and Outputs

> [!TIP]
> For descriptions of the inputs and outputs, see the complete [send-slack-notification] action.

### Inputs

- `channel-id` (defaults to `C07UG6JH44F`)
- `build-result` (required, e.g. `success`)
- `publish-manifests-result` (required, e.g. `failure`)
- `slack-token` (required)

### Outputs

None

[send-slack-notification]: ./action.yaml
