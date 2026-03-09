# Noddo

A system tray app that intercepts Claude Code's permission prompts and displays them as desktop popups. Approve, reject, or explain — without switching to the terminal.

Works over full-screen apps, doesn't steal focus, no dock icon.

## How it works

```
Claude Code ──► PreToolUse hook ──► HTTP POST ──► Noddo (localhost:3025)
                                                       │
                                                  Show popup
                                                       │
                                              User clicks action
                                                       │
Claude Code ◄── Hook response ◄── HTTP response ◄─────┘
```

1. Claude Code pauses and sends the tool details to Noddo via a `PreToolUse` hook
2. Noddo holds the HTTP connection open and shows a popup below the tray icon
3. You pick an action — the HTTP response flows back and Claude continues (or stops)

## Actions

| Action | Shortcut | What it does |
|--------|----------|--------------|
| Allow | `y` | Approve this one request |
| Allow All | `a` | Auto-approve this tool for the rest of the session |
| Deny | `n` / `Esc` | Block the request |
| Explain | `e` | Block with a message — tell Claude what to do instead |

## Setup

### 1. Install Noddo

Build from source (see below) or copy `Noddo.app` to `/Applications`.

### 2. Configure Claude Code hook

Add this to your `~/.claude/settings.json`:

```json
{
  "hooks": {
    "PreToolUse": [
      {
        "matcher": "",
        "hooks": [
          {
            "type": "command",
            "command": "curl -s --connect-timeout 1 --max-time 300 -X POST -H 'Content-Type: application/json' -d @- http://127.0.0.1:3025/api/permission 2>/dev/null || true"
          }
        ]
      }
    ]
  }
}
```

The empty `matcher` fires for every tool use. If Noddo isn't running, `|| true` makes the hook exit silently and Claude falls back to its normal terminal prompt.

### 3. Start a new Claude Code session

Every tool use (Bash, Edit, Write, etc.) will now route through Noddo.

## Build from source

**Prerequisites:** Rust, Node.js, npm

```bash
# Install dependencies
npm install

# Development
npm run tauri dev

# Production build
npm run tauri build
```

The built app is at `src-tauri/target/release/bundle/macos/Noddo.app`.

## Tech stack

- **Framework:** Tauri v2
- **Backend:** Rust (axum HTTP server, tokio async)
- **Frontend:** React 19, TypeScript, Tailwind CSS
- **macOS overlay:** NSPanel via tauri-nspanel (shows over full-screen apps without stealing focus)
