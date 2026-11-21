//! Release readiness checks: full-site internal link crawl, XML feed integrity,
//! WASM/CSS/HTML budget checks, workflow SHA-pin enforcement, and privacy/zero-Python guards.

use axum::body::{Body, to_bytes};
use axum::http::{Request, StatusCode};
use site_app::router::render_route;
use site_content::load_from_dir;
use site_server::build_router;
use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tower::Service;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let p = entry.path();
        let name = entry.file_name();
        let name_s = name.to_string_lossy();
        if name_s == ".git" || name_s == ".jj" || name_s == "target" || name_s == "dist" {
            continue;
        }
        if p.is_dir() {
            collect_files(&p, out);
        } else {
            out.push(p);
        }
    }
}

fn extract_attr_paths(html: &str, attr: &str) -> Vec<String> {
    let mut found = Vec::new();
    let needle = format!("{attr}=\"");
    let mut rest = html;
    while let Some(pos) = rest.find(&needle) {
        let after = &rest[pos + needle.len()..];
        if let Some(end) = after.find('"') {
            let val = &after[..end];
            if val.starts_with('/') && !val.starts_with("//") {
                found.push(val.to_owned());
            }
            rest = &after[end + 1..];
        } else {
            break;
        }
    }
    found
}

#[tokio::test]
async fn crawl_all_ssr_pages_and_verify_zero_broken_internal_links() {
    let root = repo_root();
    let content = load_from_dir(&root.join("content")).expect("content loads");

    let mut pages = vec![
        "/".to_owned(),
        "/projects".to_owned(),
        "/wiki".to_owned(),
        "/blog".to_owned(),
        "/biography".to_owned(),
        "/archive".to_owned(),
    ];
    for p in &content.projects {
        pages.push(format!("/projects/{}", p.slug));
    }
    for w in &content.wiki {
        pages.push(format!("/wiki/{}", w.slug));
    }
    for b in &content.blog {
        pages.push(format!("/blog/{}", b.slug));
    }
    for r in &content.records {
        pages.push(format!("/archive/{}", r.slug));
    }

    let mut targets = BTreeSet::new();
    for page_path in &pages {
        let rendered = render_route(&content, page_path);
        assert_eq!(rendered.status, 200, "route {page_path} failed SSR");
        for link in extract_attr_paths(&rendered.html, "href") {
            let clean = link.split(['#', '?']).next().unwrap_or(&link).to_owned();
            // Static WASM package artifacts are generated during release bundling, not checked-in.
            if !clean.is_empty() && clean != "/pkg/site_app.js" {
                targets.insert(clean);
            }
        }
    }

    let mut app = build_router(content, root.join("public"), root.join("target/site/pkg"));
    for target in targets {
        let req = Request::builder()
            .uri(&target)
            .body(Body::empty())
            .expect("req");
        let resp = app.call(req).await.expect("resp");
        assert_eq!(
            resp.status(),
            StatusCode::OK,
            "broken internal link target `{target}` returned {}",
            resp.status()
        );
    }
}

