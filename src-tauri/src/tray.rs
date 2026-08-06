use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Manager,
};

pub fn create_tray(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let open_widget_i = MenuItem::with_id(app, "open_widget", "Open Widget", true, None::<&str>)?;
    let refresh_i = MenuItem::with_id(app, "refresh", "Refresh Now", true, None::<&str>)?;
    let always_on_top_i = MenuItem::with_id(app, "always_on_top", "Always On Top", true, None::<&str>)?;
    let settings_i = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Exit", true, None::<&str>)?;

    let separator = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
        app,
        &[
            &open_widget_i,
            &refresh_i,
            &always_on_top_i,
            &separator,
            &settings_i,
            &separator,
            &quit_i,
        ],
    )?;

    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip("OpenRouter Monitor")
        .menu(&menu)
        .on_menu_event(move |app, event| {
            let id = event.id().as_ref();
            match id {
                "open_widget" => {
                    if let Some(window) = app.get_webview_window("main-widget") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                "refresh" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let db = app_handle.state::<crate::storage::database::Database>();
                        let _ = crate::commands::dashboard::refresh_dashboard(db).await;
                    });
                }
                "always_on_top" => {
                    if let Some(window) = app.get_webview_window("main-widget") {
                        let current = window.is_always_on_top().unwrap_or(false);
                        let _ = window.set_always_on_top(!current);
                    }
                }
                "settings" => {
                    let app_handle = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = crate::commands::windows::show_settings(app_handle).await;
                    });
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main-widget") {
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app)?;

    Ok(())
}
