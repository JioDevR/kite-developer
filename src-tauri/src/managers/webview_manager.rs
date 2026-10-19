use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Instant;
use tauri::{
    AppHandle, Emitter, LogicalPosition, LogicalSize, Manager, WebviewBuilder, WebviewUrl,
};
use crate::models::service::{ServiceInstance, ServiceState};
use crate::managers::profile_storage::{get_profile_dir, load_services, save_services};

pub const SIDEBAR_WIDTH: f64 = 68.0;

#[cfg(target_os = "linux")]
pub const SPOOFED_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

#[cfg(target_os = "macos")]
pub const SPOOFED_USER_AGENT: &str =
    "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

#[cfg(target_os = "windows")]
pub const SPOOFED_USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
pub const SPOOFED_USER_AGENT: &str =
    "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36";

pub struct ActiveServiceState {
    pub active_id: Mutex<Option<String>>,
    pub is_modal_open: Mutex<bool>,
    pub background_since: Mutex<HashMap<String, Instant>>,
}

impl Default for ActiveServiceState {
    fn default() -> Self {
        Self {
            active_id: Mutex::new(None),
            is_modal_open: Mutex::new(false),
            background_since: Mutex::new(HashMap::new()),
        }
    }
}

pub fn service_label(service_id: &str) -> String {
    format!("srv_{}", service_id)
}

/// Sincroniza el estado (Active, Background, Suspended) de cada servicio según el runtime real
pub fn sync_service_runtime_states(app: &AppHandle, services: &mut [ServiceInstance]) {
    let state = app.state::<ActiveServiceState>();
    let active_id = state.active_id.lock().unwrap().clone();

    for service in services.iter_mut() {
        let label = service_label(&service.id);
        if active_id.as_deref() == Some(&service.id) {
            service.state = ServiceState::Active;
        } else if app.get_webview(&label).is_some() {
            service.state = ServiceState::Background;
        } else {
            service.state = ServiceState::Suspended;
        }
    }
}

/// Carga los servicios desde disco y sincroniza sus estados en memoria con el runtime
pub fn get_services_synced(app: &AppHandle) -> Result<Vec<ServiceInstance>, String> {
    let mut services = load_services(app)?;
    sync_service_runtime_states(app, &mut services);
    Ok(services)
}

/// Extrae el conteo de no leídos analizando el título de la página (ej: "(3) WhatsApp", "[1] Discord", "• Telegram")
pub fn parse_unread_from_title(title: &str) -> u32 {
    if let Some(start) = title.find(|c| c == '(' || c == '[') {
        if let Some(end) = title[start + 1..].find(|c| c == ')' || c == ']') {
            let inner = &title[start + 1..start + 1 + end];
            if let Ok(num) = inner.trim().parse::<u32>() {
                return num;
            }
        }
    }
    if title.starts_with('•') || title.starts_with('●') || title.contains(" • ") {
        return 1;
    }
    0
}

/// Actualiza el badge de no leídos de un servicio por su ID y notifica a la UI de Kite
pub fn update_service_badge_by_id(app: &AppHandle, service_id: &str, count: u32) -> Result<(), String> {
    let mut services = load_services(app)?;
    if let Some(s) = services.iter_mut().find(|s| s.id == service_id) {
        if s.unread_count != count {
            s.unread_count = count;
            sync_service_runtime_states(app, &mut services);
            save_services(app, &services)?;
            let _ = app.emit_to("ui", "services_updated", &services);
        }
    }
    Ok(())
}

/// Obtiene las dimensiones lógicas de la ventana principal
pub fn get_window_dimensions(app: &AppHandle) -> Result<(f64, f64), String> {
    let main_window = app
        .get_window("main")
        .ok_or_else(|| "Ventana principal no encontrada".to_string())?;

    let inner_size = main_window.inner_size().map_err(|e| e.to_string())?;
    let scale_factor = main_window.scale_factor().unwrap_or(1.0);

    let logical_w = inner_size.width as f64 / scale_factor;
    let logical_h = inner_size.height as f64 / scale_factor;

    Ok((logical_w.max(200.0), logical_h.max(200.0)))
}

