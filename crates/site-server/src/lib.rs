#![forbid(unsafe_code)]
//! Axum + Tower SSR/Islands HTTP server library for `kennedy.mosoti.dev`.

pub mod cache;
pub mod headers;
pub mod server;

pub use server::{AppState, build_router};
