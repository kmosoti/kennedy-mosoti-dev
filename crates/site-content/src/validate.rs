//! Claim-discipline, internal-link integrity, and strict privacy checks (`CONTRACT.md` §4–§6).

use crate::model::{EvidenceClass, RecordKind, SiteContent};
use crate::privacy::{contains_content_policy_or_privacy_leak, privacy_error};
use crate::rss::{build_atom_feed, build_rss_feed, build_sitemap_xml};
use crate::search::WikiSearchIndex;
use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Runs all Contract §4–§6 validation checks and returns all accumulated errors.
pub fn validate_all(content: &SiteContent) -> Result<(), Vec<String>> {
    let mut errors = Vec::new();
    check_identity(content, &mut errors);
    check_skills_and_records(content, &mut errors);
    check_projects(content, &mut errors);
    check_internal_links(content, &mut errors);
    check_privacy_and_nda(content, &mut errors);

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn safe_label(val: &str) -> &str {
    if contains_content_policy_or_privacy_leak(val) {
        "[redacted]"
    } else {
        val
    }
}

fn check_identity(content: &SiteContent, errors: &mut Vec<String>) {
    if content.identity.headline != "Platform, Reliability & Systems Automation Engineer" {
        errors.push(format!(
            "identity headline `{}` violates CONTRACT §5",
            safe_label(&content.identity.headline)
        ));
    }
    if content.identity.disciplines.len() != 4 {
        errors.push(format!(
            "expected 4 disciplines in identity.md, found {}",
            content.identity.disciplines.len()
        ));
    }
    if content.identity.operating_pattern.len() != 6 {
        errors.push(format!(
            "expected 6 operating_pattern stages, found {}",
            content.identity.operating_pattern.len()
        ));
    }
    for link in &content.identity.links {
        if !link.href.starts_with("https://") && !link.href.starts_with("mailto:") {
            errors.push(format!(
                "identity link `{}` must use https:// or mailto:",
                safe_label(&link.href)
            ));
        }
    }
}

fn check_skills_and_records(content: &SiteContent, errors: &mut Vec<String>) {
    let record_map: HashMap<&str, EvidenceClass> = content
        .records
        .iter()
        .map(|r| (r.slug.as_str(), r.evidence))
        .collect();

    for skill in &content.skills.skills {
        for rec_slug in &skill.records {
            match record_map.get(rec_slug.as_str()) {
                None => errors.push(format!(
                    "skill `{}` references unknown record `{}`",
                    safe_label(&skill.tag),
                    safe_label(rec_slug)
                )),
                Some(&ev) if ev != skill.evidence => errors.push(format!(
                    "skill `{}` ({:?}) cites record `{}` of mismatched class ({:?})",
                    safe_label(&skill.tag),
                    skill.evidence,
                    safe_label(rec_slug),
                    ev
                )),
                Some(_) => {}
            }
        }
    }

    for rec in &content.records {
        if rec.kind == RecordKind::CaseStudy || rec.summary.chars().any(|c| c.is_ascii_digit()) {
            let valid_boundary = rec
                .claim_boundary
                .as_deref()
                .is_some_and(|b| b.trim().len() > 20);
            if !valid_boundary {
                errors.push(format!(
                    "record `{}` lacks an adequate claim_boundary",
                    safe_label(&rec.slug)
                ));
            }
        }
        if matches!(
            rec.evidence,
            EvidenceClass::Production | EvidenceClass::OperationalExposure
        ) {
            for (field, text) in [
                ("title", rec.title.as_str()),
                ("summary", rec.summary.as_str()),
                (
                    "claim_boundary",
                    rec.claim_boundary.as_deref().unwrap_or(""),
                ),
                ("body", rec.body_md.as_str()),
            ] {
                if text.chars().any(|c| c.is_ascii_digit()) {
                    errors.push(format!(
                        "production/exposure record `{}` states an ASCII digit in {field}",
                        safe_label(&rec.slug)
                    ));
                }
            }
        }
    }
}

fn check_projects(content: &SiteContent, errors: &mut Vec<String>) {
    let expected = ["nomos", "fabrico11y", "metron", "jacquard", "hephaestus"];
    let found: HashSet<&str> = content.projects.iter().map(|p| p.slug.as_str()).collect();
    for slug in expected {
        if !found.contains(slug) {
            errors.push(format!("missing required Moiric project `{slug}`"));
        }
    }
    for proj in &content.projects {
        if proj.claim_boundary.trim().len() < 20 {
            errors.push(format!(
                "project `{}` must include a substantive claim_boundary",
                safe_label(&proj.slug)
            ));
        }
        if !proj.repo.starts_with("https://github.com/kmosoti/") {
            errors.push(format!(
                "project `{}` repo `{}` must target https://github.com/kmosoti/...",
                safe_label(&proj.slug),
                safe_label(&proj.repo)
            ));
        }
    }
}

fn check_internal_links(content: &SiteContent, errors: &mut Vec<String>) {
    let mut valid_routes: HashSet<String> = HashSet::from([
        "/".into(),
        "/projects".into(),
        "/wiki".into(),
        "/blog".into(),
        "/biography".into(),
        "/archive".into(),
        "/feed.xml".into(),
        "/atom.xml".into(),
        "/rss.xml".into(),
        "/sitemap.xml".into(),
        "/health".into(),
    ]);
    for p in &content.projects {
        valid_routes.insert(format!("/projects/{}", p.slug));
    }
    for w in &content.wiki {
        valid_routes.insert(format!("/wiki/{}", w.slug));
    }
    for b in &content.blog {
        valid_routes.insert(format!("/blog/{}", b.slug));
    }
    for r in &content.records {
        valid_routes.insert(format!("/archive/{}", r.slug));
    }

    let check_set = |source: &str, links: &[String], errs: &mut Vec<String>| {
        for link in links {
            let clean = link.split(['#', '?']).next().unwrap_or(link);
            if !valid_routes.contains(clean) {
                errs.push(format!(
                    "{}: broken internal link `{}`",
                    safe_label(source),
                    safe_label(link)
                ));
            }
        }
    };

    check_set("identity.md", &content.identity.internal_links, errors);
    for r in &content.records {
        check_set(&format!("records/{}", r.slug), &r.internal_links, errors);
    }
    for p in &content.projects {
        check_set(&format!("projects/{}", p.slug), &p.internal_links, errors);
    }
    for w in &content.wiki {
        check_set(&format!("wiki/{}", w.slug), &w.internal_links, errors);
    }
    for b in &content.blog {
        check_set(&format!("blog/{}", b.slug), &b.internal_links, errors);
    }
}

fn check_privacy_and_nda(content: &SiteContent, errors: &mut Vec<String>) {
    match serde_json::to_value(content) {
        Ok(val) => scan_json_value("content", &val, errors),
        Err(_) => errors.push(privacy_error("content")),
    }

    match serde_json::to_string(content) {
        Ok(canonical_json) => {
            if contains_content_policy_or_privacy_leak(&canonical_json) {
                errors.push(privacy_error("content_json"));
            }
        }
        Err(_) => errors.push(privacy_error("content_json")),
    }

    let atom = build_atom_feed(content);
    if contains_content_policy_or_privacy_leak(&atom) {
        errors.push(privacy_error("atom_feed"));
    }

    let rss = build_rss_feed(content);
    if contains_content_policy_or_privacy_leak(&rss) {
        errors.push(privacy_error("rss_feed"));
    }

    let sitemap = build_sitemap_xml(content);
    if contains_content_policy_or_privacy_leak(&sitemap) {
        errors.push(privacy_error("sitemap_xml"));
    }

    let search_json = WikiSearchIndex::build(&content.wiki).to_client_json();
    if contains_content_policy_or_privacy_leak(&search_json) {
        errors.push(privacy_error("wiki_search_json"));
    }
}

fn scan_json_value(loc: &str, val: &Value, errors: &mut Vec<String>) {
    match val {
        Value::String(text) => {
            if contains_content_policy_or_privacy_leak(text) {
                errors.push(privacy_error(loc));
            }
        }
        Value::Array(items) => {
            for (idx, item) in items.iter().enumerate() {
                scan_json_value(&format!("{loc}[{idx}]"), item, errors);
            }
        }
        Value::Object(map) => {
            for (key, item) in map {
                if contains_content_policy_or_privacy_leak(key) {
                    errors.push(privacy_error(&format!("{loc}.{key}")));
                }
                scan_json_value(&format!("{loc}.{key}"), item, errors);
            }
        }
        Value::Null | Value::Bool(_) | Value::Number(_) => {}
    }
}
