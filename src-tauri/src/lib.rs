use tauri::Emitter;
use tauri::Manager;
mod commands;
mod error;
mod openrouter;
mod storage;
mod tray;

pub use error::{AppError, AppResult};

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_autostart::init(tauri_plugin_autostart::MacosLauncher::LaunchAgent, None))
        .plugin(
            tauri_plugin_window_state::Builder::new()
                .skip_initial_state("main-widget")
                .build(),
        )
        .setup(|app| {
            // Initialize database
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");

            std::fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");

            let db_path = app_data_dir.join("openrouter-monitor.db");
            let conn = rusqlite::Connection::open(&db_path)
                .expect("Failed to open database");

            // Run migrations
            let migration_sql = include_str!("../migrations/001_initial.sql");
            conn.execute_batch(migration_sql)
                .expect("Failed to run migrations");

            let db = storage::database::Database::new(conn);
            app.manage(db);

            // Create system tray
            tray::create_tray(app)?;

            // Show the main widget window and apply saved window settings
            if let Some(window) = app.get_webview_window("main-widget") {
                let mut s = crate::storage::settings::get_settings().unwrap_or_default();
                if s.launch_at_startup {
                    let _ = tauri_plugin_autostart::ManagerExt::autolaunch(app).enable();
                } else {
                    let _ = tauri_plugin_autostart::ManagerExt::autolaunch(app).disable();
                }
                if s.show_in_taskbar {
                    s.show_in_taskbar = false;
                    let _ = crate::storage::settings::save_settings(&s);
                }
                let _ = window.set_always_on_top(s.always_on_top);
                let _ = window.set_skip_taskbar(!s.show_in_taskbar);
                let _ = window.center();
                let _ = window.show();
                let _ = window.set_focus();
            }

            // Handle close request for close-to-tray behavior
            let app_handle = app.handle().clone();
            if let Some(window) = app.get_webview_window("main-widget") {
                let handle = app_handle.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent closing, hide instead
                        api.prevent_close();
                        if let Some(win) = handle.get_webview_window("main-widget") {
                            let _ = win.hide();
                        }
                    }
                });
            }

            let startup_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                let should_check = crate::storage::settings::get_settings()
                    .ok()
                    .and_then(|settings| settings.last_update_check_at)
                    .map(|last| chrono::DateTime::parse_from_rfc3339(&last)
                        .map(|timestamp| chrono::Utc::now().signed_duration_since(timestamp.with_timezone(&chrono::Utc)).num_days() >= 15)
                        .unwrap_or(true))
                    .unwrap_or(true);
                if should_check {
                    if let Ok(info) = commands::update::check_for_updates().await {
                        let _ = startup_handle.emit("update-status", info);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::app_state::get_app_state,
            commands::credentials::validate_and_save_credential,
            commands::credentials::replace_credential,
            commands::credentials::forget_credential,
            commands::credentials::reset_app_data,
            commands::dashboard::refresh_dashboard,
            commands::dashboard::get_cached_dashboard,
            commands::history::get_usage_history,
            commands::history::export_usage_history_csv,
            commands::history::clear_usage_history,
            commands::settings::get_settings,
            commands::settings::save_settings,
            commands::update::check_for_updates,
            commands::update::download_and_install_update,
            commands::update::get_update_status,
            commands::windows::show_widget,
            commands::windows::show_settings,
            commands::windows::quit_application,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
