use tauri::{AppHandle, Emitter, Manager};
use uuid::Uuid;

use crate::managers::profile_storage::{delete_profile_data, load_services, save_services};
use crate::managers::webview_manager::{
    activate_service as wm_activate, get_services_synced, service_label, set_modal_overlay,
    suspend_service as wm_suspend, sync_service_runtime_states, ActiveServiceState,
};
use crate::models::service::{CreateServiceDto, ServiceInstance, ServiceState, ServiceTemplate};

fn default_url_for_template(template: &ServiceTemplate) -> &'static str {
    match template {
        ServiceTemplate::WhatsApp => "https://web.whatsapp.com",
        ServiceTemplate::Telegram => "https://web.telegram.org/a/",
        ServiceTemplate::Discord => "https://discord.com/app",
        ServiceTemplate::Slack => "https://app.slack.com/client",
        ServiceTemplate::Messenger => "https://www.messenger.com",
        ServiceTemplate::Instagram => "https://www.instagram.com",
        ServiceTemplate::Twitter => "https://x.com",
        ServiceTemplate::Gmail => "https://mail.google.com",
        ServiceTemplate::CustomUrl => "https://google.com",
    }
}

fn default_icon_for_template(template: &ServiceTemplate) -> &'static str {
    match template {
        ServiceTemplate::WhatsApp => "whatsapp",
        ServiceTemplate::Telegram => "telegram",
        ServiceTemplate::Discord => "discord",
        ServiceTemplate::Slack => "slack",
        ServiceTemplate::Messenger => "messenger",
        ServiceTemplate::Instagram => "instagram",
        ServiceTemplate::Twitter => "twitter",
        ServiceTemplate::Gmail => "gmail",
        ServiceTemplate::CustomUrl => "globe",
    }
}

#[tauri::command]
pub fn get_services(app: AppHandle) -> Result<Vec<ServiceInstance>, String> {
    get_services_synced(&app)
}

#[tauri::command]
pub fn create_service(
    app: AppHandle,
    input: CreateServiceDto,
) -> Result<ServiceInstance, String> {
    let mut services = load_services(&app)?;
    let id = Uuid::new_v4().to_string();

    let raw_url = input
        .url
        .filter(|u| !u.trim().is_empty())
        .unwrap_or_else(|| default_url_for_template(&input.template).to_string());

    // Validación de seguridad de URL: solo permitir http o https
    let parsed_url = url::Url::parse(&raw_url)
        .map_err(|e| format!("URL no válida (debe incluir http:// o https://): {}", e))?;
    if parsed_url.scheme() != "https" && parsed_url.scheme() != "http" {
        return Err("Por seguridad, solo se permiten protocolos web http:// o https://".to_string());
    }
    let url = parsed_url.to_string();

    let icon = input
        .icon
        .filter(|i| !i.trim().is_empty())
        .unwrap_or_else(|| default_icon_for_template(&input.template).to_string());

    let order = services.len() as u32;

    // Respetar Some(0) si el usuario eligió "Mantener siempre en RAM".
    // Fallback a 20 minutos recomendados si es None.
    let auto_suspend = input.auto_suspend_minutes.or(Some(20));

    let new_service = ServiceInstance {
        id: id.clone(),
        name: input.name,
        template: input.template,
        url,
        icon,
        state: ServiceState::Suspended,
        auto_suspend_minutes: auto_suspend,
        unread_count: 0,
        order,
    };

    services.push(new_service.clone());
    save_services(&app, &services)?;

    let mut synced_services = services;
    sync_service_runtime_states(&app, &mut synced_services);
    let _ = app.emit_to("ui", "services_updated", &synced_services);
    Ok(new_service)
}

#[tauri::command]
pub fn activate_service(
    app: AppHandle,
    service_id: String,
) -> Result<Vec<ServiceInstance>, String> {
    let mut services = load_services(&app)?;

    let target = services
        .iter()
        .find(|s| s.id == service_id)
        .cloned()
        .ok_or_else(|| "Servicio no encontrado".to_string())?;

    // Activar en el WebviewManager
    wm_activate(&app, &target)?;

    // Sincronizar estados con el runtime (Active, Background, Suspended)
    sync_service_runtime_states(&app, &mut services);

    save_services(&app, &services)?;
    let _ = app.emit_to("ui", "services_updated", &services);

    Ok(services)
}

