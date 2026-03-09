use tauri::{Manager, State};

use crate::models::{DecisionAction, PermissionDecision, PermissionRequest};
use crate::state::AppState;

#[tauri::command]
pub async fn resolve_permission(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    id: String,
    action: String,
    reason: Option<String>,
    tool_name: Option<String>,
) -> Result<(), String> {
    let decision_action = match action.as_str() {
        "allow" => DecisionAction::Allow,
        "allow_all" => {
            // Add tool to auto-allow list for this session
            if let Some(name) = tool_name {
                state.add_auto_allow(name).await;
            }
            DecisionAction::AllowAll
        }
        "deny" => DecisionAction::Deny,
        "block" => DecisionAction::Block,
        "bypass" => DecisionAction::Bypass,
        _ => return Err(format!("Invalid action: {}", action)),
    };

    let decision = PermissionDecision {
        id,
        action: decision_action,
        reason,
    };

    state.resolve(decision).await?;

    // Hide window if no more pending requests
    let remaining = state.get_pending_requests().await;
    if remaining.is_empty() {
        if let Some(window) = app.get_webview_window("permission") {
            let _ = window.hide();
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn get_pending_requests(
    state: State<'_, AppState>,
) -> Result<Vec<PermissionRequest>, String> {
    Ok(state.get_pending_requests().await)
}

#[tauri::command]
pub async fn dismiss_request(
    state: State<'_, AppState>,
    app: tauri::AppHandle,
    id: String,
) -> Result<(), String> {
    // Remove from pending without sending a decision (channel drops, handler returns GONE)
    let mut map = state.pending.lock().await;
    map.remove(&id);
    drop(map);

    let remaining = state.get_pending_requests().await;
    if remaining.is_empty() {
        if let Some(window) = app.get_webview_window("permission") {
            let _ = window.hide();
        }
    }

    Ok(())
}
