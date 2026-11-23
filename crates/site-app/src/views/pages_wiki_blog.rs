//! SSR views for Public Wiki (`/wiki`, `/wiki/{slug}`) and Engineering Blog (`/blog`, `/blog/{slug}`).

use crate::views::layout::{NavSection, SiteShell};
use leptos::prelude::*;
use site_content::{BlogPost, SiteContent, WikiArticle, WikiSearchHit};

/// Lightweight wiki summary row rendered on `/wiki` and `/wiki?q=...` without cloning article bodies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiCardItem {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub updated: String,
    pub summary: String,
    pub tags: Vec<String>,
}

impl From<&WikiArticle> for WikiCardItem {
    fn from(w: &WikiArticle) -> Self {
        Self {
            slug: w.slug.clone(),
            title: w.title.clone(),
            category: w.category.clone(),
            updated: w.updated.clone(),
            summary: w.summary.clone(),
            tags: w.tags.clone(),
        }
    }
}

impl From<&WikiSearchHit> for WikiCardItem {
    fn from(h: &WikiSearchHit) -> Self {
        Self {
            slug: h.slug.clone(),
            title: h.title.clone(),
            category: h.category.clone(),
            updated: h.updated.clone(),
            summary: h.summary.clone(),
            tags: h.tags.clone(),
        }
    }
}

/// Public Wiki index (`/wiki`, with zero-JS `?q=` precomputed BM25/trigram search results).
#[component]
pub fn WikiIndexPage(
    total_count: usize,
    visible_articles: Vec<WikiCardItem>,
    #[prop(default = None)] query: Option<String>,
) -> impl IntoView {
    let active_q = query.unwrap_or_default().trim().to_owned();

    let status_text = if active_q.is_empty() {
        format!("Showing all {total_count} wiki articles.")
    } else {
        format!(
            "{} result(s) for \"{}\" (BM25 + trigram).",
            visible_articles.len(),
            active_q
        )
    };
    let has_query = !active_q.is_empty();

    view! {
        <SiteShell section=NavSection::Wiki>
            <div class="page-container app-surface">
                <header>
                    <h1 class="view-heading">"Engineering Wiki"</h1>
                    <p class="surface-lead">
                        "Living technical notes on Moiric repository boundaries, claim discipline, pure-Rust 2.5D SVG architecture, and deterministic reconciliation. Served as full zero-JS SSR HTML with Mermaid diagram sources."
                    </p>
                </header>

                <form
                    class="wiki-search-form"
                    role="search"
                    method="get"
                    action="/wiki"
                    data-wiki-search-index="/wiki/search.json"
                >
                    <label class="sr-only" for="wiki-search-input">"Search engineering wiki"</label>
                    <input
                        id="wiki-search-input"
                        class="wiki-search-input"
                        type="search"
                        name="q"
                        value=active_q
                        placeholder="Search wiki articles (BM25 + trigram)..."
                        autocomplete="off"
                    />
                    <button type="submit" class="btn primary">"Search"</button>
                    {has_query.then(|| view! {
                        <a class="btn quiet" href="/wiki">"Clear"</a>
                    })}
                </form>

                <p id="wiki-search-status" class="brand-sub" aria-live="polite">{status_text}</p>

                <div id="wiki-card-grid" class="card-grid">
                    {if visible_articles.is_empty() {
                        view! {
                            <article class="panel-card">
                                <h2>"No matching wiki articles"</h2>
                                <p class="brand-sub">"Try broader terms such as moiric, cockpit, nomos, or evidence."</p>
                                <a class="btn" href="/wiki">"Show all articles"</a>
                            </article>
                        }.into_any()
                    } else {
                        visible_articles
                            .into_iter()
                            .map(|w| view! {
                                <article class="panel-card" data-slug=w.slug.clone()>
                                    <div class="card-header">
                                        <span class="kbd-hint">{w.category}</span>
                                        <span class="kbd-hint num">{w.updated}</span>
                                    </div>
                                    <h2><a href=format!("/wiki/{}", w.slug)>{w.title}</a></h2>
                                    <p class="brand-sub">{w.summary}</p>
                                    <div class="tag-row">
                                        {w.tags.into_iter().map(|t| view! { <span class="kbd-hint">{t}</span> }).collect_view()}
                                    </div>
                                </article>
                            })
                            .collect_view()
                            .into_any()
                    }}
                </div>
            </div>
        </SiteShell>
    }
}

