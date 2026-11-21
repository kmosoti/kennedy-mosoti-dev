use site_content::{
    WikiSearchIndex, build_atom_feed, build_rss_feed, build_sitemap_xml, load_from_dir,
    markdown::render_markdown, validate_all,
};
use std::path::PathBuf;

fn content_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content")
}

#[test]
fn live_content_loads_and_passes_all_contract_checks() {
    let site = load_from_dir(&content_dir()).expect("content directory loads cleanly");
    assert_eq!(site.projects.len(), 5, "expected 5 Moiric project pages");
    assert_eq!(site.records.len(), 20, "expected 20 migrated records");
    assert_eq!(site.wiki.len(), 4, "expected 4 seed wiki articles");
    assert_eq!(site.blog.len(), 3, "expected 3 seed blog posts");
    assert!(!site.skills.skills.is_empty(), "expected skills entries");

    if let Err(errs) = validate_all(&site) {
        panic!("content validation failed:\n{}", errs.join("\n"));
    }
}

#[test]
fn atom_feed_and_sitemap_include_all_posts_and_projects() {
    let site = load_from_dir(&content_dir()).expect("content directory loads");
    let feed = build_atom_feed(&site);
    assert!(feed.starts_with("<?xml version=\"1.0\""));
    assert!(feed.contains("<feed xmlns=\"http://www.w3.org/2005/Atom\">"));
    for post in &site.blog {
        assert!(feed.contains(&post.slug), "feed missing post {}", post.slug);
    }

    let rss = build_rss_feed(&site);
    assert!(rss.contains("<rss version=\"2.0\">"));
    for post in &site.blog {
        assert!(rss.contains(&post.slug), "rss missing post {}", post.slug);
    }

    let sitemap = build_sitemap_xml(&site);
    assert!(sitemap.contains("https://kennedy.mosoti.dev/projects/nomos"));
    assert!(sitemap.contains("https://kennedy.mosoti.dev/wiki/moiric-architecture"));
    assert!(sitemap.contains("https://kennedy.mosoti.dev/archive/repave-observer-not-waiting"));
}

#[test]
fn markdown_renders_mermaid_safely_and_escapes_raw_html() {
    let md = "## Overview\n\n```mermaid\nflowchart LR\n  A --> B\n```\n\n<script>alert(1)</script>\n\n[Nomos](/projects/nomos)";
    let rendered = render_markdown(md);
    assert!(rendered.html.contains("id=\"overview\""));
    assert!(
        rendered.html.contains("data-lang=\"mermaid\"") && rendered.html.contains("<svg"),
        "expected native iso-diagram SVG for mermaid flowchart"
    );
    assert!(!rendered.html.contains("<script>"));
    assert_eq!(rendered.internal_links, vec!["/projects/nomos"]);
}

#[test]
fn wiki_search_index_ranks_bm25_and_trigram_fallback_queries() {
    let site = load_from_dir(&content_dir()).expect("content loads");
    let index = WikiSearchIndex::build(&site.wiki);
    assert_eq!(index.doc_count(), site.wiki.len());

    let bm25_hits = index.search("moiric", 5);
    assert!(
        !bm25_hits.is_empty(),
        "BM25 query 'moiric' must return wiki hits"
    );
    assert!(
        bm25_hits
            .iter()
            .any(|h| h.slug == "moiric-architecture" && h.score > 0.0),
        "expected moiric-architecture in BM25 hits: {bm25_hits:?}"
    );

    let trigram_hits = index.search("moir", 5);
    assert!(
        !trigram_hits.is_empty(),
        "trigram prefix query 'moir' must return fuzzy fallback hits"
    );

    let json = index.to_client_json();
    assert!(json.contains("\"moiric-architecture\""));
}

#[test]
fn validate_rejects_broken_link_and_production_digit_negative_controls() {
    let mut site = load_from_dir(&content_dir()).expect("content loads");
    site.wiki[0]
        .internal_links
        .push("/nonexistent-route".into());
    site.records[0].body_md.push_str("\n42 hosts");
    let errs = validate_all(&site).expect_err("negative controls must fail");
    assert!(
        errs.iter().any(|e| e.contains("broken internal link")),
        "expected broken internal link error, got {errs:?}"
    );
}
