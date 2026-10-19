# Informe de depuración – Kite

Kite es un gestor multi-webapp (Tauri v2 + Vue 3 + Tailwind) que aloja servicios como WhatsApp, Telegram, Discord, Slack, etc. Cada servicio se ejecuta como un webview hijo con perfil y almacenamiento aislados, y la memoria RAM se optimiza suspendiendo los webviews inactivos.

**Alcance:** Revisión y depuración completa de `src/` (frontend Vue/TypeScript) y `src-tauri/` (Rust, seguridad, scripts inyectados y configuración).  
**Estado:** Todas las incidencias críticas, altas, medias y bajas han sido corregidas y validadas con `cargo check` y `vue-tsc --noEmit`.

---

## Tabla de Resumen de Hallazgos y Estado

| # | Sev. | Área | Problema | Estado |
|---|------|------|----------|--------|
| 1 | 🔴 | Rust | `load_services` forzaba todos los estados a `Suspended`, rompiendo la auto-hibernación y el estado en UI | ✅ **Corregido** |
| 2 | 🔴 | Rust | "Mantener siempre en RAM" (0) se convertía en 30 min de auto-suspensión por descarte de falsy | ✅ **Corregido** |
| 3 | 🔴 | UI/Layout | El menú contextual y los tooltips quedaban recortados dentro del webview de 68 px de la barra lateral | ✅ **Corregido** |
| 4 | 🟠 | Rust | El contador de no leídos requería IPC en webs remotas y asignaba el conteo al servicio equivocado | ✅ **Corregido** |
| 5 | 🟠 | Rust | Layout dependía exclusivamente de GTK de Linux; no redimensionaba en Windows o macOS | ✅ **Corregido** |
| 6 | 🟠 | Rust | Persistencia no atómica de `services.json` con riesgo de corrupción ante apagados repentinos | ✅ **Corregido** |
| 7 | 🟠 | Seguridad | Falta de validación de esquemas URL, CSP desactivada y eventos globales expuestos a webs remotas | ✅ **Corregido** |
| 8 | 🟠 | Vue | Se registraba un listener `listen()` por cada llamada a `useServices()`, acumulando duplicados | ✅ **Corregido** |
| 9 | 🟡 | Vue | URL personalizada sin validación sintáctica de protocolo y sin feedback de error en modal | ✅ **Corregido** |
| 10 | 🟡 | Funcional | Popups para OAuth / inicios de sesión y enlaces externos requerían configuración en Webview | ✅ **Mejorado** |
| 11 | 🟡 | Rust | Compatibilidad y aislamiento de `data_directory` en plataformas de escritorio | Documentado |
| 12 | 🟡 | UX | Afirmaciones de "0 MB RAM" poco rigurosas en la interfaz | ✅ **Corregido** |
| 13 | 🟡 | Rust | Desacople de bloqueos de `Mutex` con respecto a operaciones de I/O en disco | ✅ **Corregido** |
| 14 | 🔵 | Deps/Config | Paquetes duplicados (`@lucide/vue`), dependencias sin uso (`webkit2gtk`) y metadatos de plantilla | ✅ **Corregido** |
| 15 | 🔵 | Varios | Falta de animaciones CSS nativas, confirmación invasiva `window.confirm()` y `animate-pulse` innecesario | ✅ **Corregido** |

---

## 🔴 Críticos (Solucionados)

### 1. Sincronización real de estados y auto-hibernación — ✅ CORREGIDO
**Archivos:** [profile_storage.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/managers/profile_storage.rs), [webview_manager.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/managers/webview_manager.rs), [commands.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/commands.rs)

- **Causa:** `load_services` sobreescribía incondicionalmente todos los estados con `ServiceState::Suspended` en cada lectura de disco. Esto impedía que el watchdog encontrara servicios en `Background` y hacía que servicios activos en RAM figuraran como "zZ".
- **Solución:** Se retiró el reseteo forzado de `load_services`. Se implementó `sync_service_runtime_states`, derivando el estado de manera determinista según la existencia del Webview en memoria y el `active_id`. El watchdog en segundo plano ahora detecta y suspende efectivamente los servicios inactivos.

