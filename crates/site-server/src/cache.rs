//! Pre-rendered immutable `Bytes` cache, BLAKE3 strong `ETag` + `304 Not Modified`,
//! and content-hashed `/pkg/*` asset URL resolution.

use axum::body::Bytes;
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use site_app::router::AssetManifest;
use std::fs;
use std::path::Path;

pub const NO_CACHE: &str = "no-cache";
pub const IMMUTABLE: &str = "public, max-age=31536000, immutable";
pub const DAY_CACHE: &str = "public, max-age=86400";
pub const HOUR_CACHE: &str = "public, max-age=3600";

/// Progressive-enhancement ES module for live BM25 + trigram filtering on `/wiki`.
pub const WIKI_SEARCH_JS: &str = r#"const input = document.getElementById("wiki-search-input");
const grid = document.getElementById("wiki-card-grid");
const status = document.getElementById("wiki-search-status");
const form = input && input.closest("form");
if (input && grid && form) {
  let indexPromise = null;
  const loadIndex = () => {
    if (!indexPromise) {
      const url = form.dataset.wikiSearchIndex || "/wiki/search.json";
      indexPromise = fetch(url).then((r) => (r.ok ? r.json() : [])).catch(() => []);
    }
    return indexPromise;
  };
  const tokenize = (s) => s.toLowerCase().split(/[^a-z0-9]+/).filter((t) => t.length > 1);
  const trigrams = (s) => {
    const out = new Set();
    for (const w of tokenize(s)) {
      for (let i = 0; i + 3 <= w.length; i++) out.add(w.slice(i, i + 3));
    }
    return out;
  };
  const cards = Array.from(grid.querySelectorAll("article[data-slug]"));
  const bySlug = new Map(cards.map((c) => [c.dataset.slug, c]));
  input.addEventListener("focus", loadIndex, { once: true });
  input.addEventListener("input", async () => {
    const q = input.value.trim();
    if (!q) {
      for (const c of cards) { c.hidden = false; grid.appendChild(c); }
      if (status) status.textContent = `Showing all ${cards.length} wiki articles.`;
      return;
    }
    const docs = await loadIndex();
    const qTerms = tokenize(q);
    const qTris = trigrams(q);
    const scored = [];
    for (const d of docs) {
      const termSet = new Set(d.terms || []);
      let score = 0;
      for (const t of qTerms) {
        if (termSet.has(t)) score += 2.0;
        else if ((d.title || "").toLowerCase().includes(t)) score += 1.5;
      }
      if (score === 0 && qTris.size > 0) {
        const dTris = trigrams(`${d.title} ${d.summary} ${(d.tags || []).join(" ")}`);
        let m = 0;
        for (const tri of qTris) if (dTris.has(tri)) m++;
        const ratio = m / qTris.size;
        if (ratio >= 0.34) score = ratio * 0.85;
      }
      if (score > 0) scored.push([d.slug, score]);
    }
    scored.sort((a, b) => b[1] - a[1] || a[0].localeCompare(b[0]));
    const matched = new Set(scored.map(([slug]) => slug));
    for (const c of cards) c.hidden = !matched.has(c.dataset.slug);
    for (const [slug] of scored) {
      const el = bySlug.get(slug);
      if (el) grid.appendChild(el);
    }
    if (status) status.textContent = `${scored.length} result(s) for "${q}" (BM25 + trigram).`;
  });
}
"#;

/// Pre-rendered immutable HTTP response payload with precomputed BLAKE3 `ETag`.
#[derive(Clone)]
pub struct CachedAsset {
    pub status: StatusCode,
    pub body: Bytes,
    pub etag: HeaderValue,
    pub content_type: &'static str,
    pub cache_control: &'static str,
}

impl CachedAsset {
    /// Builds a `200 OK` cached asset with a strong 128-bit BLAKE3 hex `ETag`.
    pub fn ok(
        body: impl Into<Bytes>,
        content_type: &'static str,
        cache_control: &'static str,
    ) -> Self {
        Self::with_status(StatusCode::OK, body, content_type, cache_control)
    }

    /// Builds a cached asset with explicit HTTP `status` and BLAKE3 `ETag`.
    pub fn with_status(
        status: StatusCode,
        body: impl Into<Bytes>,
        content_type: &'static str,
        cache_control: &'static str,
    ) -> Self {
        let bytes = body.into();
        let hex = blake3::hash(bytes.as_ref()).to_hex();
        let etag_str = format!("\"{}\"", &hex[..32]);
        let etag = HeaderValue::from_str(&etag_str).expect("valid ASCII hex ETag");
        Self {
            status,
            body: bytes,
            etag,
            content_type,
            cache_control,
        }
    }

    /// Serves `304 Not Modified` on matching `If-None-Match`, or `O(1)` `Bytes::clone()` body.
    pub fn respond(&self, req_headers: &HeaderMap) -> Response {
        if self.status == StatusCode::OK && etag_matches(req_headers, &self.etag) {
            let mut resp = StatusCode::NOT_MODIFIED.into_response();
            resp.headers_mut().insert(header::ETAG, self.etag.clone());
            resp.headers_mut().insert(
                header::CACHE_CONTROL,
                HeaderValue::from_static(self.cache_control),
            );
            return resp;
        }
        let mut resp = (self.status, self.body.clone()).into_response();
        resp.headers_mut().insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static(self.content_type),
        );
        resp.headers_mut().insert(
            header::CACHE_CONTROL,
            HeaderValue::from_static(self.cache_control),
        );
        resp.headers_mut().insert(header::ETAG, self.etag.clone());
        resp
    }
}

