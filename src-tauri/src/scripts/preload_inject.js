// Kite Webview Preload Script
(function() {
    try {
        // Eliminar huella de automatización para evitar bloqueos por Cloudflare / WhatsApp Web
        Object.defineProperty(navigator, 'webdriver', {
            get: () => undefined,
            configurable: true
        });
    } catch (e) {
        console.error("Kite preload inject error:", e);
    }
})();
