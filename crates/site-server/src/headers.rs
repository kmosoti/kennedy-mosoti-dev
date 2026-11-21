//! Strict security and CSP response headers for `kennedy.mosoti.dev`.

use axum::http::{HeaderName, HeaderValue, header};
use tower_http::set_header::SetResponseHeaderLayer;

/// Content-Security-Policy permitting self-hosted styles, fonts, images, and WASM island scripts only.
pub const STRICT_CSP: &str = "default-src 'self'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; font-src 'self'; img-src 'self' data:; connect-src 'self'; base-uri 'self'; form-action 'self'; frame-ancestors 'none'; object-src 'none'";

/// Helper creating an overriding response header layer.
pub fn always_header(name: HeaderName, value: &'static str) -> SetResponseHeaderLayer<HeaderValue> {
    SetResponseHeaderLayer::overriding(name, HeaderValue::from_static(value))
}

/// Content-Security-Policy header layer.
pub fn csp_layer() -> SetResponseHeaderLayer<HeaderValue> {
    always_header(header::CONTENT_SECURITY_POLICY, STRICT_CSP)
}

/// X-Content-Type-Options: nosniff layer.
pub fn nosniff_layer() -> SetResponseHeaderLayer<HeaderValue> {
    always_header(header::X_CONTENT_TYPE_OPTIONS, "nosniff")
}

/// Referrer-Policy layer.
pub fn referrer_layer() -> SetResponseHeaderLayer<HeaderValue> {
    always_header(header::REFERRER_POLICY, "strict-origin-when-cross-origin")
}

/// X-Frame-Options: DENY layer.
pub fn frame_deny_layer() -> SetResponseHeaderLayer<HeaderValue> {
    always_header(header::X_FRAME_OPTIONS, "DENY")
}

/// Permissions-Policy layer.
pub fn permissions_layer() -> SetResponseHeaderLayer<HeaderValue> {
    always_header(
        HeaderName::from_static("permissions-policy"),
        "geolocation=(), microphone=(), camera=(), payment=(), usb=()",
    )
}

/// Cache-Control helper layer.
pub fn cache_layer(value: &'static str) -> SetResponseHeaderLayer<HeaderValue> {
    always_header(header::CACHE_CONTROL, value)
}
