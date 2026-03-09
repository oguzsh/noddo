use axum::{extract::State, http::StatusCode, response::Json, routing::{get, post}, Router};
use tauri::Emitter;

use crate::models::{DecisionAction, HookInput, HookResponse, PermissionRequest};
use crate::state::AppState;

const HOST: &str = "127.0.0.1";
const PORT: u16 = 3025;

pub fn create_router(state: AppState) -> Router {
    // No CORS layer needed — the only client is curl from the hook script,
    // which is not subject to CORS. Omitting CORS headers means browsers
    // cannot make cross-origin requests to this server at all.
    Router::new()
        .route("/api/health", get(health))
        .route("/api/permission", post(handle_permission))
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "version": "0.1.0"
    }))
}

async fn handle_permission(
    State(state): State<AppState>,
    Json(input): Json<HookInput>,
) -> Result<Json<HookResponse>, StatusCode> {
    // If this tool is in the auto-allow list, approve immediately
    if state.is_auto_allowed(&input.tool_name).await {
        tracing::info!("Auto-allowing {} (Allow All active)", input.tool_name);
        return Ok(Json(HookResponse::allow()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let request = PermissionRequest {
        id: id.clone(),
        tool_name: input.tool_name,
        tool_input: input.tool_input,
        received_at: now,
    };

    // Emit event to the Tauri frontend and show window anchored below tray
    if let Some(app) = state.get_app_handle().await {
        if let Err(e) = app.emit("new-permission-request", &request) {
            tracing::error!("Failed to emit permission request event: {}", e);
        }
        crate::window::show_anchored(&app, &state).await;

        // Play macOS notification sound
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("afplay")
                .arg("/System/Library/Sounds/Submarine.aiff")
                .spawn();
        }
    } else {
        tracing::warn!("App handle not yet available — permission request may not show UI");
    }

    // Insert and await the user's decision
    let rx = state.insert_request(request).await;

    match rx.await {
        Ok(decision) => {
            let response = match decision.action {
                DecisionAction::Allow | DecisionAction::AllowAll => HookResponse::allow(),
                DecisionAction::Bypass => HookResponse::allow(),
                DecisionAction::Deny | DecisionAction::Block => {
                    HookResponse::block(decision.reason)
                }
            };
            Ok(Json(response))
        }
        Err(_) => {
            // Channel was dropped (e.g., request dismissed or app closing)
            // Return empty response so hook script passes through
            Err(StatusCode::GONE)
        }
    }
}

pub async fn start_server(state: AppState) {
    let addr = format!("{}:{}", HOST, PORT);
    let app = create_router(state);

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!(
                "Failed to bind to {}. Is another Noddo instance running? Error: {}",
                addr,
                e
            );
            return;
        }
    };

    tracing::info!("Noddo HTTP server listening on {}", addr);

    if let Err(e) = axum::serve(listener, app).await {
        tracing::error!("HTTP server error: {}", e);
    }
}
