channel: "${{ env.CHANNEL_ID }}"
${{ env.SLACK_THREAD_YAML }}
blocks:
  - type: "section"
    text:
      type: "mrkdwn"
      text: "${{ env.MESSAGE_TEXT }}"
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
        url: "${{ env.WORKFLOW_RUN_URL }}"
      - type: button
        text:
          type: "plain_text"
          text: "View Dashboard"
          emoji: false
        url: "https://example.org"
