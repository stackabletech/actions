FROM alpine@sha256:beefdbd8a1da6d2915566fde36db9db0b524eb737fc57cd1367effd16dc0d06d as builder

ARG GITHUB_RUN_ATTEMPT="<unknown>"
ENV GITHUB_RUN_ATTEMPT=${GITHUB_RUN_ATTEMPT}

ARG GITHUB_RUN_ID="<unknown>"
ENV GITHUB_RUN_ID=${GITHUB_RUN_ID}

RUN <<EOF
touch workflow.txt
echo "Workflow run id: ${GITHUB_RUN_ID}" | tee -a workflow.txt
echo "Workflow run attempt: ${GITHUB_RUN_ATTEMPT}" | tee -a workflow.txt
EOF

FROM alpine@sha256:beefdbd8a1da6d2915566fde36db9db0b524eb737fc57cd1367effd16dc0d06d

COPY --from=builder workflow.txt workflow.txt
RUN <<EOF
echo "I'm a smoke test coming from https://github.com/stackabletech/actions"
cat workflow.txt
EOF
