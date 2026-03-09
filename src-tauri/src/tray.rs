use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    App, Manager,
};
use tauri_plugin_autostart::ManagerExt;

use crate::state::{AppState, TrayPosition};
use crate::window;

pub fn setup_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let autostart_mgr = app.autolaunch();
    let is_enabled = autostart_mgr.is_enabled().unwrap_or(false);

    let launch_at_login = CheckMenuItem::with_id(
        app,
        "launch_at_login",
        "Launch at Login",
        true,
        is_enabled,
        None::<&str>,
    )?;
    let separator = PredefinedMenuItem::separator(app)?;
    let quit = MenuItem::with_id(app, "quit", "Quit Noddo", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&launch_at_login, &separator, &quit])?;

    TrayIconBuilder::with_id("main")
        .icon(
            app.default_window_icon()
                .ok_or("Default window icon not found")?
                .clone(),
        )
        .tooltip("Noddo - Permission Manager")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            if event.id() == "quit" {
                app.exit(0);
            } else if event.id() == "launch_at_login" {
                let autostart_mgr = app.autolaunch();
                let currently_enabled = autostart_mgr.is_enabled().unwrap_or(false);
                if currently_enabled {
                    let _ = autostart_mgr.disable();
                } else {
                    let _ = autostart_mgr.enable();
                }
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let tauri::tray::TrayIconEvent::Click {
                button: tauri::tray::MouseButton::Left,
                rect,
                ..
            } = event
            {
                let app = tray.app_handle();

                let scale = get_scale(&app);
                let tray_pos = rect_to_tray_position(&rect, scale);
                let state = app.state::<AppState>();
                tauri::async_runtime::block_on(async {
                    state.set_tray_rect(tray_pos).await;
                });

                if let Some(win) = app.get_webview_window("permission") {
                    if win.is_visible().unwrap_or(false) {
                        let _ = win.hide();
                    } else {
                        window::position_below_tray(&win, &tray_pos);
                        window::show_and_focus(&win);
                    }
                }
            }
        })
        .build(app)?;

    // Store the initial tray position so HTTP-triggered popups
    // can anchor correctly before any tray click
    if let Some(tray) = app.tray_by_id("main") {
        if let Ok(Some(rect)) = tray.rect() {
            let handle = app.handle();
            let scale = get_scale(handle);
            let tray_pos = rect_to_tray_position(&rect, scale);
            let state = app.state::<AppState>();
            tauri::async_runtime::block_on(async {
                state.set_tray_rect(tray_pos).await;
            });
        }
    }

    Ok(())
}

fn get_scale(handle: &tauri::AppHandle) -> f64 {
    handle
        .primary_monitor()
        .ok()
        .flatten()
        .map(|m| m.scale_factor())
        .unwrap_or(1.0)
}

fn rect_to_tray_position(rect: &tauri::Rect, scale: f64) -> TrayPosition {
    let (px, py) = match rect.position {
        tauri::Position::Logical(p) => (p.x, p.y),
        tauri::Position::Physical(p) => (p.x as f64 / scale, p.y as f64 / scale),
    };
    let (sw, sh) = match rect.size {
        tauri::Size::Logical(s) => (s.width, s.height),
        tauri::Size::Physical(s) => (s.width as f64 / scale, s.height as f64 / scale),
    };
    TrayPosition {
        x: px,
        y: py,
        width: sw,
        height: sh,
    }
}
