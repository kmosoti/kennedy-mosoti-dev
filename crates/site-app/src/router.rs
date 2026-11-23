//! SSR route resolver producing complete standalone HTML documents without JS requirements.

pub use crate::query::{
    MAX_DECODED_QUERY_BYTES, MAX_QUERY_TERMS, MAX_RAW_QUERY_BYTES, QueryParamError,
    extract_query_param, parse_bounded_query_param,
};
use crate::shell_doc::{
    ScriptKind, bad_request_page, not_found_page, render_view, wrap_html_document,
};
use crate::views::pages_bio_archive::{ArchiveIndexPage, ArchiveRecordPage, BiographyPage};
use crate::views::pages_projects::{HomePage, ProjectDetailPage, ProjectsIndexPage};
use crate::views::pages_wiki_blog::{
    BlogIndexPage, BlogPostPage, WikiArticlePage, WikiCardItem, WikiIndexPage,
};
use leptos::prelude::*;
use site_content::{SiteContent, WikiSearchIndex};

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

/// Renders `/wiki` or `/wiki?q=...` using an existing startup-built [`WikiSearchIndex`],
/// avoiding per-request index reconstruction or full-content cloning.
pub fn render_wiki_search_page(
    positioning: &str,
    wiki_search: &WikiSearchIndex,
    query: Option<&str>,
    assets: &AssetManifest,
) -> RenderedPage {
    let total_count = wiki_search.doc_count();
    let active_q = query
        .map(str::trim)
        .filter(|q| !q.is_empty())
        .map(str::to_owned);

    let visible_articles: Vec<WikiCardItem> = match &active_q {
        None => wiki_search
            .articles()
            .iter()
            .map(WikiCardItem::from)
            .collect(),
        Some(q) => wiki_search
            .search(q, total_count)
            .iter()
            .map(WikiCardItem::from)
            .collect(),
    };

    let body_html = render_view(move || {
        view! {
            <WikiIndexPage
                total_count=total_count
                visible_articles=visible_articles
                query=active_q
            />
        }
    });
    let title = "Engineering Wiki — Kennedy Mosoti".to_owned();
    let html = wrap_html_document(
        positioning,
        &title,
        ScriptKind::WikiSearch,
        assets,
        &body_html,
    );
    RenderedPage {
        status: 200,
        title,
        html,
    }
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
            let q_res = match query_raw {
                Some(qs) => parse_bounded_query_param(qs, "q"),
                None => Ok(None),
            };
            match q_res {
                Err(_) => bad_request_page("Invalid wiki search query."),
                Ok(q) => {
                    let total_count = content.wiki.len();
                    let visible_articles: Vec<WikiCardItem> = match &q {
                        None => content.wiki.iter().map(WikiCardItem::from).collect(),
                        Some(term_str) => filter_wiki_cards_lightweight(&content.wiki, term_str),
                    };
                    let html = render_view(move || {
                        view! {
                            <WikiIndexPage
                                total_count=total_count
                                visible_articles=visible_articles
                                query=q
                            />
                        }
                    });
                    (
                        200,
                        "Engineering Wiki — Kennedy Mosoti".to_owned(),
                        ScriptKind::WikiSearch,
                        html,
                    )
                }
            }
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

    let html = wrap_html_document(
        &content.identity.positioning,
        &title,
        script_kind,
        assets,
        &body_html,
    );
    RenderedPage {
        status,
        title,
        html,
    }
}

fn filter_wiki_cards_lightweight(
    articles: &[site_content::WikiArticle],
    query: &str,
) -> Vec<WikiCardItem> {
    let terms: Vec<String> = query
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| !t.is_empty())
        .map(str::to_ascii_lowercase)
        .collect();
    if terms.is_empty() {
        return articles.iter().map(WikiCardItem::from).collect();
    }
    articles
        .iter()
        .filter(|w| {
            let hay = format!(
                "{} {} {} {} {}",
                w.slug,
                w.title,
                w.category,
                w.tags.join(" "),
                w.summary
            )
            .to_ascii_lowercase();
            terms.iter().all(|t| hay.contains(t.as_str()))
        })
        .map(WikiCardItem::from)
        .collect()
}