/// Single Wiki article view (`/wiki/{slug}`).
#[component]
pub fn WikiArticlePage(article: WikiArticle) -> impl IntoView {
    view! {
        <SiteShell section=NavSection::Wiki>
            <div class="page-container app-surface">
                <nav aria-label="Breadcrumb">
                    <a class="btn quiet" href="/wiki">"← All Wiki Articles"</a>
                </nav>

                <div class="editorial-grid">
                    <article class="prose">
                        <div class="card-header">
                            <span class="kbd-hint">{article.category}</span>
                            <span class="kbd-hint num">{format!("Updated {}", article.updated)}</span>
                        </div>
                        <h1 class="view-heading">{article.title}</h1>
                        <p class="surface-lead">{article.summary}</p>
                        <div inner_html=article.body_html></div>
                    </article>

                    <aside class="toc-rail" aria-label="Article Outline">
                        <strong>"On this page"</strong>
                        <ul class="toc-list">
                            {article
                                .headings
                                .into_iter()
                                .map(|h| view! {
                                    <li>
                                        <a class="toc-link" href=format!("#{}", h.id)>{h.text}</a>
                                    </li>
                                })
                                .collect_view()}
                        </ul>
                        <div class="tag-row">
                            {article.tags.into_iter().map(|t| view! { <span class="kbd-hint">{t}</span> }).collect_view()}
                        </div>
                    </aside>
                </div>
            </div>
        </SiteShell>
    }
}

/// Public Engineering Blog index (`/blog`).
#[component]
pub fn BlogIndexPage(content: SiteContent) -> impl IntoView {
    view! {
        <SiteShell section=NavSection::Blog>
            <div class="page-container app-surface">
                <header class="card-header">
                    <div>
                        <h1 class="view-heading">"Engineering Blog"</h1>
                        <p class="surface-lead">
                            "Technical write-ups on systems reliability, Rust architecture, and honest evidence discipline. Subscribe via Atom/RSS at /feed.xml."
                        </p>
                    </div>
                    <a class="btn primary" href="/feed.xml">"Atom / RSS Feed"</a>
                </header>

                <div class="card-grid">
                    {content
                        .blog
                        .into_iter()
                        .map(|p| view! {
                            <article class="panel-card">
                                <div class="card-header">
                                    <span class="kbd-hint num">{p.date}</span>
                                    <span class="brand-sub">{p.author}</span>
                                </div>
                                <h2><a href=format!("/blog/{}", p.slug)>{p.title}</a></h2>
                                <p class="brand-sub">{p.summary}</p>
                                <div class="tag-row">
                                    {p.tags.into_iter().map(|t| view! { <span class="kbd-hint">{t}</span> }).collect_view()}
                                </div>
                            </article>
                        })
                        .collect_view()}
                </div>
            </div>
        </SiteShell>
    }
}

/// Single Blog post view (`/blog/{slug}`).
#[component]
pub fn BlogPostPage(post: BlogPost) -> impl IntoView {
    view! {
        <SiteShell section=NavSection::Blog>
            <div class="page-container app-surface">
                <nav aria-label="Breadcrumb" class="stage-toolbar">
                    <a class="btn quiet" href="/blog">"← All Posts"</a>
                    <a class="btn quiet" href="/feed.xml">"Atom Feed"</a>
                </nav>

                <div class="editorial-grid">
                    <article class="prose">
                        <div class="card-header">
                            <span class="kbd-hint num">{post.date}</span>
                            <span class="brand-sub">{post.author}</span>
                        </div>
                        <h1 class="view-heading">{post.title}</h1>
                        <p class="surface-lead">{post.summary}</p>
                        <div inner_html=post.body_html></div>
                    </article>

                    <aside class="toc-rail" aria-label="Post Outline">
                        <strong>"Sections"</strong>
                        <ul class="toc-list">
                            {post
                                .headings
                                .into_iter()
                                .map(|h| view! {
                                    <li>
                                        <a class="toc-link" href=format!("#{}", h.id)>{h.text}</a>
                                    </li>
                                })
                                .collect_view()}
                        </ul>
                        <div class="tag-row">
                            {post.tags.into_iter().map(|t| view! { <span class="kbd-hint">{t}</span> }).collect_view()}
                        </div>
                    </aside>
                </div>
            </div>
        </SiteShell>
    }
}
