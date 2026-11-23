//! Bounded concurrency, timeout, and non-ETag handlers for dynamic wiki search routes.

use crate::cache::{dynamic_error_response, dynamic_no_cache_response};
use crate::server::AppState;
use axum::extract::State;
use axum::http::{StatusCode, Uri};
use axum::response::Response;
use site_app::router::{parse_bounded_query_param, render_wiki_search_page};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::OwnedSemaphorePermit;

/// Default maximum in-flight dynamic search computations across `/wiki?q=` and `/wiki/search`.
pub const DEFAULT_MAX_CONCURRENT_SEARCHES: usize = 16;
/// Default queue wait budget when all dynamic search permits are busy.
pub const DEFAULT_SEARCH_QUEUE_TIMEOUT: Duration = Duration::from_millis(50);
/// Default execution timeout for a single dynamic search/render call.
pub const DEFAULT_SEARCH_EXEC_TIMEOUT: Duration = Duration::from_millis(250);

/// Concurrency and timeout budgets applied to dynamic search endpoints.
#[derive(Debug, Clone, Copy)]
pub struct SearchLimitConfig {
    pub max_concurrent: usize,
    pub queue_timeout: Duration,
    pub exec_timeout: Duration,
}

impl Default for SearchLimitConfig {
    fn default() -> Self {
        Self {
            max_concurrent: DEFAULT_MAX_CONCURRENT_SEARCHES,
            queue_timeout: DEFAULT_SEARCH_QUEUE_TIMEOUT,
            exec_timeout: DEFAULT_SEARCH_EXEC_TIMEOUT,
        }
    }
}

async fn acquire_search_permit(state: &AppState) -> Option<OwnedSemaphorePermit> {
    match state.search_semaphore.clone().try_acquire_owned() {
        Ok(permit) => Some(permit),
        Err(_) if state.search_limits.queue_timeout.is_zero() => None,
        Err(_) => tokio::time::timeout(
            state.search_limits.queue_timeout,
            state.search_semaphore.clone().acquire_owned(),
        )
        .await
        .ok()?
        .ok(),
    }
}

/// Handles `GET`/`HEAD` `/wiki/search?q=...` with bounded query validation,
/// concurrency gating, execution timeout, and zero BLAKE3 `ETag` hashing.
pub async fn serve_wiki_search_api(State(state): State<AppState>, uri: Uri) -> Response {
    let q = match uri.query() {
        Some(qs) => match parse_bounded_query_param(qs, "q") {
            Ok(opt) => opt.unwrap_or_default(),
            Err(_) => {
                return dynamic_error_response(
                    StatusCode::BAD_REQUEST,
                    r#"{"error":"invalid search query"}"#,
                    "application/json; charset=utf-8",
                    None,
                );
            }
        },
        None => String::new(),
    };

    if q.is_empty() {
        return dynamic_no_cache_response(
            StatusCode::OK,
            r#"{"query":"","hits":[]}"#,
            "application/json; charset=utf-8",
        );
    }

    let Some(permit) = acquire_search_permit(&state).await else {
        return dynamic_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            r#"{"error":"search capacity saturated"}"#,
            "application/json; charset=utf-8",
            Some("1"),
        );
    };
    if state.search_limits.exec_timeout.is_zero() {
        return dynamic_error_response(
            StatusCode::GATEWAY_TIMEOUT,
            r#"{"error":"search timed out"}"#,
            "application/json; charset=utf-8",
            Some("1"),
        );
    }

    let wiki_search = Arc::clone(&state.wiki_search);
    let handle = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        let hits = wiki_search.search(&q, 20);
        serde_json::json!({ "query": q, "hits": hits }).to_string()
    });

    match tokio::time::timeout(state.search_limits.exec_timeout, handle).await {
        Ok(Ok(body)) => {
            dynamic_no_cache_response(StatusCode::OK, body, "application/json; charset=utf-8")
        }
        Ok(Err(_)) => dynamic_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            r#"{"error":"search worker failed"}"#,
            "application/json; charset=utf-8",
            None,
        ),
        Err(_) => dynamic_error_response(
            StatusCode::GATEWAY_TIMEOUT,
            r#"{"error":"search timed out"}"#,
            "application/json; charset=utf-8",
            Some("1"),
        ),
    }
}

/// Handles `GET`/`HEAD` `/wiki?q=...` SSR searches using `&state.wiki_search` (zero index rebuild).
/// Returns `None` when no non-empty `q` parameter is present so static `/wiki` serves from cache.
pub async fn serve_wiki_ssr_search(state: &AppState, qs: &str) -> Option<Response> {
    let q = match parse_bounded_query_param(qs, "q") {
        Ok(Some(q)) => q,
        Ok(None) => return None,
        Err(_) => {
            return Some(dynamic_error_response(
                StatusCode::BAD_REQUEST,
                "400 Bad Request: invalid wiki search query",
                "text/html; charset=utf-8",
                None,
            ));
        }
    };

    let Some(permit) = acquire_search_permit(state).await else {
        return Some(dynamic_error_response(
            StatusCode::SERVICE_UNAVAILABLE,
            "503 Service Unavailable: search capacity saturated",
            "text/html; charset=utf-8",
            Some("1"),
        ));
    };
    if state.search_limits.exec_timeout.is_zero() {
        return Some(dynamic_error_response(
            StatusCode::GATEWAY_TIMEOUT,
            "504 Gateway Timeout: search timed out",
            "text/html; charset=utf-8",
            Some("1"),
        ));
    }

    let wiki_search = Arc::clone(&state.wiki_search);
    let assets = Arc::clone(&state.assets);
    let positioning = state.content.identity.positioning.clone();
    let handle = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        render_wiki_search_page(&positioning, &wiki_search, Some(&q), &assets)
    });

    let resp = match tokio::time::timeout(state.search_limits.exec_timeout, handle).await {
        Ok(Ok(page)) => {
            dynamic_no_cache_response(StatusCode::OK, page.html, "text/html; charset=utf-8")
        }
        Ok(Err(_)) => dynamic_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "500 Internal Server Error: search worker failed",
            "text/html; charset=utf-8",
            None,
        ),
        Err(_) => dynamic_error_response(
            StatusCode::GATEWAY_TIMEOUT,
            "504 Gateway Timeout: search timed out",
            "text/html; charset=utf-8",
            Some("1"),
        ),
    };
    Some(resp)
}
