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

/// Position the panel on whichever monitor currently has the cursor.
/// Uses native Cocoa coordinates (bottom-left origin) via setFrame:display:.
/// Must be called on the main thread.
#[cfg(target_os = "macos")]
fn position_on_cursor_monitor(window: &tauri::WebviewWindow) {
    use tauri_nspanel::cocoa::base::id;
    use tauri_nspanel::cocoa::foundation::{NSPoint, NSRect};
    use tauri_nspanel::objc::{class, msg_send, runtime::NO, sel, sel_impl};

    let Some(mon) = monitor::get_monitor_with_cursor() else {
        return;
    };

    let scale = mon.scale_factor();
    let area = mon.visible_area();
    let mon_pos = area.position().to_logical::<f64>(scale);
    let mon_size = area.size().to_logical::<f64>(scale);

    let mouse_loc: NSPoint = unsafe { msg_send![class!(NSEvent), mouseLocation] };

    let handle: id = window.ns_window().unwrap() as _;
    let mut frame: NSRect = unsafe { msg_send![handle, frame] };

    // Y: flush against top of visible area (just below menu bar)
    frame.origin.y = (mon_pos.y + mon_size.height) - frame.size.height;

    // X: centered on cursor, clamped to monitor bounds
    let mut x = mouse_loc.x - (frame.size.width / 2.0);
    let right_edge = mon_pos.x + mon_size.width;
    if x + frame.size.width > right_edge {
        x = right_edge - frame.size.width;
    }
    if x < mon_pos.x {
        x = mon_pos.x;
    }
    frame.origin.x = x;

    let _: () = unsafe { msg_send![handle, setFrame: frame display: NO] };
}

/// Show the panel on the monitor with the cursor. Safe to call from any thread.
pub async fn show_anchored(app: &tauri::AppHandle, _state: &crate::state::AppState) {
    let handle = app.clone();

    let _ = app.run_on_main_thread(move || {
        #[cfg(target_os = "macos")]
        {
            if let Some(window) = handle.get_webview_window("permission") {
                position_on_cursor_monitor(&window);
            }
            if let Ok(panel) = handle.get_webview_panel("permission") {
                panel.show();
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            if let Some(win) = handle.get_webview_window("permission") {
                let _ = win.set_position(tauri::Position::Logical(tauri::LogicalPosition {
                    x: 800.0,
                    y: 30.0,
                }));
                let _ = win.show();
                let _ = win.set_focus();
            }
        }
    });
}
