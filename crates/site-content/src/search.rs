//! BM25 + trigram search index over public Wiki articles powered by `cockpit-wiki::search`.

use crate::model::WikiArticle;
use cockpit_wiki::model::{Article, ArticleKind, ArticleMeta};
use cockpit_wiki::search::{SearchIndex, tokenize};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// Ranked wiki search result combining Okapi BM25 and trigram fuzzy fallback.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WikiSearchHit {
    pub slug: String,
    pub route: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub updated: String,
    pub tags: Vec<String>,
    pub score: f64,
}

/// Compact client-side index entry serialized to `/wiki/search.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClientSearchEntry {
    pub slug: String,
    pub route: String,
    pub title: String,
    pub summary: String,
    pub category: String,
    pub updated: String,
    pub tags: Vec<String>,
    pub terms: Vec<String>,
}

/// Deterministic BM25 + trigram search index built from [`WikiArticle`] entries.
#[derive(Debug, Clone, Default)]
pub struct WikiSearchIndex {
    bm25: SearchIndex,
    articles: Vec<WikiArticle>,
    route_to_idx: BTreeMap<String, usize>,
    doc_trigrams: Vec<BTreeSet<String>>,
}

impl WikiSearchIndex {
    /// Builds a deterministic BM25 + trigram index over `articles`.
    #[must_use]
    pub fn build(articles: &[WikiArticle]) -> Self {
        let mut wiki_docs = Vec::with_capacity(articles.len());
        let mut route_to_idx = BTreeMap::new();
        let mut doc_trigrams = Vec::with_capacity(articles.len());

        for (idx, w) in articles.iter().enumerate() {
            let route = format!("/wiki/{}", w.slug);
            route_to_idx.insert(route.clone(), idx);

            let combined = format!(
                "{} {} {} {} {}",
                w.title,
                w.category,
                w.tags.join(" "),
                w.summary,
                w.body_md
            );
            doc_trigrams.push(extract_trigrams(&combined));

            wiki_docs.push(Article {
                meta: ArticleMeta {
                    slug: w.slug.clone(),
                    title: w.title.clone(),
                    summary: w.summary.clone(),
                    kind: ArticleKind::Wiki,
                    published_at: w.updated.clone(),
                    updated_at: Some(w.updated.clone()),
                    author: String::new(),
                    tags: w.tags.clone(),
                    evidence_class: None,
                    draft: false,
                },
                source_path: format!("wiki/{}.md", w.slug),
                route,
                html: w.body_html.clone(),
                toc: Vec::new(),
                internal_links: w.internal_links.clone(),
                external_links: Vec::new(),
                has_mermaid: false,
                word_count: w.body_md.split_whitespace().count(),
                raw_markdown: w.body_md.clone(),
            });
        }

        Self {
            bm25: SearchIndex::build(&wiki_docs),
            articles: articles.to_vec(),
            route_to_idx,
            doc_trigrams,
        }
    }

    /// Number of indexed wiki articles.
    #[must_use]
    pub fn doc_count(&self) -> usize {
        self.bm25.doc_count()
    }

    /// Queries the index using `cockpit-wiki` Okapi BM25 first, then trigram overlap fallback
    /// for partial/prefix queries (e.g. `"nom"`, `"iso"`, `"merm"`).
    #[must_use]
    pub fn search(&self, query: &str, limit: usize) -> Vec<WikiSearchHit> {
        let trimmed = query.trim();
        if trimmed.is_empty() || limit == 0 || self.articles.is_empty() {
            return Vec::new();
        }

        let mut scores: BTreeMap<usize, f64> = BTreeMap::new();
        for hit in self.bm25.search(trimmed, self.articles.len()) {
            if let Some(&idx) = self.route_to_idx.get(&hit.route) {
                scores.insert(idx, hit.score + 1.0);
            }
        }

        let q_trigrams = extract_trigrams(trimmed);
        if !q_trigrams.is_empty() {
            let q_len = q_trigrams.len() as f64;
            for (idx, d_tris) in self.doc_trigrams.iter().enumerate() {
                if scores.contains_key(&idx) {
                    continue;
                }
                let matches = q_trigrams.iter().filter(|t| d_tris.contains(*t)).count();
                if matches > 0 {
                    let ratio = (matches as f64) / q_len;
                    if ratio >= 0.34 {
                        scores.insert(idx, ratio * 0.85);
                    }
                }
            }
        }

        let mut ranked: Vec<(usize, f64)> = scores.into_iter().collect();
        ranked.sort_by(|&(ia, sa), &(ib, sb)| {
            sb.total_cmp(&sa)
                .then_with(|| self.articles[ia].slug.cmp(&self.articles[ib].slug))
        });
        ranked.truncate(limit);

        ranked
            .into_iter()
            .map(|(idx, score)| {
                let w = &self.articles[idx];
                WikiSearchHit {
                    slug: w.slug.clone(),
                    route: format!("/wiki/{}", w.slug),
                    title: w.title.clone(),
                    summary: w.summary.clone(),
                    category: w.category.clone(),
                    updated: w.updated.clone(),
                    tags: w.tags.clone(),
                    score: (score * 1000.0).round() / 1000.0,
                }
            })
            .collect()
    }

    /// Exports a compact JSON index payload for client-side progressive search (`/wiki/search.json`).
    #[must_use]
    pub fn to_client_json(&self) -> String {
        let entries: Vec<ClientSearchEntry> = self
            .articles
            .iter()
            .map(|w| {
                let mut terms: BTreeSet<String> = BTreeSet::new();
                for src in [&w.title, &w.category, &w.summary, &w.body_md] {
                    for tok in tokenize(src) {
                        terms.insert(tok);
                    }
                }
                for tag in &w.tags {
                    for tok in tokenize(tag) {
                        terms.insert(tok);
                    }
                }
                ClientSearchEntry {
                    slug: w.slug.clone(),
                    route: format!("/wiki/{}", w.slug),
                    title: w.title.clone(),
                    summary: w.summary.clone(),
                    category: w.category.clone(),
                    updated: w.updated.clone(),
                    tags: w.tags.clone(),
                    terms: terms.into_iter().collect(),
                }
            })
            .collect();
        serde_json::to_string(&entries).unwrap_or_else(|_| "[]".to_owned())
    }
}

fn extract_trigrams(input: &str) -> BTreeSet<String> {
    let mut tris = BTreeSet::new();
    let lower: String = input
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                ' '
            }
        })
        .collect();
    for word in lower.split_whitespace() {
        let bytes = word.as_bytes();
        if bytes.len() < 3 {
            continue;
        }
        for win in bytes.windows(3) {
            if let Ok(tri) = std::str::from_utf8(win) {
                tris.insert(tri.to_owned());
            }
        }
    }
    tris
}
