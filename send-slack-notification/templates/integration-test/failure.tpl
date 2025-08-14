channel: "${{ env.CHANNEL_ID }}"
text: "${{ env.MESSAGE_TEXT }}"
${{ env.SLACK_THREAD_YAML }}
blocks:
  - type: "section"
    text:
      type: "mrkdwn"
      text: "${{ env.HEALTH_SLACK_EMOJI }} (${{ env.HEALTH_RATE }}) The integration test failed because of the following individual tests:"
  - type: "rich_text"
    elements:
      - type: "rich_text_preformatted"
        elements:
          - type: "text"
            text: "${{ env.FAILED_TESTS }}"
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
