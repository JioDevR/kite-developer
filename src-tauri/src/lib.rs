pub mod commands;
pub mod managers;
pub mod models;

use tauri::{LogicalPosition, LogicalSize, Manager, WebviewBuilder, WebviewUrl, WindowEvent};
use managers::webview_manager::{resize_layout, start_auto_suspend_watchdog, ActiveServiceState};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ActiveServiceState::default())
        .setup(|app| {
            let width = 1280.0;
            let height = 800.0;

            // 1. Crear ventana principal nativa
            let window = tauri::Window::builder(app, "main")
                .title("Kite")
                .inner_size(width, height)
                .min_inner_size(900.0, 600.0)
                .build()?;

            // 2. En Linux (GTK), configurar el contenedor principal en orientación Horizontal
            // para que los Webviews hijos se ubiquen de izquierda a derecha (Sidebar al lado del servicio)
            #[cfg(target_os = "linux")]
            {
                use gtk::prelude::*;
                if let Ok(vbox) = window.default_vbox() {
                    vbox.set_orientation(gtk::Orientation::Horizontal);
                }
            }

            // 3. Añadir el webview de la barra lateral (Sidebar) fijo a 68 px
            let _ui_webview = window.add_child(
                WebviewBuilder::new("ui", WebviewUrl::App("index.html?view=sidebar".into())),
                LogicalPosition::new(0.0, 0.0),
                LogicalSize::new(68.0, height),
            )?;

            // 4. Añadir el webview de la pantalla de bienvenida (Welcome) en el canvas principal
            let _welcome_webview = window.add_child(
                WebviewBuilder::new("welcome", WebviewUrl::App("index.html?view=welcome".into())),
                LogicalPosition::new(68.0, 0.0),
                LogicalSize::new(width - 68.0, height),
            )?;

            let handle = app.handle().clone();
            start_auto_suspend_watchdog(handle);

            Ok(())
        })
        .on_menu_event(|app, event| {
            let id = event.id().0.as_str();
            if let Some((action, service_id)) = id.split_once(':') {
                match action {
                    "reload" => {
                        let _ = commands::reload_service(app.clone(), service_id.to_string(), false);
                    }
                    "suspend" => {
                        let _ = commands::suspend_service(app.clone(), service_id.to_string());
                    }
                    "delete" => {
                        let _ = commands::delete_service(app.clone(), service_id.to_string());
                    }
                    _ => {}
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_services,
            commands::create_service,
            commands::activate_service,
            commands::suspend_service,
            commands::delete_service,
            commands::reload_service,
            commands::toggle_modal_overlay,
            commands::open_add_service_modal,
            commands::close_add_service_modal,
            commands::show_service_context_menu,
            commands::update_service_badge
        ])
        .on_window_event(|window, event| {
            if let WindowEvent::Resized(..) = event {
                let app = window.app_handle();
                let _ = resize_layout(app);
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running Kite application");
}
