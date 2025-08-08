channel: "${{ inputs.channel-id }}"
text: "${{ env.MESSAGE_TEXT }}"
${{ steps.retrieve-slack-thread-id.outcome == 'success' && format('thread_ts: "{0}"', env.SLACK_THREAD_ID) || '' }}
blocks:
  - type: "section"
    text:
      type: "mrkdwn"
      text: "${{ env.HEALTH_SLACK_EMOJI }} (${{ env.HEALTH_RATE }})" The integration test for *${{ github.repository }}* succeeded."
  - type: "actions"
    elements:
      - type: button
        text:
          type: "plain_text"
          text: "View Workflow Run"
          emoji: false
        url: "${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}/attempts/${{ github.run_attempt }}"
      - type: button
        text:
          type: "plain_text"
          text: "View Dashboard"
          emoji: false
        url: "https://example.org"
