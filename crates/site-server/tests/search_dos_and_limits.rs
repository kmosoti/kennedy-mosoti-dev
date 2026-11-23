//! Negative, header, zero-ETag, and concurrency/timeout DoS tests for `/wiki` & `/wiki/search`.

use axum::body::{Body, to_bytes};
use axum::http::{Method, Request, StatusCode, header};
use site_content::load_from_dir;
use site_server::{AppState, SearchLimitConfig, build_router, build_router_from_state};
use std::path::PathBuf;
use std::time::Duration;
use tower::Service;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn assert_security_headers(resp: &axum::response::Response, context: &str) {
    let h = resp.headers();
    let csp = h
        .get(header::CONTENT_SECURITY_POLICY)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    assert!(
        csp.contains("default-src 'self'"),
        "missing CSP on {context}"
    );
    assert_eq!(
        h.get(header::X_CONTENT_TYPE_OPTIONS)
            .and_then(|v| v.to_str().ok()),
        Some("nosniff"),
        "missing X-Content-Type-Options on {context}"
    );
    assert_eq!(
        h.get(header::REFERRER_POLICY).and_then(|v| v.to_str().ok()),
        Some("strict-origin-when-cross-origin"),
        "missing Referrer-Policy on {context}"
    );
    assert_eq!(
        h.get(header::X_FRAME_OPTIONS).and_then(|v| v.to_str().ok()),
        Some("DENY"),
        "missing X-Frame-Options on {context}"
    );
    assert!(
        h.get("permissions-policy").is_some(),
        "missing Permissions-Policy on {context}"
    );
}