#[tauri::command]
pub fn suspend_service(
    app: AppHandle,
    service_id: String,
) -> Result<Vec<ServiceInstance>, String> {
    wm_suspend(&app, &service_id)?;

    let mut services = load_services(&app)?;
    sync_service_runtime_states(&app, &mut services);

    save_services(&app, &services)?;
    let _ = app.emit_to("ui", "services_updated", &services);

    Ok(services)
}

#[tauri::command]
pub fn delete_service(
    app: AppHandle,
    service_id: String,
) -> Result<Vec<ServiceInstance>, String> {
    // 1. Cerrar Webview y liberar memoria
    let _ = wm_suspend(&app, &service_id);

    // 2. Eliminar directorio de datos en disco
    delete_profile_data(&app, &service_id)?;

    // 3. Quitar del registro
    let mut services = load_services(&app)?;
    services.retain(|s| s.id != service_id);

    // Reasignar orden
    for (idx, s) in services.iter_mut().enumerate() {
        s.order = idx as u32;
    }

    sync_service_runtime_states(&app, &mut services);
    save_services(&app, &services)?;
    let _ = app.emit_to("ui", "services_updated", &services);

    Ok(services)
}

#[tauri::command]
pub fn reload_service(
    app: AppHandle,
    service_id: String,
    _clear_cache: bool,
) -> Result<(), String> {
    let label = service_label(&service_id);
    if let Some(wv) = app.get_webview(&label) {
        let _ = wv.eval("window.location.reload()");
    }
    Ok(())
}

#[tauri::command]
pub fn toggle_modal_overlay(app: AppHandle, is_open: bool) -> Result<(), String> {
    set_modal_overlay(&app, is_open)
}

#[tauri::command]
pub fn open_add_service_modal(app: AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("add_service_modal") {
        let _ = w.show();
        let _ = w.set_focus();
        return Ok(());
    }

    let builder = tauri::WebviewWindowBuilder::new(
        &app,
        "add_service_modal",
        tauri::WebviewUrl::App("index.html?view=add-service".into()),
    )
    .title("Añadir Servicio Web - Kite")
    .inner_size(520.0, 640.0)
    .resizable(false)
    .center();

    builder.build().map_err(|e| format!("Error al abrir ventana de añadir servicio: {}", e))?;
    Ok(())
}

#[tauri::command]
pub fn close_add_service_modal(app: AppHandle) -> Result<(), String> {
    if let Some(w) = app.get_webview_window("add_service_modal") {
        let _ = w.destroy();
    }
    Ok(())
}

#[tauri::command]
pub fn show_service_context_menu(app: AppHandle, service_id: String) -> Result<(), String> {
    use tauri::menu::ContextMenu;

    let window = app
        .get_window("main")
        .ok_or_else(|| "Ventana principal no encontrada".to_string())?;

    let menu = tauri::menu::MenuBuilder::new(&app)
        .text(format!("reload:{}", service_id), "Recargar página")
        .text(format!("suspend:{}", service_id), "Hibernar (Ahorrar RAM)")
        .separator()
        .text(format!("delete:{}", service_id), "Eliminar servicio")
        .build()
        .map_err(|e| e.to_string())?;

    menu.popup(window).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn update_service_badge(app: AppHandle, count: u32) -> Result<(), String> {
    let state = app.state::<ActiveServiceState>();
    let active_id_guard = state.active_id.lock().unwrap();

    if let Some(ref active_id) = *active_id_guard {
        let mut services = load_services(&app)?;
        if let Some(s) = services.iter_mut().find(|s| s.id == *active_id) {
            if s.unread_count != count {
                s.unread_count = count;
                let _ = save_services(&app, &services);
                let _ = app.emit_to("ui", "services_updated", &services);
            }
        }
    }
    Ok(())
}
