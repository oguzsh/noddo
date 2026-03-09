#!/usr/bin/env bash
set -euo pipefail

# Noddo Hook Bridge
# Reads Claude Code hook input from stdin, forwards to Noddo app via HTTP,
# and writes the response back to stdout.
# If Noddo is not running, exits 0 with no output (passthrough to terminal).

NODDO_PORT="${NODDO_PORT:-3025}"
NODDO_HOST="127.0.0.1"
NODDO_TIMEOUT="${NODDO_TIMEOUT:-300}"

# Read all of stdin
input=$(cat)

# Validate that input is valid JSON before forwarding
if ! echo "$input" | jq empty 2>/dev/null; then
  exit 0
fi

# Attempt to forward to Noddo
# --connect-timeout 1: fail fast if Noddo is not running
# --max-time: allow up to N seconds for user to decide
response=$(echo "$input" | curl -s \
  --connect-timeout 1 \
  --max-time "$NODDO_TIMEOUT" \
  -X POST \
  -H "Content-Type: application/json" \
  -d @- \
  "http://${NODDO_HOST}:${NODDO_PORT}/api/permission" 2>/dev/null) || {
    # Noddo not running or connection failed — passthrough
    exit 0
  }

# If empty response, passthrough
if [[ -z "$response" ]]; then
  exit 0
fi

# Output the hook response
echo "$response"
exit 0
