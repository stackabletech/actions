channel: "${{ inputs.channel-id }}"
text: "${{ env.MESSAGE_TEXT }}"
${{ steps.retrieve-slack-thread-id.outcome == 'success' && format('thread_ts: "{0}"', env.SLACK_THREAD_ID) || '' }}
attachments:
  - pretext: "See the details below for a summary of which job(s) ${{ env.MESSAGE_VERB }}."
    color: "${{ env.MESSAGE_COLOR }}"
    fields:
      - title: Build/Publish Image
        short: true
        value: "${{ inputs.build-result }}"
      - title: Build/Publish Manifests
        short: true
        value: "${{ inputs.publish-manifests-result }}"
    actions:
      - type: button
        text: Go to workflow run
        url: "${{ github.server_url }}/${{ github.repository }}/actions/runs/${{ github.run_id }}/attempts/${{ github.run_attempt }}"
