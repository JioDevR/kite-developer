use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ServiceTemplate {
    WhatsApp,
    Telegram,
    Discord,
    Slack,
    Messenger,
    Instagram,
    Twitter,
    Gmail,
    CustomUrl,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ServiceState {
    Active,     // Visible actualmente en pantalla
    Background, // Cargado en RAM pero oculto (.hide())
    Suspended,  // Proceso destruido (0 MB RAM), sesión guardada en disco
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInstance {
    pub id: String,                        // UUID v4 único
    pub name: String,                      // Ej: "WhatsApp Personal"
    pub template: ServiceTemplate,         // Tipo de servicio
    pub url: String,                       // URL de destino
    pub icon: String,                      // Nombre o clase de icono
    pub state: ServiceState,               // Estado actual
    pub auto_suspend_minutes: Option<u32>, // Minutos de inactividad para suspender
    pub unread_count: u32,                 // Mensajes no leídos
    pub order: u32,                        // Posición en la barra lateral
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateServiceDto {
    pub name: String,
    pub template: ServiceTemplate,
    pub url: Option<String>,
    pub icon: Option<String>,
    pub auto_suspend_minutes: Option<u32>,
}
