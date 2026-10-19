use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use crate::models::service::ServiceInstance;

const SERVICES_FILENAME: &str = "services.json";

/// Obtiene el directorio base de datos de la aplicación
pub fn get_app_data_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    Ok(path)
}

/// Obtiene la ruta del archivo de configuración `services.json`
pub fn get_services_json_path(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(get_app_data_dir(app)?.join(SERVICES_FILENAME))
}

/// Obtiene la carpeta exclusiva y aislada de un servicio (cookies, IndexedDB, etc.)
pub fn get_profile_dir(app: &AppHandle, service_id: &str) -> Result<PathBuf, String> {
    let profiles_dir = get_app_data_dir(app)?.join("profiles").join(service_id);
    if !profiles_dir.exists() {
        fs::create_dir_all(&profiles_dir).map_err(|e| e.to_string())?;
    }
    Ok(profiles_dir)
}

/// Carga la lista de servicios guardados desde disco con tolerancia a corrupción
pub fn load_services(app: &AppHandle) -> Result<Vec<ServiceInstance>, String> {
    let path = get_services_json_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    match serde_json::from_str::<Vec<ServiceInstance>>(&content) {
        Ok(services) => Ok(services),
        Err(e) => {
            eprintln!("[Kite] Advertencia: services.json inválido: {}. Creando backup de seguridad.", e);
            let backup_path = path.with_extension("corrupted.bak");
            let _ = fs::copy(&path, &backup_path);
            Ok(Vec::new())
        }
    }
}

/// Guarda la lista completa de servicios en disco de forma atómica (tmp -> rename)
pub fn save_services(app: &AppHandle, services: &[ServiceInstance]) -> Result<(), String> {
    let path = get_services_json_path(app)?;
    let tmp_path = path.with_extension("tmp");
    let serialized = serde_json::to_string_pretty(services).map_err(|e| e.to_string())?;

    fs::write(&tmp_path, serialized).map_err(|e| format!("Error al escribir archivo temporal: {}", e))?;
    fs::rename(&tmp_path, &path).map_err(|e| format!("Error en reemplazo atómico de archivo: {}", e))?;
    Ok(())
}

/// Elimina los datos del perfil en disco (cookies, caché, etc.)
pub fn delete_profile_data(app: &AppHandle, service_id: &str) -> Result<(), String> {
    let profile_path = get_profile_dir(app, service_id)?;
    if profile_path.exists() {
        fs::remove_dir_all(&profile_path).map_err(|e| e.to_string())?;
    }
    Ok(())
}
