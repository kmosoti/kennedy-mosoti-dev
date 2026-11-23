#![forbid(unsafe_code)]
//! Axum + Tower SSR/Islands HTTP server library for `kennedy.mosoti.dev`.

pub mod cache;
pub mod headers;
pub mod search_routes;
pub mod server;

pub use search_routes::SearchLimitConfig;
pub use server::{AppState, build_router, build_router_from_state, build_router_with_limits};
