#![forbid(unsafe_code)]
#![recursion_limit = "256"]
//! Hallmark × Leptos 0.8.23 Islands/SSR application crate for `kennedy.mosoti.dev`.

#[cfg(feature = "ssr")]
pub mod css;
pub mod dto;
#[cfg(feature = "ssr")]
pub mod router;
pub mod views;

/// Progressive WASM hydration entry point for `#[island]` cockpit components.
#[cfg(all(target_arch = "wasm32", feature = "hydrate"))]
#[wasm_bindgen::prelude::wasm_bindgen(start)]
pub fn hydrate_site_islands() {
    leptos::mount::hydrate_islands();
}
