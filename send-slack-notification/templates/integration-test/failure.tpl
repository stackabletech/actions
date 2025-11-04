channel: "${CHANNEL_ID}"
${SLACK_THREAD_YAML}
blocks:
  - type: "section"
    text:
      type: "mrkdwn"
      text: "${MESSAGE_TEXT}"
  - type: "section"
    text:
      type: "mrkdwn"
      text: "${HEALTH_SLACK_EMOJI} (${HEALTH_RATE}) The integration test failed because of the following individual tests:"
  - type: "rich_text"
    elements:
      - type: "rich_text_preformatted"
        elements:
          - type: "text"
            text: "${FAILED_TESTS}"
  - type: "actions"
    elements:
      - type: button
        text:
          type: "plain_text"
          text: "View Workflow Run"
          emoji: false
        url: "${WORKFLOW_RUN_URL}"
      - type: button
        text:
          type: "plain_text"
          text: "View Dashboard"
          emoji: false
        url: "https://example.org"
