//! SSR route resolver producing complete standalone HTML documents without JS requirements.

use crate::views::layout::{NavSection, SiteShell};
use crate::views::pages_bio_archive::{ArchiveIndexPage, ArchiveRecordPage, BiographyPage};
use crate::views::pages_projects::{HomePage, ProjectDetailPage, ProjectsIndexPage};
use crate::views::pages_wiki_blog::{BlogIndexPage, BlogPostPage, WikiArticlePage, WikiIndexPage};
use leptos::prelude::*;
use site_content::SiteContent;
use site_content::markdown::{escape_attr, escape_html};

/// Asset URLs injected into SSR `<head>` tags (supports content-hashed immutable URLs).
#[derive(Debug, Clone)]
pub struct AssetManifest {
    pub css_href: String,
    pub island_boot_src: String,
    pub wiki_search_src: String,
}

impl Default for AssetManifest {
    fn default() -> Self {
        Self {
            css_href: "/pkg/site.css".to_owned(),
            island_boot_src: "/pkg/site_boot.js".to_owned(),
            wiki_search_src: "/pkg/wiki_search.js".to_owned(),
        }
    }
}

/// Rendered SSR response status, title, and complete HTML document.
#[derive(Debug, Clone)]
pub struct RenderedPage {
    pub status: u16,
    pub title: String,
    pub html: String,
}

/// Renders an SSR document for `path` using default asset URLs.
pub fn render_route(content: &SiteContent, path: &str) -> RenderedPage {
    render_route_with_assets(content, path, &AssetManifest::default())
}

/// Renders an SSR document for `path` using explicit [`AssetManifest`] URLs.
/// Pages with interactive 2.5D cockpit islands load `island_boot_src` progressively,
/// and `/wiki` loads `wiki_search_src` progressively while keeping full zero-JS fallback.
pub fn render_route_with_assets(
    content: &SiteContent,
    path: &str,
    assets: &AssetManifest,
) -> RenderedPage {
    let without_frag = path.split('#').next().unwrap_or("/");
    let (clean, query_raw) = match without_frag.split_once('?') {
        Some((p, q)) => (p, Some(q)),
        None => (without_frag, None),
    };
    let trimmed = clean.trim_end_matches('/');
    let route = if trimmed.is_empty() { "/" } else { trimmed };

    let (status, title, script_kind, body_html) = match route {
        "/" => {
            let c = content.clone();
            let title = format!("{} — Engineering Cockpit", c.identity.name);
            let html = render_view(move || view! { <HomePage content=c /> });
            (200, title, ScriptKind::Island, html)
        }
        "/projects" => {
            let c = content.clone();
            let html = render_view(move || view! { <ProjectsIndexPage content=c /> });
            (
                200,
                "Projects — Kennedy Mosoti".to_owned(),
                ScriptKind::None,
                html,
            )
        }
        "/wiki" => {
            let c = content.clone();
            let q = query_raw.and_then(|qs| extract_query_param(qs, "q"));
            let html = render_view(move || view! { <WikiIndexPage content=c query=q /> });
            (
                200,
                "Engineering Wiki — Kennedy Mosoti".to_owned(),
                ScriptKind::WikiSearch,
                html,
            )
        }
        "/blog" => {
            let c = content.clone();
            let html = render_view(move || view! { <BlogIndexPage content=c /> });
            (
                200,
                "Engineering Blog — Kennedy Mosoti".to_owned(),
                ScriptKind::None,
                html,
            )
        }
        "/biography" => {
            let c = content.clone();
            let html = render_view(move || view! { <BiographyPage content=c /> });
            (
                200,
                "Biography — Kennedy Mosoti".to_owned(),
                ScriptKind::None,
                html,
            )
        }
        "/archive" => {
            let c = content.clone();
            let html = render_view(move || view! { <ArchiveIndexPage content=c /> });
            (
                200,
                "Evidence Archive — Kennedy Mosoti".to_owned(),
                ScriptKind::None,
                html,
            )
        }
        _ if route.starts_with("/projects/") => {
            let slug = &route["/projects/".len()..];
            if let Some(proj) = content.projects.iter().find(|p| p.slug == slug).cloned() {
                let c = content.clone();
                let title = format!("{} — Moiric Project", proj.name);
                let html =
                    render_view(move || view! { <ProjectDetailPage content=c project=proj /> });
                (200, title, ScriptKind::Island, html)
            } else {
                not_found_page(route)
            }
        }
        _ if route.starts_with("/wiki/") => {
            let slug = &route["/wiki/".len()..];
            if let Some(art) = content.wiki.iter().find(|w| w.slug == slug).cloned() {
                let title = format!("{} — Wiki", art.title);
                let html = render_view(move || view! { <WikiArticlePage article=art /> });
                (200, title, ScriptKind::None, html)
            } else {
                not_found_page(route)
            }
        }
        _ if route.starts_with("/blog/") => {
            let slug = &route["/blog/".len()..];
            if let Some(post) = content.blog.iter().find(|b| b.slug == slug).cloned() {
                let title = format!("{} — Blog", post.title);
                let html = render_view(move || view! { <BlogPostPage post=post /> });
                (200, title, ScriptKind::None, html)
            } else {
                not_found_page(route)
            }
        }
        _ if route.starts_with("/archive/") => {
            let slug = &route["/archive/".len()..];
            if let Some(rec) = content.records.iter().find(|r| r.slug == slug).cloned() {
                let title = format!("{} — Archive", rec.title);
                let html = render_view(move || view! { <ArchiveRecordPage record=rec /> });
                (200, title, ScriptKind::None, html)
            } else {
                not_found_page(route)
            }
        }
        _ => not_found_page(route),
    };

    let extra_script = match script_kind {
        ScriptKind::Island => format!(
            "\n  <script type=\"module\" src=\"{}\" defer></script>",
            escape_attr(&assets.island_boot_src)
        ),
        ScriptKind::WikiSearch => format!(
            "\n  <script type=\"module\" src=\"{}\" defer></script>",
            escape_attr(&assets.wiki_search_src)
        ),
        ScriptKind::None => String::new(),
    };

    let doc = format!(
        "<!doctype html>\n\
<html lang=\"en\">\n\
<head>\n\
  <meta charset=\"utf-8\">\n\
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
  <meta name=\"color-scheme\" content=\"light dark\">\n\
  <meta name=\"description\" content=\"{desc}\">\n\
  <title>{title_esc}</title>\n\
  <link rel=\"stylesheet\" href=\"{css_href}\">\n\
  <link rel=\"manifest\" href=\"/manifest.webmanifest\">\n\
  <link rel=\"icon\" href=\"/icons/icon.svg\" type=\"image/svg+xml\">\n\
  <link rel=\"alternate\" type=\"application/atom+xml\" title=\"Kennedy Mosoti Blog\" href=\"/feed.xml\">{extra_script}\n\
</head>\n\
<body>\n\
{body_html}\n\
</body>\n\
</html>\n",
        desc = escape_attr(&content.identity.positioning),
        title_esc = escape_html(&title),
        css_href = escape_attr(&assets.css_href),
    );

    RenderedPage {
        status,
        title,
        html: doc,
    }
}