/// Returns a deterministic 12-character lowercase BLAKE3 hex digest for asset filenames.
#[must_use]
pub fn short_blake3(data: &[u8]) -> String {
    blake3::hash(data).to_hex()[..12].to_owned()
}

/// Evaluates HTTP `If-None-Match` against a strong `ETag` (supporting weak `W/` and lists).
#[must_use]
pub fn etag_matches(headers: &HeaderMap, etag: &HeaderValue) -> bool {
    let Some(inm) = headers
        .get(header::IF_NONE_MATCH)
        .and_then(|v| v.to_str().ok())
    else {
        return false;
    };
    let Ok(target) = etag.to_str() else {
        return false;
    };
    let target_clean = target.trim().strip_prefix("W/").unwrap_or(target.trim());
    for candidate in inm.split(',') {
        let c = candidate.trim();
        if c == "*" {
            return true;
        }
        let c_clean = c.strip_prefix("W/").unwrap_or(c);
        if c_clean == target_clean {
            return true;
        }
    }
    false
}

/// Resolved content-hashed `/pkg/*` asset URLs and pre-cached immutable payloads.
pub struct ResolvedPkgAssets {
    pub manifest: AssetManifest,
    pub css_path: String,
    pub boot_path: String,
    pub wiki_search_path: String,
    pub wasm_path: String,
    pub js_path: String,
    pub css_immutable: CachedAsset,
    pub css_compat: CachedAsset,
    pub boot_immutable: CachedAsset,
    pub boot_compat: CachedAsset,
    pub wiki_search_immutable: CachedAsset,
    pub wiki_search_compat: CachedAsset,
    pub wasm_immutable: Option<CachedAsset>,
    pub js_immutable: Option<CachedAsset>,
}

/// Resolves content-hashed URLs (`/pkg/site.<hash>.css`, `/pkg/site_app.<hash>_bg.wasm`,
/// `/pkg/site_app.<hash>.js`, `/pkg/site_boot.<hash>.js`, `/pkg/wiki_search.<hash>.js`).
pub fn resolve_pkg_assets(pkg_dir: &Path, css: &str, wiki_search_js: &str) -> ResolvedPkgAssets {
    let css_bytes = Bytes::from(css.to_owned());
    let css_hash = short_blake3(&css_bytes);
    let css_path = format!("/pkg/site.{css_hash}.css");

    let ws_bytes = Bytes::from(wiki_search_js.to_owned());
    let ws_hash = short_blake3(&ws_bytes);
    let wiki_search_path = format!("/pkg/wiki_search.{ws_hash}.js");

    let wasm_raw = read_pkg_artifact(pkg_dir, "site_app_bg.wasm", "_bg.wasm");
    let js_raw = read_pkg_artifact(pkg_dir, "site_app.js", ".js");

    let wasm_hash = wasm_raw
        .as_ref()
        .map(|b| short_blake3(b))
        .unwrap_or_else(|| css_hash.clone());
    let js_hash = js_raw
        .as_ref()
        .map(|b| short_blake3(b))
        .unwrap_or_else(|| css_hash.clone());

    let wasm_path = format!("/pkg/site_app.{wasm_hash}_bg.wasm");
    let js_path = format!("/pkg/site_app.{js_hash}.js");

    let boot_src =
        format!("import init from \"{js_path}\";\ninit({{ module_or_path: \"{wasm_path}\" }});\n");
    let boot_bytes = Bytes::from(boot_src);
    let boot_hash = short_blake3(&boot_bytes);
    let boot_path = format!("/pkg/site_boot.{boot_hash}.js");

    let manifest = AssetManifest {
        css_href: css_path.clone(),
        island_boot_src: boot_path.clone(),
        wiki_search_src: wiki_search_path.clone(),
    };

    let wasm_immutable = wasm_raw.map(|b| CachedAsset::ok(b, "application/wasm", IMMUTABLE));
    let js_immutable =
        js_raw.map(|b| CachedAsset::ok(b, "application/javascript; charset=utf-8", IMMUTABLE));

    ResolvedPkgAssets {
        manifest,
        css_path,
        boot_path,
        wiki_search_path,
        wasm_path,
        js_path,
        css_immutable: CachedAsset::ok(css_bytes.clone(), "text/css; charset=utf-8", IMMUTABLE),
        css_compat: CachedAsset::ok(css_bytes, "text/css; charset=utf-8", NO_CACHE),
        boot_immutable: CachedAsset::ok(
            boot_bytes.clone(),
            "application/javascript; charset=utf-8",
            IMMUTABLE,
        ),
        boot_compat: CachedAsset::ok(
            boot_bytes,
            "application/javascript; charset=utf-8",
            NO_CACHE,
        ),
        wiki_search_immutable: CachedAsset::ok(
            ws_bytes.clone(),
            "application/javascript; charset=utf-8",
            IMMUTABLE,
        ),
        wiki_search_compat: CachedAsset::ok(
            ws_bytes,
            "application/javascript; charset=utf-8",
            NO_CACHE,
        ),
        wasm_immutable,
        js_immutable,
    }
}

fn read_pkg_artifact(pkg_dir: &Path, exact_name: &str, suffix: &str) -> Option<Bytes> {
    let direct = pkg_dir.join(exact_name);
    if let Ok(bytes) = fs::read(&direct) {
        return Some(Bytes::from(bytes));
    }
    let entries = fs::read_dir(pkg_dir).ok()?;
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_s = name.to_string_lossy();
        if name_s.starts_with("site_app.")
            && name_s.ends_with(suffix)
            && !name_s.ends_with(".br")
            && !name_s.ends_with(".gz")
            && !name_s.ends_with(".zst")
            && let Ok(bytes) = fs::read(entry.path())
        {
            return Some(Bytes::from(bytes));
        }
    }
    None
}