/// Sincroniza la posición y tamaño de la barra lateral (UI de 68px) y la vista principal (Welcome o Servicio activo)
pub fn resize_layout(app: &AppHandle) -> Result<(), String> {
    let state = app.state::<ActiveServiceState>();
    let active_id = state.active_id.lock().unwrap().clone();
    let (win_w, win_h) = get_window_dimensions(app)?;
    let srv_w = (win_w - SIDEBAR_WIDTH).max(100.0);

    #[cfg(target_os = "linux")]
    {
        use gtk::prelude::*;
        if let Some(main_window) = app.get_window("main") {
            if let Ok(vbox) = main_window.default_vbox() {
                vbox.set_orientation(gtk::Orientation::Horizontal);

                let children = vbox.children();
                // 1. Sidebar (Child 0): SIEMPRE fija a 68 px de ancho, nunca expandir ni llenar
                if let Some(ui_widget) = children.first() {
                    vbox.set_child_packing(ui_widget, false, false, 0, gtk::PackType::Start);
                    ui_widget.set_size_request(SIDEBAR_WIDTH as i32, -1);
                    ui_widget.show();
                }

                // 2. Todos los webviews de contenido (Welcome y Servicios) expanden al 100% restante
                for srv_widget in children.iter().skip(1) {
                    vbox.set_child_packing(srv_widget, true, true, 0, gtk::PackType::Start);
                }
            }
        }
    }

    // Posicionamiento explícito permanente de la barra lateral (68 px)
    if let Some(ui_wv) = app.get_webview("ui") {
        let _ = ui_wv.set_position(LogicalPosition::new(0.0, 0.0));
        let _ = ui_wv.set_size(LogicalSize::new(SIDEBAR_WIDTH, win_h));
        let _ = ui_wv.show();
    }

    // Manejo de la pantalla principal: Servicio activo vs Pantalla de bienvenida
    if let Some(active_id) = &active_id {
        let active_label = service_label(active_id);

        // Ocultar pantalla de bienvenida cuando hay un servicio activo
        if let Some(welcome_wv) = app.get_webview("welcome") {
            let _ = welcome_wv.hide();
        }

        // Mostrar únicamente el servicio activo en todo el espacio restante
        for (label, wv) in app.webviews() {
            if label.starts_with("srv_") {
                if label == active_label {
                    let _ = wv.set_position(LogicalPosition::new(SIDEBAR_WIDTH, 0.0));
                    let _ = wv.set_size(LogicalSize::new(srv_w, win_h));
                    let _ = wv.show();
                    let _ = wv.set_focus();
                } else {
                    let _ = wv.hide();
                }
            }
        }
    } else {
        // Ningún servicio activo: ocultar todos los servicios y mostrar pantalla de bienvenida
        for (label, wv) in app.webviews() {
            if label.starts_with("srv_") {
                let _ = wv.hide();
            }
        }

        if let Some(welcome_wv) = app.get_webview("welcome") {
            let _ = welcome_wv.set_position(LogicalPosition::new(SIDEBAR_WIDTH, 0.0));
            let _ = welcome_wv.set_size(LogicalSize::new(srv_w, win_h));
            let _ = welcome_wv.show();
            let _ = welcome_wv.set_focus();
        }
    }

    Ok(())
}