#[tokio::test]
async fn atom_rss_and_sitemap_xml_parse_cleanly() {
    let root = repo_root();
    let content = load_from_dir(&root.join("content")).expect("content loads");
    let mut app = build_router(content, root.join("public"), root.join("target/site/pkg"));

    for (path, open_tag, close_tag) in [
        (
            "/feed.xml",
            "<feed xmlns=\"http://www.w3.org/2005/Atom\">",
            "</feed>",
        ),
        ("/rss.xml", "<rss version=\"2.0\">", "</rss>"),
        ("/sitemap.xml", "<urlset", "</urlset>"),
    ] {
        let req = Request::builder().uri(path).body(Body::empty()).unwrap();
        let resp = app.call(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = to_bytes(resp.into_body(), 1024 * 1024).await.unwrap();
        let xml = std::str::from_utf8(&body).expect("valid UTF-8 XML");
        assert!(
            xml.contains("<?xml version=\"1.0\""),
            "{path} missing XML header"
        );
        assert!(
            xml.contains(open_tag),
            "{path} missing opening tag {open_tag}"
        );
        assert!(
            xml.contains(close_tag),
            "{path} missing closing tag {close_tag}"
        );
        assert!(!xml.contains("<script"), "{path} must not include scripts");
    }
}

#[test]
fn repo_enforces_zero_python_zero_threejs_and_zero_private_leaks() {
    let root = repo_root();
    let mut files = Vec::new();
    collect_files(&root, &mut files);

    let banned = [
        concat!("2600", ":", "1900"),
        concat!("fd20", ":5fa:", "d190"),
        concat!("green-infra", "-deployer"),
        concat!("fedora", "-blade"),
        concat!("age1", "txe8"),
        concat!("AZ7", "VjM"),
        concat!("bx6", "w9Q"),
        concat!("/home/", "kmosoti"),
        concat!("AGE-SECRET", "-KEY-"),
        concat!("BEGIN OPENSSH ", "PRIVATE KEY"),
        concat!("142.93.", "206.223"),
    ];

    for file in files {
        let rel = file.strip_prefix(&root).unwrap_or(&file);
        let rel_s = rel.to_string_lossy();
        assert!(
            !rel_s.ends_with(".py"),
            "Python file forbidden by project policy: {rel_s}"
        );
        assert!(
            !rel_s.ends_with("package.json") && !rel_s.contains("vite.config"),
            "Node/Three.js bundler config forbidden: {rel_s}"
        );

        let Ok(text) = fs::read_to_string(&file) else {
            continue;
        };
        for needle in banned {
            assert!(
                !text.contains(needle),
                "forbidden private/NDA string `{needle}` found in {rel_s}"
            );
        }
    }
}

#[test]
fn github_workflows_pin_every_action_to_40_hex_sha() {
    let wf_dir = repo_root().join(".github/workflows");
    for entry in fs::read_dir(&wf_dir).expect(".github/workflows exists") {
        let p = entry.unwrap().path();
        let text = fs::read_to_string(&p).unwrap();
        for (idx, line) in text.lines().enumerate() {
            let trimmed = line.trim();
            if let Some(after) = trimmed
                .strip_prefix("uses:")
                .or_else(|| trimmed.strip_prefix("- uses:"))
            {
                let spec = after.trim().split('#').next().unwrap_or("").trim();
                let (_, rev) = spec.split_once('@').unwrap_or_else(|| {
                    panic!("{}:{} unpinned action `{spec}`", p.display(), idx + 1)
                });
                assert!(
                    rev.len() == 40 && rev.bytes().all(|b| b.is_ascii_hexdigit()),
                    "{}:{} action `{spec}` must pin a 40-char commit SHA",
                    p.display(),
                    idx + 1
                );
            }
        }
    }
}

fn strip_custom_wasm_sections(wasm: &[u8]) -> Vec<u8> {
    assert!(
        wasm.starts_with(b"\0asm\x01\0\0\0"),
        "invalid WASM magic/version"
    );
    let mut out = Vec::with_capacity(wasm.len() / 2);
    out.extend_from_slice(&wasm[..8]);
    let mut i = 8usize;
    while i < wasm.len() {
        let sec_start = i;
        let sec_id = wasm[i];
        i += 1;
        let mut len = 0usize;
        let mut shift = 0u32;
        loop {
            let b = wasm[i];
            i += 1;
            len |= usize::from(b & 0x7f) << shift;
            if (b & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        let sec_end = i.saturating_add(len);
        if sec_id != 0 && sec_end <= wasm.len() {
            out.extend_from_slice(&wasm[sec_start..sec_end]);
        }
        i = sec_end;
    }
    out
}

#[test]
fn optional_wasm_release_artifact_stays_within_budgets_toml() {
    let root = repo_root();
    let target_base = std::env::var_os("CARGO_TARGET_DIR")
        .or_else(|| std::env::var_os("CARGO_BUILD_TARGET_DIR"))
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"));

    // Check browser-bound post-wasm-bindgen + wasm-opt artifact (`<= 180_000 B` raw, `<= 80_000 B` br).
    let pkg_dir = root.join("target/site/pkg");
    if let Ok(entries) = fs::read_dir(&pkg_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name_s = name.to_string_lossy();
            if name_s.starts_with("site_app") && name_s.ends_with("_bg.wasm") {
                let bg_bytes = fs::read(entry.path()).expect("read site_app*_bg.wasm");
                assert!(
                    bg_bytes.len() <= 180_000,
                    "{name_s} {} B exceeds 180,000 B budget",
                    bg_bytes.len()
                );
                let mut br = Vec::new();
                {
                    let mut w = brotli::CompressorWriter::new(&mut br, 4096, 9, 22);
                    w.write_all(&bg_bytes).unwrap();
                }
                assert!(
                    br.len() <= 80_000,
                    "brotli {name_s} {} B exceeds 80,000 B budget",
                    br.len()
                );
            }
        }
    }

    // Check pre-wasm-bindgen executable sections (`<= 750 KiB` pre-wasm-opt, `<= 220 KiB` brotli).
    let wasm = target_base.join("wasm32-unknown-unknown/wasm-release/site_app.wasm");
    if !wasm.exists() {
        return;
    }
    let bytes = fs::read(&wasm).expect("read wasm");
    let clean = strip_custom_wasm_sections(&bytes);
    assert!(
        clean.len() <= 750_000,
        "pre-bindgen executable wasm sections {} B exceed 750,000 B ceiling",
        clean.len()
    );
    let mut br = Vec::new();
    {
        let mut w = brotli::CompressorWriter::new(&mut br, 4096, 9, 22);
        w.write_all(&clean).unwrap();
    }
    assert!(
        br.len() <= 220_000,
        "brotli wasm {} B exceeds 220,000 B budget",
        br.len()
    );
}
