channel: "${{ env.CHANNEL_ID }}"
text: "${{ env.MESSAGE_TEXT }}"
${{ env.SLACK_THREAD_YAML }}
blocks:
  - type: "section"
    text:
      type: "mrkdwn"
      text: "${{ env.HEALTH_SLACK_EMOJI }} (${{ env.HEALTH_RATE }}) The integration test for *${{ github.repository }}* succeeded."
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
