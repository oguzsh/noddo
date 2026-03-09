#!/usr/bin/env bash
set -euo pipefail

# Noddo Hook Installer
# Registers Noddo hooks in ~/.claude/settings.json

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
HOOK_SCRIPT="$SCRIPT_DIR/noddo-hook.sh"
SETTINGS_FILE="$HOME/.claude/settings.json"

# Ensure the hook script is executable
chmod +x "$HOOK_SCRIPT"

if ! command -v jq &>/dev/null; then
  echo "Error: jq is required. Install it with: brew install jq"
  exit 1
fi

if [[ ! -f "$SETTINGS_FILE" ]]; then
  echo "Error: $SETTINGS_FILE not found"
  exit 1
fi

echo "Installing Noddo hooks..."
echo "Hook script: $HOOK_SCRIPT"
echo "Settings file: $SETTINGS_FILE"

# Create a backup
cp "$SETTINGS_FILE" "${SETTINGS_FILE}.bak"

# Build the hook entry
HOOK_ENTRY=$(jq -n --arg cmd "$HOOK_SCRIPT" '{
  type: "command",
  command: $cmd
}')

# Tools to intercept
TOOLS=("Bash" "Write" "Edit")

for tool in "${TOOLS[@]}"; do
  echo "  Adding PreToolUse hook for: $tool"

  # Check if PreToolUse array exists, create if not
  if ! jq -e '.hooks.PreToolUse' "$SETTINGS_FILE" &>/dev/null; then
    jq '.hooks.PreToolUse = []' "$SETTINGS_FILE" > "${SETTINGS_FILE}.tmp"
    mv "${SETTINGS_FILE}.tmp" "$SETTINGS_FILE"
  fi

  # Check if a Noddo hook already exists for this tool
  EXISTING=$(jq --arg tool "$tool" --arg cmd "$HOOK_SCRIPT" '
    .hooks.PreToolUse // [] | map(
      select(.matcher == $tool and (.hooks // [] | any(.command == $cmd)))
    ) | length
  ' "$SETTINGS_FILE")

  if [[ "$EXISTING" -gt 0 ]]; then
    echo "    Already registered, skipping"
    continue
  fi

  # Add the hook entry
  jq --arg tool "$tool" --argjson hook "$HOOK_ENTRY" '
    .hooks.PreToolUse += [{
      matcher: $tool,
      hooks: [$hook],
      description: ("Noddo: Desktop permission popup for " + $tool + " operations")
    }]
  ' "$SETTINGS_FILE" > "${SETTINGS_FILE}.tmp"
  mv "${SETTINGS_FILE}.tmp" "$SETTINGS_FILE"
done

echo ""
echo "Done! Noddo hooks installed."
echo "Backup saved to: ${SETTINGS_FILE}.bak"
echo ""
echo "To uninstall, restore the backup:"
echo "  cp ${SETTINGS_FILE}.bak ${SETTINGS_FILE}"