pub fn activate_service(app: &AppHandle, service: &ServiceInstance) -> Result<(), String> {
    let main_window = app
        .get_window("main")
        .ok_or_else(|| "Ventana principal no encontrada".to_string())?;

    let target_label = service_label(&service.id);
    let state = app.state::<ActiveServiceState>();

    // Registrar servicio anterior como background timestamp
    {
        let mut active_id_guard = state.active_id.lock().unwrap();
        if let Some(ref old_id) = *active_id_guard {
            if old_id != &service.id {
                state
                    .background_since
                    .lock()
                    .unwrap()
                    .insert(old_id.clone(), Instant::now());
            }
        }
        *active_id_guard = Some(service.id.clone());
    }

    // Quitar de background tracking el recién activado
    state
        .background_since
        .lock()
        .unwrap()
        .remove(&service.id);

    // Ocultar todos los otros webviews de servicios existentes
    for (label, wv) in app.webviews() {
        if label.starts_with("srv_") && label != target_label {
            let _ = wv.hide();
        }
    }

    // Si ya existe en RAM, sincronizar layout
    if app.get_webview(&target_label).is_some() {
        resize_layout(app)?;
        return Ok(());
    }

    // Si estaba suspendido, instanciar nuevo Webview con profile aislado
    let profile_path = get_profile_dir(app, &service.id)?;
    let (win_w, win_h) = get_window_dimensions(app)?;
    let srv_w = (win_w - SIDEBAR_WIDTH).max(100.0);

    let parsed_url: tauri::Url = service
        .url
        .parse()
        .map_err(|e: url::ParseError| format!("URL inválida {}: {}", service.url, e))?;

    let preload_script = include_str!("../scripts/preload_inject.js");

    let app_for_title = app.clone();
    let service_id_for_title = service.id.clone();

    let webview_builder = WebviewBuilder::new(&target_label, WebviewUrl::External(parsed_url))
        .data_directory(profile_path)
        .user_agent(SPOOFED_USER_AGENT)
        .initialization_script(preload_script)
        .on_document_title_changed(move |_wv, title| {
            let count = parse_unread_from_title(&title);
            let _ = update_service_badge_by_id(&app_for_title, &service_id_for_title, count);
        });

    main_window
        .add_child(
            webview_builder,
            LogicalPosition::new(SIDEBAR_WIDTH, 0.0),
            LogicalSize::new(srv_w, win_h),
        )
        .map_err(|e| format!("Error al instanciar Webview hijo: {}", e))?;

    resize_layout(app)?;

    Ok(())
}

pub fn suspend_service(app: &AppHandle, service_id: &str) -> Result<(), String> {
    let label = service_label(service_id);
    if let Some(wv) = app.get_webview(&label) {
        let _ = wv.close();
    }

    let state = app.state::<ActiveServiceState>();
    {
        let mut active_id_guard = state.active_id.lock().unwrap();
        if active_id_guard.as_deref() == Some(service_id) {
            *active_id_guard = None;
        }
    }
    state.background_since.lock().unwrap().remove(service_id);

    let _ = resize_layout(app);
    Ok(())
}

pub fn set_modal_overlay(app: &AppHandle, is_open: bool) -> Result<(), String> {
    let state = app.state::<ActiveServiceState>();
    *state.is_modal_open.lock().unwrap() = is_open;
    resize_layout(app)
}

/// Watchdog automático en segundo plano para hibernar servicios inactivos
pub fn start_auto_suspend_watchdog(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));
        loop {
            interval.tick().await;
            check_and_auto_suspend(&app_handle);
        }
    });
}

fn check_and_auto_suspend(app: &AppHandle) {
    let state = app.state::<ActiveServiceState>();
    let mut services = match load_services(app) {
        Ok(s) => s,
        Err(_) => return,
    };
    sync_service_runtime_states(app, &mut services);

    let mut services_to_suspend = Vec::new();

    {
        let bg_map = state.background_since.lock().unwrap();
        for service in &services {
            if service.state == ServiceState::Background {
                if let Some(minutes) = service.auto_suspend_minutes {
                    if minutes > 0 {
                        if let Some(since) = bg_map.get(&service.id) {
                            if since.elapsed().as_secs() >= (minutes as u64 * 60) {
                                services_to_suspend.push(service.id.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    if !services_to_suspend.is_empty() {
        for id in services_to_suspend {
            let _ = suspend_service(app, &id);
        }
        sync_service_runtime_states(app, &mut services);
        let _ = save_services(app, &services);
        let _ = app.emit_to("ui", "services_updated", &services);
    }
}
