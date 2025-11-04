channel: "${CHANNEL_ID}"
text: "${MESSAGE_TEXT}"
${SLACK_THREAD_YAML}
attachments:
  - pretext: "See the details below for a summary of which job(s) ${MESSAGE_VERB}."
    color: "${MESSAGE_COLOR}"
    fields:
      - title: Build/Publish Image
        short: true
        value: "${BUILD_RESULT}"
      - title: Build/Publish Manifests
        short: true
        value: "${PUBLISH_MANIFESTS_RESULT}"
    actions:
      - type: button
        text: Go to workflow run
        url: "${WORKFLOW_RUN_URL}"
