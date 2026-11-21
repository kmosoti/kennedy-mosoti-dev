//! End-to-end Axum router tests, security header checks, RSS/Atom parsing,
//! and uncompressed + Brotli/Zstd + post-`wasm-opt -Oz` WASM payload budgets.

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use site_app::css::combined_stylesheet;
use site_app::router::render_route;
use site_content::load_from_dir;
use site_server::build_router;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use tower::Service;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn brotli_size(bytes: &[u8]) -> usize {
    let mut out = Vec::new();
    {
        let mut w = brotli::CompressorWriter::new(&mut out, 4096, 9, 22);
        w.write_all(bytes).expect("brotli write");
    }
    out.len()
}

fn zstd_size(bytes: &[u8]) -> usize {
    zstd::bulk::compress(bytes, 11)
        .expect("zstd compress")
        .len()
}

#[tokio::test]
async fn server_serves_health_feed_css_fonts_and_ssr_routes_with_csp() {
    let root = repo_root();
    let content = load_from_dir(&root.join("content")).expect("content loads");
    let mut app = build_router(content, root.join("public"), root.join("target/site/pkg"));

    for (path, expect_status, expect_ct, expect_substr) in [
        (
            "/health",
            StatusCode::OK,
            "application/json",
            "\"projects\":5",
        ),
        (
            "/feed.xml",
            StatusCode::OK,
            "application/atom+xml",
            "<feed xmlns=\"http://www.w3.org/2005/Atom\">",
        ),
        (
            "/rss.xml",
            StatusCode::OK,
            "application/rss+xml",
            "<rss version=\"2.0\">",
        ),
        ("/sitemap.xml", StatusCode::OK, "application/xml", "<urlset"),
        ("/pkg/site.css", StatusCode::OK, "text/css", "@layer reset"),
        (
            "/pkg/site_boot.js",
            StatusCode::OK,
            "application/javascript",
            "init(",
        ),
        (
            "/fonts/public-sans-var-latin.woff2",
            StatusCode::OK,
            "font/woff2",
            "",
        ),
        ("/robots.txt", StatusCode::OK, "text/plain", "Allow: /"),
        ("/", StatusCode::OK, "text/html", "Kennedy Mosoti"),
        ("/projects/nomos", StatusCode::OK, "text/html", "Nomos"),
        (
            "/wiki/moiric-architecture",
            StatusCode::OK,
            "text/html",
            "Moiric",
        ),
        (
            "/wiki?q=moiric",
            StatusCode::OK,
            "text/html",
            "moiric-architecture",
        ),
        (
            "/wiki/search.json",
            StatusCode::OK,
            "application/json",
            "moiric-architecture",
        ),
        (
            "/wiki/search?q=moiric",
            StatusCode::OK,
            "application/json",
            "\"hits\"",
        ),
        (
            "/pkg/wiki_search.js",
            StatusCode::OK,
            "application/javascript",
            "wiki-search-input",
        ),
        (
            "/blog/replacing-threejs-with-pure-rust-cockpit",
            StatusCode::OK,
            "text/html",
            "Pure-Rust",
        ),
        (
            "/biography",
            StatusCode::OK,
            "text/html",
            "Platform, Reliability &amp; Systems Automation Engineer",
        ),
        ("/does-not-exist", StatusCode::NOT_FOUND, "text/html", "404"),
    ] {
        let req = Request::builder()
            .uri(path)
            .body(Body::empty())
            .expect("valid request");
        let resp = app.call(req).await.expect("router response");
        assert_eq!(resp.status(), expect_status, "unexpected status for {path}");

        let csp = resp
            .headers()
            .get(header::CONTENT_SECURITY_POLICY)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(csp.contains("default-src 'self'"), "missing CSP on {path}");
        assert!(
            resp.headers().get(header::X_CONTENT_TYPE_OPTIONS).is_some(),
            "missing nosniff on {path}"
        );

        let ct = resp
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert!(
            ct.contains(expect_ct),
            "route {path} Content-Type `{ct}` did not contain `{expect_ct}`"
        );

        let body = to_bytes(resp.into_body(), 2 * 1024 * 1024)
            .await
            .expect("body bytes");
        if !expect_substr.is_empty() {
            let text = String::from_utf8_lossy(&body);
            assert!(
                text.contains(expect_substr),
                "route {path} missing expected substring `{expect_substr}`"
            );
        }
    }
}

#[test]
fn css_fonts_ssr_html_and_wasm_respect_budgets_toml() {
    let root = repo_root();
    let content = load_from_dir(&root.join("content")).expect("content loads");

    let css = combined_stylesheet();
    let css_br = brotli_size(css.as_bytes());
    let css_zst = zstd_size(css.as_bytes());
    assert!(
        css_br <= 40_000,
        "CSS brotli size {css_br} B exceeds 40,000 B limit"
    );
    assert!(
        css_zst <= 40_000,
        "CSS zstd size {css_zst} B exceeds 40,000 B limit"
    );

    let font_a = fs::metadata(root.join("public/fonts/public-sans-var-latin.woff2"))
        .expect("font a")
        .len();
    let font_b = fs::metadata(root.join("public/fonts/jetbrains-mono-var-latin.woff2"))
        .expect("font b")
        .len();
    assert!(
        font_a + font_b <= 70_000,
        "fonts total {} B exceeds 70,000 B limit",
        font_a + font_b
    );

    for route in [
        "/",
        "/projects",
        "/projects/nomos",
        "/wiki",
        "/blog",
        "/biography",
        "/archive",
    ] {
        let page = render_route(&content, route);
        let html_raw = page.html.len();
        let html_br = brotli_size(page.html.as_bytes());
        let html_zst = zstd_size(page.html.as_bytes());
        assert!(
            html_raw <= 90_000,
            "SSR HTML uncompressed size on {route} ({html_raw} B) exceeds 90,000 B limit"
        );
        assert!(
            html_br <= 25_000,
            "SSR HTML brotli size on {route} ({html_br} B) exceeds 25,000 B limit"
        );
        assert!(
            html_zst <= 25_000,
            "SSR HTML zstd size on {route} ({html_zst} B) exceeds 25,000 B limit"
        );
    }

    // When browser-bound WASM is staged in target/site/pkg, enforce <= 180,000 B raw & <= 80,000 B br.
    let pkg_dir = root.join("target/site/pkg");
    if let Ok(entries) = fs::read_dir(&pkg_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_s = name.to_string_lossy();
            if name_s.starts_with("site_app") && name_s.ends_with("_bg.wasm") {
                let bytes = fs::read(entry.path()).expect("read site_app*_bg.wasm");
                assert!(
                    bytes.len() <= 180_000,
                    "{name_s} ({} B) exceeds 180,000 B post-wasm-opt budget",
                    bytes.len()
                );
                let br = brotli_size(&bytes);
                assert!(
                    br <= 80_000,
                    "brotli {name_s} ({br} B) exceeds 80,000 B budget"
                );
            }
        }
    }
}
