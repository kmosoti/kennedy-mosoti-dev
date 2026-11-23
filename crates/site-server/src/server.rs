//! Axum router serving startup pre-rendered SSR `Bytes` + BLAKE3 `ETag`/`304`,
//! content-hashed immutable `/pkg/*` assets, wiki BM25/trigram search, and Zstd/Brotli/Gzip.

use crate::cache::{
    CachedAsset, DAY_CACHE, HOUR_CACHE, IMMUTABLE, NO_CACHE, ResolvedPkgAssets, WIKI_SEARCH_JS,
    resolve_pkg_assets,
};
use crate::headers::{
    cache_layer, csp_layer, frame_deny_layer, nosniff_layer, permissions_layer, referrer_layer,
};
use crate::search_routes::{SearchLimitConfig, serve_wiki_search_api, serve_wiki_ssr_search};
use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, Uri};
use axum::response::Response;
use axum::routing::get;
use site_app::router::{AssetManifest, render_route_with_assets};
use site_content::{
    SiteContent, WikiSearchIndex, build_atom_feed, build_rss_feed, build_sitemap_xml,
};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Semaphore;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::services::{ServeDir, ServeFile};

/// Shared immutable server state holding pre-rendered `Bytes` + BLAKE3 `ETag` caches.
/// Zero per-request SSR re-rendering or `String::clone` on static routes and feeds.
#[derive(Clone)]
pub struct AppState {
    pub content: Arc<SiteContent>,
    pub assets: Arc<AssetManifest>,
    pub wiki_search: Arc<WikiSearchIndex>,
    pub ssr_cache: Arc<HashMap<String, CachedAsset>>,
    pub not_found_page: Arc<CachedAsset>,
    pub health_json: Arc<CachedAsset>,
    pub wiki_search_json: Arc<CachedAsset>,
    pub atom_xml: Arc<CachedAsset>,
    pub rss_xml: Arc<CachedAsset>,
    pub sitemap_xml: Arc<CachedAsset>,
    pub pkg_assets: Arc<HashMap<String, CachedAsset>>,
    pub search_semaphore: Arc<Semaphore>,
    pub search_limits: SearchLimitConfig,
}

impl AppState {
    /// Builds server state with default asset lookup (used when `pkg_dir` is empty).
    pub fn new(content: SiteContent) -> Self {
        Self::from_pkg_dir(content, Path::new(""))
    }

    /// Pre-renders all static SSR routes, feeds, CSS, and content-hashed `/pkg/*` assets at startup.
    pub fn from_pkg_dir(content: SiteContent, pkg_dir: &Path) -> Self {
        Self::from_pkg_dir_with_limits(content, pkg_dir, SearchLimitConfig::default())
    }

    /// Pre-renders all static routes and configures explicit dynamic search concurrency/timeout budgets.
    pub fn from_pkg_dir_with_limits(
        content: SiteContent,
        pkg_dir: &Path,
        search_limits: SearchLimitConfig,
    ) -> Self {
        let combined_css = site_app::css::combined_stylesheet();
        let resolved = resolve_pkg_assets(pkg_dir, &combined_css, WIKI_SEARCH_JS);
        let manifest = resolved.manifest.clone();

        let wiki_search = WikiSearchIndex::build(&content.wiki);
        let wiki_search_json = CachedAsset::ok(
            wiki_search.to_client_json(),
            "application/json; charset=utf-8",
            HOUR_CACHE,
        );
        let atom_xml = CachedAsset::ok(
            build_atom_feed(&content),
            "application/atom+xml; charset=utf-8",
            HOUR_CACHE,
        );
        let rss_xml = CachedAsset::ok(
            build_rss_feed(&content),
            "application/rss+xml; charset=utf-8",
            HOUR_CACHE,
        );
        let sitemap_xml = CachedAsset::ok(
            build_sitemap_xml(&content),
            "application/xml; charset=utf-8",
            HOUR_CACHE,
        );
        let health_body = serde_json::json!({
            "status": "ok",
            "projects": content.projects.len(),
            "wiki": content.wiki.len(),
            "blog": content.blog.len(),
            "records": content.records.len(),
        })
        .to_string();
        let health_json = CachedAsset::ok(health_body, "application/json; charset=utf-8", NO_CACHE);

        let ssr_cache = build_ssr_route_cache(&content, &manifest);
        let nf = render_route_with_assets(&content, "/404", &manifest);
        let not_found_page = CachedAsset::with_status(
            StatusCode::NOT_FOUND,
            nf.html,
            "text/html; charset=utf-8",
            NO_CACHE,
        );

        let pkg_assets = build_pkg_asset_map(resolved);
        let max_permits = search_limits.max_concurrent.max(1);

        Self {
            content: Arc::new(content),
            assets: Arc::new(manifest),
            wiki_search: Arc::new(wiki_search),
            ssr_cache: Arc::new(ssr_cache),
            not_found_page: Arc::new(not_found_page),
            health_json: Arc::new(health_json),
            wiki_search_json: Arc::new(wiki_search_json),
            atom_xml: Arc::new(atom_xml),
            rss_xml: Arc::new(rss_xml),
            sitemap_xml: Arc::new(sitemap_xml),
            pkg_assets: Arc::new(pkg_assets),
            search_semaphore: Arc::new(Semaphore::new(max_permits)),
            search_limits,
        }
    }
}

