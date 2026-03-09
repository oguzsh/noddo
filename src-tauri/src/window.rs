use tauri::Manager;

#[cfg(target_os = "macos")]
use tauri_nspanel::ManagerExt;

use crate::state::TrayPosition;

const WINDOW_WIDTH: f64 = 400.0;
const GAP: f64 = 4.0;

/// Position the popup window directly below the tray icon, centered horizontally.
pub fn position_below_tray(window: &tauri::WebviewWindow, tray_rect: &TrayPosition) {
    let x = tray_rect.x + (tray_rect.width / 2.0) - (WINDOW_WIDTH / 2.0);
    let y = tray_rect.y + tray_rect.height + GAP;

    let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition { x, y }));
}

/// Show the panel and bring it to front. Safe to call from any thread.
pub fn show_panel(app: &tauri::AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        #[cfg(target_os = "macos")]
        {
            if let Ok(panel) = handle.get_webview_panel("permission") {
                panel.show();
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            if let Some(win) = handle.get_webview_window("permission") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }
    });
}

/// Hide the panel. Safe to call from any thread.
pub fn hide_panel(app: &tauri::AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        #[cfg(target_os = "macos")]
        {
            if let Ok(panel) = handle.get_webview_panel("permission") {
                panel.order_out(None);
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            if let Some(win) = handle.get_webview_window("permission") {
                let _ = win.hide();
            }
        }
    });
}

/// Check if the panel is currently visible.
pub fn is_panel_visible(app: &tauri::AppHandle) -> bool {
    #[cfg(target_os = "macos")]
    {
        if let Ok(panel) = app.get_webview_panel("permission") {
            return panel.is_visible();
        }
        return false;
    }

    #[cfg(not(target_os = "macos"))]
    {
        app.get_webview_window("permission")
            .map(|w| w.is_visible().unwrap_or(false))
            .unwrap_or(false)
    }
}

/// Show the panel anchored below the tray icon. Safe to call from any thread.
pub async fn show_anchored(app: &tauri::AppHandle, state: &crate::state::AppState) {
    let tray_rect = state.get_tray_rect().await;
    let handle = app.clone();

    let _ = app.run_on_main_thread(move || {
        if let Some(window) = handle.get_webview_window("permission") {
            if let Some(rect) = tray_rect {
                position_below_tray(&window, &rect);
            } else {
                let _ = window.set_position(tauri::Position::Logical(tauri::LogicalPosition {
                    x: 800.0,
                    y: 30.0,
                }));
            }
        }

        #[cfg(target_os = "macos")]
        {
            if let Ok(panel) = handle.get_webview_panel("permission") {
                panel.show();
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            if let Some(win) = handle.get_webview_window("permission") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }
    });
}
