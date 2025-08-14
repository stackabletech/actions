channel: "${{ env.CHANNEL_ID }}"
text: "${{ env.MESSAGE_TEXT }}"
${{ env.SLACK_THREAD_YAML }}
attachments:
  - pretext: "See the details below for a summary of which job(s) ${{ env.MESSAGE_VERB }}."
    color: "${{ env.MESSAGE_COLOR }}"
    fields:
      - title: Build/Publish Image
        short: true
        value: "${{ env.BUILD_RESULT }}"
      - title: Build/Publish Manifests
        short: true
        value: "${{ env.PUBLISH_MANIFESTS_RESULT }}"
    actions:
      - type: button
        text: Go to workflow run
        url: "${{ env.WORKFLOW_RUN_URL }}"
