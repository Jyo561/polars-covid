#!/data/data/com.termux/files/usr/bin/bash

REPO="Jyo561/polars-covid"
BINARY="polars-covid"

echo "🔎 Getting latest successful run..."

RUN_ID=$(gh run list \
  --repo $REPO \
  --workflow "Remote Android Build" \
  --json databaseId,conclusion \
  -q '.[] | select(.conclusion=="success") | .databaseId' \
  | head -n 1)

if [ -z "$RUN_ID" ]; then
  echo "❌ No successful run found."
  exit 1
fi

echo "⬇ Downloading artifact from run $RUN_ID..."

gh run download $RUN_ID --repo $REPO

chmod +x $BINARY

echo "🚀 Running binary..."
./$BINARY
