use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{oneshot, Mutex};

use crate::models::{PermissionDecision, PermissionRequest};

pub struct PendingRequest {
    pub request: PermissionRequest,
    pub sender: oneshot::Sender<PermissionDecision>,
}

/// Stored position of the tray icon for anchoring the popup window
#[derive(Clone, Copy)]
pub struct TrayPosition {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct AppState {
    pub pending: Arc<Mutex<HashMap<String, PendingRequest>>>,
    pub app_handle: Arc<Mutex<Option<tauri::AppHandle>>>,
    /// Tool names that are auto-allowed for the remainder of this session
    pub auto_allowed: Arc<Mutex<HashSet<String>>>,
    /// Last known tray icon rect, used to position window below the icon
    pub tray_rect: Arc<Mutex<Option<TrayPosition>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            pending: Arc::new(Mutex::new(HashMap::new())),
            app_handle: Arc::new(Mutex::new(None)),
            auto_allowed: Arc::new(Mutex::new(HashSet::new())),
            tray_rect: Arc::new(Mutex::new(None)),
        }
    }

    pub async fn set_tray_rect(&self, pos: TrayPosition) {
        *self.tray_rect.lock().await = Some(pos);
    }

    pub async fn get_tray_rect(&self) -> Option<TrayPosition> {
        *self.tray_rect.lock().await
    }

    pub async fn is_auto_allowed(&self, tool_name: &str) -> bool {
        self.auto_allowed.lock().await.contains(tool_name)
    }

    pub async fn add_auto_allow(&self, tool_name: String) {
        self.auto_allowed.lock().await.insert(tool_name);
    }

    pub async fn insert_request(
        &self,
        request: PermissionRequest,
    ) -> oneshot::Receiver<PermissionDecision> {
        let (tx, rx) = oneshot::channel();
        let id = request.id.clone();
        let pending = PendingRequest {
            request,
            sender: tx,
        };
        self.pending.lock().await.insert(id, pending);
        rx
    }

    pub async fn resolve(&self, decision: PermissionDecision) -> Result<(), String> {
        let mut map = self.pending.lock().await;
        let pending = map
            .remove(&decision.id)
            .ok_or_else(|| format!("No pending request with id: {}", decision.id))?;
        pending
            .sender
            .send(decision)
            .map_err(|_| "Failed to send decision through channel".to_string())
    }

    pub async fn get_pending_requests(&self) -> Vec<PermissionRequest> {
        self.pending
            .lock()
            .await
            .values()
            .map(|p| p.request.clone())
            .collect()
    }

    pub async fn set_app_handle(&self, handle: tauri::AppHandle) {
        *self.app_handle.lock().await = Some(handle);
    }

    pub async fn get_app_handle(&self) -> Option<tauri::AppHandle> {
        self.app_handle.lock().await.clone()
    }
}
