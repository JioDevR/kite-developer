# 🪁 Kite — Gestor Multi-WebApp Ultraligero

[![Tauri v2](https://img.shields.io/badge/Tauri-v2-24C8D8?style=flat-square&logo=tauri&logoColor=white)](https://v2.tauri.app/)
[![Vue 3](https://img.shields.io/badge/Vue-3.5-4FC08D?style=flat-square&logo=vuedotjs&logoColor=white)](https://vuejs.org/)
[![Rust](https://img.shields.io/badge/Rust-2024-DEA584?style=flat-square&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Vite](https://img.shields.io/badge/Vite-8.0-646CFF?style=flat-square&logo=vite&logoColor=white)](https://vitejs.dev/)
[![Tailwind CSS](https://img.shields.io/badge/Tailwind-3.4-38B2AC?style=flat-square&logo=tailwind-css&logoColor=white)](https://tailwindcss.com/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square)](LICENSE)

**Kite** es una aplicación de escritorio nativa de alto rendimiento desarrollada con **Tauri v2**, **Vue 3** y **Rust**. Diseñada para sustituir aplicaciones pesadas basadas en Electron (como Ferdi, Franz o Rambox), Kite unifica servicios web y aplicaciones de mensajería (WhatsApp, Telegram, Discord, Slack, Messenger, Gmail, X, etc.) consumiendo una fracción de los recursos y garantizando aislamiento absoluto de sesiones.

---

## ✨ Características Principales

- **🛡️ Aislamiento Multi-Cuenta Absoluto:** Cada servicio se ejecuta con un directorio de perfil independiente (`data_directory` aislado). Puedes tener múltiples cuentas de un mismo servicio (ej. dos cuentas de WhatsApp o Discord) simultáneamente sin conflicto de cookies ni sesiones.
- **⚡ Suspensión Inteligente de Recursos:** Watchdog en segundo plano que hiberna automáticamente los servicios inactivos según el tiempo configurado (10, 20 o 45 min, o mantener siempre en RAM), liberando memoria de forma drástica conservando la sesión.
- **🧭 Arquitectura Companion (Barra Lateral Independiente):** La barra lateral es un complemento fijo de 68 px. El resto de la ventana pertenece íntegramente al canvas de visualización nativo (el servicio activo o la pantalla de bienvenida).
- **🪟 Ventanas y Menús Nativos del Sistema:** 
  - Diálogo flotante modal nativo para añadir servicios sin alterar el layout principal.
  - Menú contextual nativo del sistema operativo (clic derecho en iconos para recargar, suspender o eliminar).
- **🔔 Conteo de No Leídos Nativo:** Detección de badges analizando el título de la ventana directamente en Rust (`on_document_title_changed`), sin inyección de IPC ni brechas de seguridad en webs remotas.
- **🔒 Seguridad Endurecida:** Content Security Policy (CSP) restrictiva, validación estricta de esquemas URL (`https://` y `http://`), persistencia atómica a prueba de fallos de energía y aislamiento de capabilities internas.

---

## 🛠️ Tecnologías

- **Núcleo:** [Tauri v2](https://v2.tauri.app/) (Rust 2024 edition)
- **Frontend:** [Vue 3](https://vuejs.org/) + [TypeScript](https://www.typescriptlang.org/) + [Vite](https://vitejs.dev/)
- **Estilos:** [Tailwind CSS](https://tailwindcss.com/) + CSS puro para micro-animaciones
- **Iconos:** [Lucide Icons](https://lucide.dev/) + SVGs optimizados
- **Plataformas Soportadas:** Linux (WebKitGTK), Windows (WebView2), macOS (WKWebView)

---

## 🚀 Instalación y Desarrollo

### Requisitos Previos

1. **Node.js** (v18 o superior) y `npm`
2. **Rust** (v1.85 o superior) con `cargo`
3. Dependencias de desarrollo de Tauri según tu sistema operativo:
   - **Linux (Debian/Ubuntu):**
     ```bash
     sudo apt update && sudo apt install -y libwebkit2gtk-4.1-dev build-essential curl wget file libssl-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev
     ```
   - **Linux (Arch/Manjaro):**
     ```bash
     sudo pacman -S webkit2gtk-4.1 base-devel openssl gtk3
     ```
   - **Windows:** Microsoft Visual Studio C++ Build Tools & WebView2.
   - **macOS:** Xcode Command Line Tools.

### Ejecución en Desarrollo

```bash
# 1. Clonar el repositorio
git clone https://github.com/tu-usuario/kite.git
cd kite

# 2. Instalar dependencias del frontend
npm install

# 3. Lanzar aplicación en modo desarrollo
npm run tauri dev
```

### Compilar para Producción

```bash
npm run tauri build
```
Los binarios e instaladores generados (.deb, .AppImage, .msi, .dmg) estarán en `src-tauri/target/release/bundle/`.

---

## 📁 Estructura del Proyecto

```text
kite/
├── src/                      # Frontend Vue 3 + TypeScript
│   ├── assets/               # Metadatos e iconos de servicios
│   ├── components/           # Componentes UI (Sidebar, WelcomeView, AddServiceModal, ServiceIcon)
│   ├── stores/               # Estado reactivo y bindings con Tauri IPC
│   ├── types/                # Definiciones de TypeScript
│   ├── App.vue               # Enrutador dinámico modular
│   ├── main.ts               # Punto de entrada de la aplicación Vue
│   └── style.css             # Estilos globales y micro-animaciones
├── src-tauri/                # Backend Rust (Tauri v2)
│   ├── capabilities/         # Configuración de permisos y seguridad
│   ├── src/
│   │   ├── commands.rs       # Comandos Tauri expuestos al frontend
│   │   ├── lib.rs            # Configuración de ventanas, webviews y eventos nativos
│   │   ├── main.rs           # Binario principal
│   │   ├── managers/         # Gestores de Webviews y perfiles en disco
│   │   ├── models/           # Estructuras de datos (ServiceInstance, etc.)
│   │   └── scripts/          # Scripts de inicialización inyectados
│   ├── Cargo.toml            # Dependencias y configuración de Rust
│   └── tauri.conf.json       # Configuración global de Tauri
├── informe.md                # Bitácora detallada de auditoría y depuración técnica
└── package.json              # Scripts y dependencias npm
```

---

## 📄 Licencia

Este proyecto está bajo la Licencia MIT. Consulta el archivo [LICENSE](LICENSE) para más información.