#[tokio::test]
async fn query_bounds_reject_oversized_and_overterm_requests_with_400_and_security_headers() {
    let root = repo_root();
    let content = load_from_dir(&root.join("content")).expect("content loads");
    let mut app = build_router(content, root.join("public"), root.join("target/site/pkg"));

    // 1. Raw query > 256 bytes (`q=` + 255 bytes = 257 bytes)
    let over_raw = format!("q={}", "a".repeat(255));
    // 2. Raw query <= 256 bytes (215 bytes) whose decoded `q` is 129 bytes (> 128)
    let over_decoded = format!("q={}{}", "%41".repeat(43), "a".repeat(86));
    // 3. > 16 whitespace terms
    let over_ws_terms = format!(
        "q={}",
        (1..=17)
            .map(|i| format!("t{i}"))
            .collect::<Vec<_>>()
            .join("+")
    );
    // 4. > 16 punctuation-separated alphanumeric terms
    let over_punct_terms = format!(
        "q={}",
        (1..=17)
            .map(|i| format!("t{i}"))
            .collect::<Vec<_>>()
            .join("-")
    );
    // 5. Second `q` param exceeds limit even though first is short
    let multi_q_overflow = format!("q=moiric&q={}", "b".repeat(129));
    // 6. Invalid UTF-8 percent-encoded sequence
    let bad_utf8 = "q=%ff%fe".to_owned();

    for qs in [
        over_raw,
        over_decoded,
        over_ws_terms,
        over_punct_terms,
        multi_q_overflow,
        bad_utf8,
    ] {
        for base in ["/wiki", "/wiki/search"] {
            let uri = format!("{base}?{qs}");
            let resp = app
                .call(Request::builder().uri(&uri).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(
                resp.status(),
                StatusCode::BAD_REQUEST,
                "expected 400 for {uri}"
            );
            assert_security_headers(&resp, &uri);
            assert_eq!(
                resp.headers()
                    .get(header::CACHE_CONTROL)
                    .and_then(|v| v.to_str().ok()),
                Some("no-store")
            );
            assert!(
                resp.headers().get(header::ETAG).is_none(),
                "400 error on {uri} must not compute ETag"
            );
        }
    }

    // Exact boundary: 16 terms, <= 128 decoded bytes, <= 256 raw bytes -> 200 OK
    let valid_16 = (1..=16)
        .map(|i| format!("t{i}"))
        .collect::<Vec<_>>()
        .join("+");
    let valid_128 = "a".repeat(128);
    for qs in [format!("q={valid_16}"), format!("q={valid_128}")] {
        for base in ["/wiki", "/wiki/search"] {
            let uri = format!("{base}?{qs}");
            let resp = app
                .call(Request::builder().uri(&uri).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(resp.status(), StatusCode::OK, "boundary query {uri} failed");
            assert_security_headers(&resp, &uri);
            assert!(
                resp.headers().get(header::ETAG).is_none(),
                "dynamic no-cache search {uri} must skip BLAKE3 ETag"
            );
        }
    }
}

#[tokio::test]
async fn dynamic_search_reuses_index_skips_etag_and_handles_trigram_fallback() {
    let root = repo_root();
    let content = load_from_dir(&root.join("content")).expect("content loads");
    let mut app = build_router(content, root.join("public"), root.join("target/site/pkg"));

    // Static /wiki still returns startup-cached response WITH ETag.
    let static_wiki = app
        .call(Request::builder().uri("/wiki").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(static_wiki.status(), StatusCode::OK);
    assert!(static_wiki.headers().get(header::ETAG).is_some());

    // SSR /wiki?q=moir uses &state.wiki_search trigram fallback and skips BLAKE3 ETag.
    let ssr_search = app
        .call(
            Request::builder()
                .uri("/wiki?q=moir")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(ssr_search.status(), StatusCode::OK);
    assert!(ssr_search.headers().get(header::ETAG).is_none());
    let body = to_bytes(ssr_search.into_body(), 1024 * 1024).await.unwrap();
    let html = std::str::from_utf8(&body).unwrap();
    assert!(
        html.contains("moiric-architecture"),
        "SSR /wiki?q=moir must use startup WikiSearchIndex trigram fallback"
    );

    // JSON /wiki/search?q=moiric skips ETag and preserves security headers.
    let api_search = app
        .call(
            Request::builder()
                .uri("/wiki/search?q=moiric")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(api_search.status(), StatusCode::OK);
    assert!(api_search.headers().get(header::ETAG).is_none());
    assert_security_headers(&api_search, "/wiki/search?q=moiric");
}

#[tokio::test]
async fn concurrent_get_and_head_searches_and_saturation_load_shedding() {
    let root = repo_root();
    let content = load_from_dir(&root.join("content")).expect("content loads");
    let app = build_router(
        content.clone(),
        root.join("public"),
        root.join("target/site/pkg"),
    );

    let max_q = "a".repeat(128);
    let mut tasks = Vec::new();
    for i in 0..32 {
        let mut router = app.clone();
        let method = if i % 2 == 0 {
            Method::GET
        } else {
            Method::HEAD
        };
        let path = if i % 3 == 0 {
            format!("/wiki?q={max_q}")
        } else {
            format!("/wiki/search?q={max_q}")
        };
        tasks.push(tokio::spawn(async move {
            let req = Request::builder()
                .method(method)
                .uri(&path)
                .body(Body::empty())
                .unwrap();
            let resp = router.call(req).await.unwrap();
            assert!(
                resp.status() == StatusCode::OK || resp.status() == StatusCode::SERVICE_UNAVAILABLE,
                "unexpected status {}",
                resp.status()
            );
            assert_security_headers(&resp, &path);
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }

    // Health remains 200 OK after burst.
    let mut health_app = app.clone();
    let health = health_app
        .call(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(health.status(), StatusCode::OK);

    // Deterministic saturation test: hold all permits and verify 503 with security headers.
    let tight_limits = SearchLimitConfig {
        max_concurrent: 2,
        queue_timeout: Duration::ZERO,
        exec_timeout: Duration::from_millis(250),
    };
    let state = AppState::from_pkg_dir_with_limits(
        content.clone(),
        &root.join("target/site/pkg"),
        tight_limits,
    );
    let mut tight_app = build_router_from_state(
        state.clone(),
        root.join("public"),
        root.join("target/site/pkg"),
    );

    let p1 = state
        .search_semaphore
        .clone()
        .acquire_owned()
        .await
        .unwrap();
    let p2 = state
        .search_semaphore
        .clone()
        .acquire_owned()
        .await
        .unwrap();

    for uri in ["/wiki?q=moiric", "/wiki/search?q=moiric"] {
        let resp = tight_app
            .call(Request::builder().uri(uri).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(
            resp.headers()
                .get(header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok()),
            Some("1")
        );
        assert_security_headers(&resp, uri);
    }

    drop((p1, p2));
    let recovered = tight_app
        .call(
            Request::builder()
                .uri("/wiki/search?q=moiric")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(recovered.status(), StatusCode::OK);

    // Deterministic execution timeout test: 0ms exec_timeout yields 504 with security headers.
    let timeout_limits = SearchLimitConfig {
        max_concurrent: 2,
        queue_timeout: Duration::from_millis(50),
        exec_timeout: Duration::ZERO,
    };
    let timeout_state =
        AppState::from_pkg_dir_with_limits(content, &root.join("target/site/pkg"), timeout_limits);
    let mut timeout_app = build_router_from_state(
        timeout_state,
        root.join("public"),
        root.join("target/site/pkg"),
    );
    let timed_out = timeout_app
        .call(
            Request::builder()
                .uri("/wiki/search?q=moiric")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(timed_out.status(), StatusCode::GATEWAY_TIMEOUT);
    assert_security_headers(&timed_out, "timeout /wiki/search");
}
