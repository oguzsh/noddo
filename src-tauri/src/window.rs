use tauri::{Manager, WebviewWindow};

use crate::state::TrayPosition;

const WINDOW_WIDTH: f64 = 400.0;
const GAP: f64 = 4.0;

/// Position the popup window directly below the tray icon, centered horizontally.
pub fn position_below_tray(window: &WebviewWindow, tray_rect: &TrayPosition) {
    let x = tray_rect.x + (tray_rect.width / 2.0) - (WINDOW_WIDTH / 2.0);
    let y = tray_rect.y + tray_rect.height + GAP;

    let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
        x,
        y,
    }));
}

/// Show the window and focus it.
pub fn show_and_focus(window: &WebviewWindow) {
    let _ = window.show();
    let _ = window.set_focus();
}

/// Show the window anchored below the tray icon. Falls back to top-right if no tray position.
pub async fn show_anchored(app: &tauri::AppHandle, state: &crate::state::AppState) {
    let Some(window) = app.get_webview_window("permission") else {
        return;
    };

    if let Some(tray_rect) = state.get_tray_rect().await {
        position_below_tray(&window, &tray_rect);
    } else {
        // No tray click yet — place near top-right where the tray icon likely is
        let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
            x: 800.0,
            y: 30.0,
        }));
    }

    show_and_focus(&window);
}
