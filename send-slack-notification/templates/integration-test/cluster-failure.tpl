channel: "${CHANNEL_ID}"
${SLACK_THREAD_YAML}
blocks:
  - type: "section"
    text:
      type: "mrkdwn"
      text: "${MESSAGE_TEXT}"
  - type: "actions"
    elements:
      - type: button
        text:
          type: "plain_text"
          text: "View Workflow Run"
          emoji: false
        url: "${WORKFLOW_RUN_URL}"
