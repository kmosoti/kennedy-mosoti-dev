//! Axum router wiring SSR routes, Leptos island assets, wiki BM25/trigram search, RSS/Atom, and compression.

use crate::headers::{
    cache_layer, csp_layer, frame_deny_layer, nosniff_layer, permissions_layer, referrer_layer,
};
use axum::Router;
use axum::extract::State;
use axum::http::{HeaderValue, StatusCode, Uri, header};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use site_content::{
    SiteContent, WikiSearchIndex, build_atom_feed, build_rss_feed, build_sitemap_xml,
};
use std::path::PathBuf;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

const NO_CACHE: &str = "no-cache";
const IMMUTABLE: &str = "public, max-age=31536000, immutable";
const DAY_CACHE: &str = "public, max-age=86400";

const ISLAND_BOOT_JS: &str = "import init from \"/pkg/site_app.js\";\ninit();\n";

const WIKI_SEARCH_JS: &str = r#"const input = document.getElementById("wiki-search-input");
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

/// Shared immutable server state holding validated site content, search index, and pre-rendered feeds.
#[derive(Clone)]
pub struct AppState {
    pub content: Arc<SiteContent>,
    pub wiki_search: Arc<WikiSearchIndex>,
    pub wiki_search_json: Arc<String>,
    pub atom_xml: Arc<String>,
    pub rss_xml: Arc<String>,
    pub sitemap_xml: Arc<String>,
    pub combined_css: Arc<String>,
}

impl AppState {
    /// Builds server state from a validated `SiteContent` bundle.
    pub fn new(content: SiteContent) -> Self {
        let wiki_search = WikiSearchIndex::build(&content.wiki);
        let wiki_search_json = Arc::new(wiki_search.to_client_json());
        let atom_xml = Arc::new(build_atom_feed(&content));
        let rss_xml = Arc::new(build_rss_feed(&content));
        let sitemap_xml = Arc::new(build_sitemap_xml(&content));
        let combined_css = Arc::new(site_app::css::combined_stylesheet());
        Self {
            content: Arc::new(content),
            wiki_search: Arc::new(wiki_search),
            wiki_search_json,
            atom_xml,
            rss_xml,
            sitemap_xml,
            combined_css,
        }
    }
}

/// Builds the complete Axum `Router` for serving and integration testing.
pub fn build_router(content: SiteContent, public_dir: PathBuf, pkg_dir: PathBuf) -> Router {
    let state = AppState::new(content);

    let precompressed_dir =
        |path: PathBuf| ServeDir::new(path).precompressed_br().precompressed_gzip();

    let pkg_router = Router::new()
        .route("/site.css", get(serve_css))
        .route("/site_boot.js", get(serve_boot_js))
        .route("/wiki_search.js", get(serve_wiki_search_js))
        .fallback_service(
            ServiceBuilder::new()
                .layer(cache_layer(NO_CACHE))
                .service(precompressed_dir(pkg_dir)),
        );

    Router::new()
        .route("/health", get(serve_health))
        .route("/feed.xml", get(serve_feed))
        .route("/atom.xml", get(serve_feed))
        .route("/rss.xml", get(serve_rss))
        .route("/sitemap.xml", get(serve_sitemap))
        .route("/wiki/search.json", get(serve_wiki_search_json))
        .route("/wiki/search", get(serve_wiki_search_api))
        .nest("/pkg", pkg_router)
        .nest_service(
            "/fonts",
            ServiceBuilder::new()
                .layer(cache_layer(IMMUTABLE))
                .service(precompressed_dir(public_dir.join("fonts"))),
        )
        .nest_service(
            "/icons",
            ServiceBuilder::new()
                .layer(cache_layer(DAY_CACHE))
                .service(precompressed_dir(public_dir.join("icons"))),
        )
        .route_service(
            "/robots.txt",
            ServiceBuilder::new()
                .layer(cache_layer(DAY_CACHE))
                .service(ServeFile::new(public_dir.join("robots.txt"))),
        )
        .route_service(
            "/manifest.webmanifest",
            ServiceBuilder::new()
                .layer(cache_layer(NO_CACHE))
                .service(ServeFile::new(public_dir.join("manifest.webmanifest"))),
        )
        .fallback(get(serve_ssr_route))
        .with_state(state)
        .layer(
            ServiceBuilder::new()
                .layer(csp_layer())
                .layer(nosniff_layer())
                .layer(referrer_layer())
                .layer(frame_deny_layer())
                .layer(permissions_layer())
                .layer(CompressionLayer::new()),
        )
}

async fn serve_health(State(state): State<AppState>) -> impl IntoResponse {
    let body = serde_json::json!({
        "status": "ok",
        "projects": state.content.projects.len(),
        "wiki": state.content.wiki.len(),
        "blog": state.content.blog.len(),
        "records": state.content.records.len(),
    });
    (
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/json; charset=utf-8")],
        body.to_string(),
    )
}

async fn serve_feed(State(state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/atom+xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        (*state.atom_xml).clone(),
    )
}

async fn serve_rss(State(state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/rss+xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        (*state.rss_xml).clone(),
    )
}

async fn serve_sitemap(State(state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/xml; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        (*state.sitemap_xml).clone(),
    )
}

async fn serve_wiki_search_json(State(state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        (*state.wiki_search_json).clone(),
    )
}

async fn serve_wiki_search_api(State(state): State<AppState>, uri: Uri) -> impl IntoResponse {
    let q = uri
        .query()
        .and_then(|qs| site_app::router::extract_query_param(qs, "q"))
        .unwrap_or_default();
    let hits = state.wiki_search.search(&q, 20);
    let body = serde_json::json!({ "query": q, "hits": hits });
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "application/json; charset=utf-8"),
            (header::CACHE_CONTROL, NO_CACHE),
        ],
        body.to_string(),
    )
}

async fn serve_css(State(state): State<AppState>) -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "text/css; charset=utf-8"),
            (header::CACHE_CONTROL, NO_CACHE),
        ],
        (*state.combined_css).clone(),
    )
}

async fn serve_boot_js() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (
                header::CONTENT_TYPE,
                "application/javascript; charset=utf-8",
            ),
            (header::CACHE_CONTROL, NO_CACHE),
        ],
        ISLAND_BOOT_JS,
    )
}

async fn serve_wiki_search_js() -> impl IntoResponse {
    (
        StatusCode::OK,
        [
            (
                header::CONTENT_TYPE,
                "application/javascript; charset=utf-8",
            ),
            (header::CACHE_CONTROL, NO_CACHE),
        ],
        WIKI_SEARCH_JS,
    )
}

async fn serve_ssr_route(State(state): State<AppState>, uri: Uri) -> Response {
    let target = uri
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or_else(|| uri.path());
    let page = site_app::router::render_route(&state.content, target);
    let code = StatusCode::from_u16(page.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut resp = (code, page.html).into_response();
    resp.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    resp.headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static(NO_CACHE));
    resp
}
