#!/bin/bash

set -euo pipefail

REPO="moondance-labs/tanssi"
KEEP_PR_NUMBER="$1"

echo "🚨 Canceling all workflows except for PR #$KEEP_PR_NUMBER"

# Get all open PRs
gh pr list -R "$REPO" --state open --json number,headRefName | jq -c '.[]' | while read -r pr; do
  pr_number=$(echo "$pr" | jq -r '.number')
  branch=$(echo "$pr" | jq -r '.headRefName')

  if [[ "$pr_number" == "$KEEP_PR_NUMBER" ]]; then
    echo "✅ Keeping PR #$pr_number — branch: $branch"
    continue
  fi

  echo "🔍 Checking workflows for PR #$pr_number — branch: $branch"

  # Find all active runs for this branch
  gh run list -R "$REPO" --limit 100 \
    --json databaseId,headBranch,status,conclusion \
    | jq -c ".[] | select(.headBranch == \"$branch\" and (.status == \"queued\" or .status == \"in_progress\"))" \
    | while read -r run; do
        run_id=$(echo "$run" | jq -r '.databaseId')
        echo "❌ Canceling run ID $run_id"
        gh run cancel "$run_id" -R "$REPO"
    done
done

echo "✅ Done. Only PR #$KEEP_PR_NUMBER workflows remain active."
