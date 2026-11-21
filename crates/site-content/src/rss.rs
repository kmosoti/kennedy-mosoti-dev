//! Atom 1.0 and RSS 2.0 XML feed generation powered by Jacquard's `cockpit-wiki`.

use crate::markdown::escape_html;
use crate::model::SiteContent;
use cockpit_wiki::{Article, ArticleKind, ArticleMeta, SiteFeedConfig, render_atom, render_rss};
use std::fmt::Write;

/// Canonical site base URL used for feed entries and canonical links.
pub const SITE_BASE_URL: &str = "https://kennedy.mosoti.dev";

fn feed_config(content: &SiteContent) -> SiteFeedConfig {
    SiteFeedConfig {
        site_title: format!("{} — Engineering Blog", content.identity.name),
        site_url: SITE_BASE_URL.to_owned(),
        description: content.identity.positioning.clone(),
        author_name: content.identity.name.clone(),
        language: "en-us".to_owned(),
    }
}

fn to_wiki_articles(content: &SiteContent) -> Vec<Article> {
    content
        .blog
        .iter()
        .map(|post| Article {
            meta: ArticleMeta {
                slug: post.slug.clone(),
                title: post.title.clone(),
                summary: post.summary.clone(),
                kind: ArticleKind::Blog,
                published_at: post.date.clone(),
                updated_at: Some(post.date.clone()),
                author: post.author.clone(),
                tags: post.tags.clone(),
                evidence_class: None,
                draft: false,
            },
            source_path: format!("blog/{}.md", post.slug),
            route: format!("/blog/{}", post.slug),
            html: post.body_html.clone(),
            toc: Vec::new(),
            internal_links: post.internal_links.clone(),
            external_links: Vec::new(),
            has_mermaid: false,
            word_count: post.body_md.split_whitespace().count(),
            raw_markdown: post.body_md.clone(),
        })
        .collect()
}

/// Builds a deterministic Atom 1.0 feed (`/feed.xml` and `/atom.xml`).
pub fn build_atom_feed(content: &SiteContent) -> String {
    let articles = to_wiki_articles(content);
    let refs: Vec<&Article> = articles.iter().collect();
    render_atom(&refs, &feed_config(content), Some(ArticleKind::Blog))
}

/// Builds a deterministic RSS 2.0 feed (`/rss.xml`).
pub fn build_rss_feed(content: &SiteContent) -> String {
    let articles = to_wiki_articles(content);
    let refs: Vec<&Article> = articles.iter().collect();
    render_rss(&refs, &feed_config(content), Some(ArticleKind::Blog))
}

/// Builds a `sitemap.xml` listing all canonical SSR routes.
pub fn build_sitemap_xml(content: &SiteContent) -> String {
    let mut xml = String::with_capacity(4096);
    xml.push_str("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    xml.push_str("<urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">\n");
    for path in ["/", "/projects", "/wiki", "/blog", "/biography", "/archive"] {
        let _ = writeln!(xml, "  <url><loc>{SITE_BASE_URL}{path}</loc></url>");
    }
    for proj in &content.projects {
        let _ = writeln!(
            xml,
            "  <url><loc>{SITE_BASE_URL}/projects/{}</loc></url>",
            escape_html(&proj.slug)
        );
    }
    for art in &content.wiki {
        let _ = writeln!(
            xml,
            "  <url><loc>{SITE_BASE_URL}/wiki/{}</loc></url>",
            escape_html(&art.slug)
        );
    }
    for post in &content.blog {
        let _ = writeln!(
            xml,
            "  <url><loc>{SITE_BASE_URL}/blog/{}</loc></url>",
            escape_html(&post.slug)
        );
    }
    for rec in &content.records {
        let _ = writeln!(
            xml,
            "  <url><loc>{SITE_BASE_URL}/archive/{}</loc></url>",
            escape_html(&rec.slug)
        );
    }
    xml.push_str("</urlset>\n");
    xml
}
