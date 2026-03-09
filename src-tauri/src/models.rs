use serde::{Deserialize, Serialize};

/// Raw input received from Claude Code's hook system on stdin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookInput {
    pub tool_name: String,
    pub tool_input: serde_json::Value,
    #[serde(default)]
    pub hook_event_name: Option<String>,
}

/// Internal representation of a pending permission request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionRequest {
    pub id: String,
    pub tool_name: String,
    pub tool_input: serde_json::Value,
    pub received_at: String,
}

/// Decision sent from the frontend via Tauri command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionDecision {
    pub id: String,
    pub action: DecisionAction,
    #[serde(default)]
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionAction {
    Allow,
    AllowAll,
    Deny,
    Block,
    Bypass,
}

/// Response sent back to the hook script as HTTP response body
#[derive(Debug, Serialize, Deserialize)]
pub struct HookResponse {
    #[serde(rename = "hookSpecificOutput")]
    pub hook_specific_output: HookSpecificOutput,
    #[serde(rename = "systemMessage", skip_serializing_if = "Option::is_none")]
    pub system_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HookSpecificOutput {
    #[serde(rename = "hookEventName")]
    pub hook_event_name: String,
    #[serde(rename = "permissionDecision")]
    pub permission_decision: String,
}

impl HookResponse {
    pub fn allow() -> Self {
        Self {
            hook_specific_output: HookSpecificOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "allow".to_string(),
            },
            system_message: None,
        }
    }

    pub fn block(reason: Option<String>) -> Self {
        Self {
            hook_specific_output: HookSpecificOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "block".to_string(),
            },
            system_message: reason,
        }
    }
}