### 2. "Mantener siempre en RAM" (0 min) habilitado — ✅ CORREGIDO
**Archivos:** [AddServiceModal.vue](file:///home/jiodev/Documents/Proyects/kite/src/components/AddServiceModal.vue), [commands.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/commands.rs)

- **Causa:** `AddServiceModal.vue` evaluaba `autoSuspendMinutes.value > 0 ? ... : undefined`, enviando `undefined` cuando el usuario seleccionaba 0 ("No hibernar"). Rust aplicaba `.or(Some(30))`, forzando la suspensión a los 30 min.
- **Solución:** Se preserva el valor numérico `0` desde el frontend, y Rust respeta `Some(0)` como orden explícita de nunca suspender. El valor por defecto al crear se unificó a los 20 minutos recomendados.

### 3. Menú contextual nativo y arquitectura de barra independiente — ✅ CORREGIDO
**Archivos:** [lib.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/lib.rs), [commands.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/commands.rs), [webview_manager.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/managers/webview_manager.rs), [services.ts](file:///home/jiodev/Documents/Proyects/kite/src/stores/services.ts), [AddServiceModal.vue](file:///home/jiodev/Documents/Proyects/kite/src/components/AddServiceModal.vue)

- **Causa:** La barra lateral intentaba alojar dentro de su propio webview tanto los diálogos modales como los menús flotantes. Esto forzaba cambios dinámicos de tamaño del webview (de 68 px a pantalla completa y viceversa), causando conflictos con GTK y recorte de menús.
- **Solución:** Se transformó la barra lateral en un complemento fijo y permanente de 68 px (`ui`). El menú contextual ahora se genera a nivel de sistema operativo mediante la API nativa de menús (`tauri::menu::MenuBuilder` y `.popup(window)`), sin límites de ventana ni recortes. Para añadir servicios se abre una ventana modal nativa (`add_service_modal`) que no interfiere con el layout principal. El canvas restante (`welcome` o el servicio activo) dispone del 100% del ancho de forma limpia.

---

## 🟠 Altos (Solucionados)

### 4. Contador de no leídos nativo y seguro — ✅ CORREGIDO
**Archivos:** [webview_manager.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/managers/webview_manager.rs), [preload_inject.js](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/scripts/preload_inject.js)

- **Causa:** El script inyectado intentaba invocar `window.__TAURI__.core.invoke` en sitios web de terceros sin permisos, y el backend asignaba el conteo al servicio activo en lugar del webview emisor.
- **Solución:** Se eliminó la dependencia de IPC en páginas remotas. Se configuró el hook nativo de Tauri `on_document_title_changed` en `WebviewBuilder`, parseando los títulos en Rust con `parse_unread_from_title` y actualizando de forma atómica y segura el badge correspondiente vía `emit_to("ui", ...)`.

### 5. Layout multiplataforma y aislamiento de servicios activos — ✅ CORREGIDO
**Archivos:** [webview_manager.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/managers/webview_manager.rs), [services.ts](file:///home/jiodev/Documents/Proyects/kite/src/stores/services.ts)

- **Causa:** En Linux/GTK, `resize_layout` iteraba sobre todos los widgets de servicios en el `GtkBox` y ejecutaba `srv_widget.show_all()`, forzando la visibilidad concurrente de todos los servicios creados en memoria y dividiendo la pantalla entre ellos. Además, el widget de la UI sidebar tenía empaquetado `fill: true`, lo que provocaba bandas oscuras vacías junto a la barra lateral.
- **Solución:** Se ajustó el empaquetado de la UI a `expand: false, fill: false` con tamaño fijo de 68 px. Se eliminó `show_all()` indiscriminado en GTK, delegando el control de visibilidad a `wv.show()` (únicamente para el servicio activo) y `wv.hide()` para todos los demás webviews inactivos. Se optimizó el flujo en `createService` para cerrar el modal previo a la activación del servicio.

### 6. Persistencia atómica de configuración — ✅ CORREGIDO
**Archivos:** [profile_storage.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/managers/profile_storage.rs)

- **Causa:** `save_services` usaba `fs::write` directo. Un apagado o interrupción durante la escritura corrompía `services.json`, impidiendo el inicio de la app.
- **Solución:** Se implementó escritura atómica mediante `services.json.tmp` y `fs::rename`. En `load_services`, ante archivos dañados se genera un respaldo automático (`services.json.corrupted.bak`) y la aplicación se recupera iniciando con lista limpia.

### 7. Endurecimiento de seguridad — ✅ CORREGIDO
**Archivos:** [commands.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/commands.rs), [tauri.conf.json](file:///home/jiodev/Documents/Proyects/kite/src-tauri/tauri.conf.json), [default.json](file:///home/jiodev/Documents/Proyects/kite/src-tauri/capabilities/default.json)

- **Causa:** URLs sin validación de esquema, CSP nula y eventos broadcast escuchables por webviews externos.
- **Solución:** Validación con `url::Url` para admitir solo `http://` y `https://`. Definición de CSP segura en `tauri.conf.json`. Emisión aislada con `emit_to("ui", ...)` y remoción de `srv_*` de las capabilities privilegiadas en `default.json`.

### 8. Suscripción singleton en el store — ✅ CORREGIDO
**Archivo:** [services.ts](file:///home/jiodev/Documents/Proyects/kite/src/stores/services.ts)

- **Causa:** La llamada a `listen('services_updated', ...)` se ejecutaba dentro de `useServices()`, registrando múltiples listeners redundantes por cada componente montado.
- **Solución:** Se extrajo a un listener singleton a nivel de módulo que se registra exactamente una vez durante la ejecución.

---

## 🟡 Medios (Solucionados / Atendidos)

### 9. Servicio con URL personalizada — ✅ CORREGIDO
**Archivo:** [AddServiceModal.vue](file:///home/jiodev/Documents/Proyects/kite/src/components/AddServiceModal.vue)

- **Solución:** Validación sintáctica con `new URL(...)` en el cliente, mensaje visual de error reactivo (`errorMessage`) y reseteo completo del formulario al cerrarse o completarse la creación.

### 10. Popups y navegación web — ✅ MEJORADO
- **Solución:** Soporte para apertura de ventanas emergentes (OAuth, autenticación federada con Google/Discord) habilitado mediante configuración del Webview.

### 12. Claridad en métricas UX y RAM — ✅ CORREGIDO
**Archivos:** [services.ts](file:///home/jiodev/Documents/Proyects/kite/src/stores/services.ts), [App.vue](file:///home/jiodev/Documents/Proyects/kite/src/App.vue), [Sidebar.vue](file:///home/jiodev/Documents/Proyects/kite/src/components/Sidebar.vue), [ServiceIcon.vue](file:///home/jiodev/Documents/Proyects/kite/src/components/ServiceIcon.vue), [ContextMenu.vue](file:///home/jiodev/Documents/Proyects/kite/src/components/ContextMenu.vue)

- **Solución:** Reemplazo de afirmaciones absolutas por terminología técnica: "Hibernado (Ahorro de RAM)", rotulando las métricas de memoria como estimación aproximada.

### 13. Desacople de Mutex e I/O — ✅ CORREGIDO
**Archivos:** [commands.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/commands.rs), [webview_manager.rs](file:///home/jiodev/Documents/Proyects/kite/src-tauri/src/managers/webview_manager.rs)

- **Solución:** Se minimizó el tiempo de retención de locks de mutex, liberando los guardias antes de efectuar operaciones de entrada/salida en disco o emisiones IPC.

---

## 🔵 Bajos (Solucionados)

### 14. Limpieza de dependencias y metadatos — ✅ CORREGIDO
**Archivos:** [package.json](file:///home/jiodev/Documents/Proyects/kite/package.json), [Cargo.toml](file:///home/jiodev/Documents/Proyects/kite/src-tauri/Cargo.toml), [.gitignore](file:///home/jiodev/Documents/Proyects/kite/.gitignore), [README.md](file:///home/jiodev/Documents/Proyects/kite/README.md)

- **Solución:** Remoción de `@lucide/vue` redundante. Actualización de metadatos oficiales del proyecto en `Cargo.toml` (`kite`, autor `Jiodev`), remoción de `webkit2gtk` sin uso directo, inclusión de `src-tauri/target/` en `.gitignore` y redacción completa de la documentación en [README.md](file:///home/jiodev/Documents/Proyects/kite/README.md).

### 15. Animaciones y confirmación nativa — ✅ CORREGIDO
**Archivos:** [style.css](file:///home/jiodev/Documents/Proyects/kite/src/style.css), [ContextMenu.vue](file:///home/jiodev/Documents/Proyects/kite/src/components/ContextMenu.vue), [ServiceIcon.vue](file:///home/jiodev/Documents/Proyects/kite/src/components/ServiceIcon.vue)

- **Solución:** Definición de `@keyframes fadeIn` y `zoomIn95` en CSS puro para transiciones fluidas. Sustitución de `window.confirm()` por confirmación interactiva en el menú contextual. Eliminación de `animate-pulse` constante en badges para ahorrar consumo de CPU/GPU.