/// Extracts and percent-decodes a query parameter value from a URL query string.
pub fn extract_query_param(query: &str, key: &str) -> Option<String> {
    for pair in query.split('&') {
        let (k, v) = pair.split_once('=').unwrap_or((pair, ""));
        if k == key {
            let decoded = percent_decode(v);
            let trimmed = decoded.trim();
            if !trimmed.is_empty() {
                return Some(trimmed.to_owned());
            }
        }
    }
    None
}

fn percent_decode(input: &str) -> String {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        match bytes[i] {
            b'+' => {
                out.push(b' ');
                i += 1;
            }
            b'%' if i + 2 < bytes.len() => {
                if let (Some(hi), Some(lo)) = (hex_val(bytes[i + 1]), hex_val(bytes[i + 2])) {
                    out.push((hi << 4) | lo);
                    i += 3;
                } else {
                    out.push(b'%');
                    i += 1;
                }
            }
            b => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hex_val(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

#[derive(Clone, Copy)]
enum ScriptKind {
    None,
    Island,
    WikiSearch,
}

fn render_view<F, V>(f: F) -> String
where
    F: FnOnce() -> V,
    V: IntoView,
{
    let owner = Owner::new();
    owner.with(|| f().to_html())
}

fn not_found_page(route: &str) -> (u16, String, ScriptKind, String) {
    let route_s = route.to_owned();
    let html = render_view(move || {
        view! {
            <SiteShell section=NavSection::Home>
                <div class="page-container app-surface">
                    <h1>"404 — Route Not Found"</h1>
                    <p class="surface-lead">{format!("No published page exists at `{route_s}`.")}</p>
                    <div class="stage-toolbar">
                        <a class="btn primary" href="/">"Return to Cockpit"</a>
                        <a class="btn" href="/projects">"Projects"</a>
                        <a class="btn" href="/wiki">"Wiki"</a>
                        <a class="btn" href="/blog">"Blog"</a>
                    </div>
                </div>
            </SiteShell>
        }
    });
    (
        404,
        "Not Found — kennedy.mosoti.dev".to_owned(),
        ScriptKind::None,
        html,
    )
}
