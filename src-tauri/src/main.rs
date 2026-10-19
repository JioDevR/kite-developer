// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    #[cfg(target_os = "linux")]
    {
        // En Linux Wayland con drivers NVIDIA, deshabilitar explicit sync en el driver
        // previene el error de protocolo wp_linux_drm_syncobj_surface_v1 en GTK3 / WebKitGTK
        // permitiendo correr en Wayland nativo sin problemas.
        unsafe {
            if std::env::var("__NV_DISABLE_EXPLICIT_SYNC").is_err() {
                std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
            }
        }
    }

    tauri_app_lib::run()
}
