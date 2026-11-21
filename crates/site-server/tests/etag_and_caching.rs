//! Integration tests for startup pre-rendered SSR `Bytes`, BLAKE3 `ETag` / `304 Not Modified`,
//! and content-hashed immutable `/pkg/*` assets.

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode, header};
use site_content::load_from_dir;
use site_server::build_router;
use std::path::PathBuf;
use tower::Service;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn extract_first_attr(html: &str, prefix: &str, end_delim: &str) -> Option<String> {
    let start = html.find(prefix)?;
    let quote_pos = prefix.find('"')?;
    let val_start = start + quote_pos + 1;
    let rest = &html[val_start..];
    let end = rest.find(end_delim)?;
    Some(rest[..end].to_owned())
}

#[tokio::test]
async fn pre_rendered_routes_and_hashed_assets_support_etag_304_and_immutable_cache() {
    let root = repo_root();
    let content = load_from_dir(&root.join("content")).expect("content loads");
    let mut app = build_router(content, root.join("public"), root.join("target/site/pkg"));

    // Fetch `/` and extract content-hashed CSS + boot JS URLs emitted in SSR HTML.
    let home_resp = app
        .call(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(home_resp.status(), StatusCode::OK);
    let home_etag = home_resp
        .headers()
        .get(header::ETAG)
        .expect("home response must include BLAKE3 ETag")
        .clone();
    let home_body = to_bytes(home_resp.into_body(), 1024 * 1024).await.unwrap();
    let home_html = std::str::from_utf8(&home_body).unwrap();

    let css_href = extract_first_attr(home_html, "href=\"/pkg/site.", "\"")
        .expect("home HTML must link content-hashed /pkg/site.<hash>.css");
    let boot_src = extract_first_attr(home_html, "src=\"/pkg/site_boot.", "\"")
        .expect("home HTML must link content-hashed /pkg/site_boot.<hash>.js");

    // Matching If-None-Match returns 304 Not Modified with empty body.
    let not_mod = app
        .call(
            Request::builder()
                .uri("/")
                .header(header::IF_NONE_MATCH, home_etag)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(not_mod.status(), StatusCode::NOT_MODIFIED);
    let not_mod_body = to_bytes(not_mod.into_body(), 1024).await.unwrap();
    assert!(not_mod_body.is_empty(), "304 response body must be empty");

    // Mismatched If-None-Match returns 200 OK.
    let mismatch = app
        .call(
            Request::builder()
                .uri("/")
                .header(header::IF_NONE_MATCH, "\"0000000000000000\"")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(mismatch.status(), StatusCode::OK);

    // Content-hashed CSS & boot JS serve immutable Cache-Control and support 304.
    for hashed_url in [css_href.as_str(), boot_src.as_str()] {
        let resp = app
            .call(
                Request::builder()
                    .uri(hashed_url)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "failed to load {hashed_url}");
        let cc = resp
            .headers()
            .get(header::CACHE_CONTROL)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");
        assert_eq!(
            cc, "public, max-age=31536000, immutable",
            "{hashed_url} must carry immutable Cache-Control"
        );
        let etag = resp
            .headers()
            .get(header::ETAG)
            .expect("hashed asset must have ETag")
            .clone();
        let resp304 = app
            .call(
                Request::builder()
                    .uri(hashed_url)
                    .header(header::IF_NONE_MATCH, etag)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp304.status(), StatusCode::NOT_MODIFIED);
    }

    // Boot JS references content-hashed site_app.<hash>.js and site_app.<hash>_bg.wasm.
    let boot_resp = app
        .call(
            Request::builder()
                .uri(boot_src.as_str())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let boot_bytes = to_bytes(boot_resp.into_body(), 16 * 1024).await.unwrap();
    let boot_text = std::str::from_utf8(&boot_bytes).unwrap();
    assert!(
        boot_text.contains("/pkg/site_app.") && boot_text.contains("_bg.wasm"),
        "boot JS must reference content-hashed site_app.<hash>_bg.wasm: {boot_text}"
    );
    if root.join("target/site/pkg/site_app_bg.wasm").exists() {
        let js_url = extract_first_attr(boot_text, "from \"/pkg/site_app.", "\"")
            .expect("boot JS must contain hashed site_app.<hash>.js");
        let wasm_url = extract_first_attr(boot_text, "module_or_path: \"/pkg/site_app.", "\"")
            .expect("boot JS must contain hashed site_app.<hash>_bg.wasm");
        for (url, expect_ct) in [
            (js_url.as_str(), "application/javascript"),
            (wasm_url.as_str(), "application/wasm"),
        ] {
            let r = app
                .call(Request::builder().uri(url).body(Body::empty()).unwrap())
                .await
                .unwrap();
            assert_eq!(r.status(), StatusCode::OK, "hashed asset {url} must load");
            assert!(
                r.headers()
                    .get(header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .contains(expect_ct)
            );
            assert_eq!(
                r.headers()
                    .get(header::CACHE_CONTROL)
                    .and_then(|v| v.to_str().ok()),
                Some("public, max-age=31536000, immutable")
            );
        }
    }

    // Feeds and search index also return ETag and honor If-None-Match -> 304.
    for path in ["/feed.xml", "/rss.xml", "/sitemap.xml", "/wiki/search.json"] {
        let r1 = app
            .call(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(r1.status(), StatusCode::OK);
        let tag = r1
            .headers()
            .get(header::ETAG)
            .expect("feed/index ETag")
            .clone();
        let r2 = app
            .call(
                Request::builder()
                    .uri(path)
                    .header(header::IF_NONE_MATCH, tag)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(
            r2.status(),
            StatusCode::NOT_MODIFIED,
            "{path} must return 304"
        );
    }
}