fn build_ssr_route_cache(
    content: &SiteContent,
    manifest: &AssetManifest,
) -> HashMap<String, CachedAsset> {
    let mut routes = vec![
        "/".to_owned(),
        "/projects".to_owned(),
        "/wiki".to_owned(),
        "/blog".to_owned(),
        "/biography".to_owned(),
        "/archive".to_owned(),
    ];
    for p in &content.projects {
        routes.push(format!("/projects/{}", p.slug));
    }
    for w in &content.wiki {
        routes.push(format!("/wiki/{}", w.slug));
    }
    for b in &content.blog {
        routes.push(format!("/blog/{}", b.slug));
    }
    for r in &content.records {
        routes.push(format!("/archive/{}", r.slug));
    }

    let mut map = HashMap::with_capacity(routes.len());
    for route in routes {
        let page = render_route_with_assets(content, &route, manifest);
        let cached = CachedAsset::ok(page.html, "text/html; charset=utf-8", NO_CACHE);
        map.insert(route, cached);
    }
    map
}

fn build_pkg_asset_map(resolved: ResolvedPkgAssets) -> HashMap<String, CachedAsset> {
    let mut map = HashMap::new();
    map.insert(resolved.css_path, resolved.css_immutable);
    map.insert("/pkg/site.css".to_owned(), resolved.css_compat);
    map.insert(resolved.boot_path, resolved.boot_immutable);
    map.insert("/pkg/site_boot.js".to_owned(), resolved.boot_compat);
    map.insert(resolved.wiki_search_path, resolved.wiki_search_immutable);
    map.insert(
        "/pkg/wiki_search.js".to_owned(),
        resolved.wiki_search_compat,
    );
    if let Some(wasm) = resolved.wasm_immutable {
        map.insert(resolved.wasm_path, wasm);
    }
    if let Some(js) = resolved.js_immutable {
        map.insert(resolved.js_path, js);
    }
    map
}

/// Builds the complete Axum `Router` for serving and integration testing.
pub fn build_router(content: SiteContent, public_dir: PathBuf, pkg_dir: PathBuf) -> Router {
    let state = AppState::from_pkg_dir(content, &pkg_dir);
    build_router_from_state(state, public_dir, pkg_dir)
}

/// Builds the Axum `Router` with explicit dynamic search concurrency/timeout budgets.
pub fn build_router_with_limits(
    content: SiteContent,
    public_dir: PathBuf,
    pkg_dir: PathBuf,
    search_limits: SearchLimitConfig,
) -> Router {
    let state = AppState::from_pkg_dir_with_limits(content, &pkg_dir, search_limits);
    build_router_from_state(state, public_dir, pkg_dir)
}

/// Builds the Axum `Router` from an existing [`AppState`].
pub fn build_router_from_state(state: AppState, public_dir: PathBuf, pkg_dir: PathBuf) -> Router {
    let precompressed_dir = |path: PathBuf| {
        ServeDir::new(path)
            .precompressed_zstd()
            .precompressed_br()
            .precompressed_gzip()
    };

    let mut pkg_router = Router::new();
    for path in state.pkg_assets.keys() {
        if let Some(rel) = path.strip_prefix("/pkg") {
            pkg_router = pkg_router.route(rel, get(serve_pkg_cached));
        }
    }
    let pkg_router = pkg_router.fallback_service(
        ServiceBuilder::new()
            .layer(cache_layer(IMMUTABLE))
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

async fn serve_health(State(state): State<AppState>, headers: HeaderMap) -> Response {
    state.health_json.respond(&headers)
}

async fn serve_feed(State(state): State<AppState>, headers: HeaderMap) -> Response {
    state.atom_xml.respond(&headers)
}

async fn serve_rss(State(state): State<AppState>, headers: HeaderMap) -> Response {
    state.rss_xml.respond(&headers)
}

async fn serve_sitemap(State(state): State<AppState>, headers: HeaderMap) -> Response {
    state.sitemap_xml.respond(&headers)
}

async fn serve_wiki_search_json(State(state): State<AppState>, headers: HeaderMap) -> Response {
    state.wiki_search_json.respond(&headers)
}

async fn serve_pkg_cached(State(state): State<AppState>, headers: HeaderMap, uri: Uri) -> Response {
    let key = format!("/pkg{}", uri.path());
    if let Some(asset) = state.pkg_assets.get(&key) {
        return asset.respond(&headers);
    }
    state.not_found_page.respond(&headers)
}

async fn serve_ssr_route(State(state): State<AppState>, headers: HeaderMap, uri: Uri) -> Response {
    let raw_path = uri.path();
    let trimmed = raw_path.trim_end_matches('/');
    let route = if trimmed.is_empty() { "/" } else { trimmed };

    if route == "/wiki"
        && let Some(qs) = uri.query()
        && let Some(resp) = serve_wiki_ssr_search(&state, qs).await
    {
        return resp;
    }

    if let Some(cached) = state.ssr_cache.get(route) {
        return cached.respond(&headers);
    }
    state.not_found_page.respond(&headers)
}
