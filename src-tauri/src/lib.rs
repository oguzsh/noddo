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

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ));

    #[cfg(target_os = "macos")]
    {
        builder = builder.plugin(tauri_nspanel::init());
    }

    builder
        .manage(state.clone())
        .invoke_handler(tauri::generate_handler![
            commands::resolve_permission,
            commands::get_pending_requests,
            commands::dismiss_request,
        ])
        .setup(move |app| {
            use tauri::Manager;

            // Store the app handle synchronously BEFORE starting the server
            let handle = app.handle().clone();
            tauri::async_runtime::block_on(async {
                state.set_app_handle(handle).await;
            });

            // Set up the system tray
            tray::setup_tray(app)?;

            // Convert the window to an NSPanel for full-screen overlay support
            #[cfg(target_os = "macos")]
            {
                use tauri::Emitter;
                use tauri_nspanel::cocoa::appkit::{
                    NSMainMenuWindowLevel, NSWindowCollectionBehavior,
                };
                use tauri_nspanel::{panel_delegate, WebviewWindowExt};

                #[allow(non_upper_case_globals)]
                const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;

                // Hide from Dock — menu bar-only app
                app.set_activation_policy(tauri::ActivationPolicy::Accessory);

                let window = app
                    .get_webview_window("permission")
                    .expect("permission window not found");

                let panel = window.to_panel().unwrap();

                // Float above the menu bar
                panel.set_level(NSMainMenuWindowLevel + 1);

                // NonActivatingPanel: shows without activating the app,
                // so it works from background HTTP triggers without needing
                // activateIgnoringOtherApps or Regular activation policy
                panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);

                // Present on all spaces including full-screen
                panel.set_collection_behaviour(
                    NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary,
                );

                // Keep visible when app is deactivated
                panel.set_hides_on_deactivate(false);

                // Delegate for focus events
                let delegate = panel_delegate!(NoddoPanelDelegate {
                    window_did_become_key,
                    window_did_resign_key
                });

                let app_handle = app.handle().clone();
                delegate.set_listener(Box::new(move |delegate_name: String| {
                    match delegate_name.as_str() {
                        "window_did_become_key" => {
                            let _ = app_handle.emit("macos-panel-focus", true);
                        }
                        "window_did_resign_key" => {
                            let _ = app_handle.emit("macos-panel-focus", false);
                        }
                        _ => (),
                    }
                }));

                panel.set_delegate(delegate);
            }

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
