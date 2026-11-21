#![forbid(unsafe_code)]
//! Axum + Tower HTTP server library for `kennedy.mosoti.dev`.

pub mod headers;
pub mod server;

pub use server::{AppState, build_router};
