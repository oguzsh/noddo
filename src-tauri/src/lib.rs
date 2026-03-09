pub mod commands;
pub mod models;
pub mod server;
pub mod state;
pub mod tray;
pub mod window;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState::new();
    let server_state = state.clone();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .manage(state.clone())
        .invoke_handler(tauri::generate_handler![
            commands::resolve_permission,
            commands::get_pending_requests,
            commands::dismiss_request,
        ])
        .setup(move |app| {
            // Hide from Dock — this is a menu bar-only app
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Store the app handle synchronously BEFORE starting the server
            // to avoid a race where early requests see no handle
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async {
                state.set_app_handle(handle).await;
            });

            // Set up the system tray
            tray::setup_tray(app)?;

            // Start the HTTP server in a background task
            tauri::async_runtime::spawn(async move {
                server::start_server(server_state).await;
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            // Hide instead of closing — keeps the app alive in the menu bar
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
