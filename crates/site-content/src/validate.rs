//! Claim-discipline, internal-link integrity, and strict privacy checks (`CONTRACT.md` §4–§6).

use crate::model::{EvidenceClass, RecordKind, SiteContent};
use std::collections::{HashMap, HashSet};

/// FNV-1a 64-bit word hash so the verifier enforces client-name exclusion
/// without storing the excluded literal strings in public source code.
fn fnv1a_64(word: &str) -> u64 {
    word.bytes().fold(0xcbf2_9ce4_8422_2325, |h, b| {
        (h ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3)
    })
}

const EXCLUDED_CLIENT_HASHES: [u64; 4] = [
    0x3d49_eeb5_c375_b7cb,
    0xa72f_7b76_4c09_639d,
    0x167b_30de_10cb_d437,
    0x4565_b6df_988d_6cb1,
];

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

fn check_identity(content: &SiteContent, errors: &mut Vec<String>) {
    if content.identity.headline != "Platform, Reliability & Systems Automation Engineer" {
        errors.push(format!(
            "identity headline `{}` violates CONTRACT §5",
            content.identity.headline
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
                link.href
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
                    "skill `{}` references unknown record `{rec_slug}`",
                    skill.tag
                )),
                Some(&ev) if ev != skill.evidence => errors.push(format!(
                    "skill `{}` ({:?}) cites record `{rec_slug}` of mismatched class ({:?})",
                    skill.tag, skill.evidence, ev
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
                    rec.slug
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
                        rec.slug
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
                proj.slug
            ));
        }
        if !proj.repo.starts_with("https://github.com/kmosoti/") {
            errors.push(format!(
                "project `{}` repo `{}` must target https://github.com/kmosoti/...",
                proj.slug, proj.repo
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
                errs.push(format!("{source}: broken internal link `{link}`"));
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
    let mut buckets: Vec<(String, String)> = vec![
        (
            "identity".into(),
            format!(
                "{} {} {}",
                content.identity.positioning, content.identity.bio, content.identity.body_md
            ),
        ),
        ("skills".into(), content.skills.body_md.clone()),
    ];
    for r in &content.records {
        buckets.push((
            format!("record/{}", r.slug),
            format!(
                "{} {} {} {}",
                r.title,
                r.summary,
                r.claim_boundary.as_deref().unwrap_or(""),
                r.body_md
            ),
        ));
    }
    for p in &content.projects {
        buckets.push((
            format!("project/{}", p.slug),
            format!(
                "{} {} {} {}",
                p.name, p.summary, p.claim_boundary, p.body_md
            ),
        ));
    }
    for w in &content.wiki {
        buckets.push((
            format!("wiki/{}", w.slug),
            format!("{} {} {}", w.title, w.summary, w.body_md),
        ));
    }
    for b in &content.blog {
        buckets.push((
            format!("blog/{}", b.slug),
            format!("{} {} {}", b.title, b.summary, b.body_md),
        ));
    }

    for (loc, raw) in buckets {
        let lower = raw.to_lowercase();
        for word in lower.split(|c: char| !c.is_alphanumeric()) {
            if !word.is_empty() && EXCLUDED_CLIENT_HASHES.contains(&fnv1a_64(word)) {
                errors.push(format!(
                    "{loc}: matched excluded client hash token `{word}`"
                ));
            }
        }
        let banned_phrases = [
            "compensation",
            "salary",
            "w-2",
            "interview",
            "story bank",
            "career assessment",
            "splunk engineer",
            "senior sre",
            concat!("2600", ":", "1900"),
            concat!("fd20", ":5fa:", "d190"),
            concat!("green-infra", "-deployer"),
            concat!("fedora", "-blade"),
            concat!("age-secret", "-key-"),
            concat!("begin openssh ", "private key"),
            concat!("/home/", "kmosoti"),
        ];
        for banned in banned_phrases {
            if lower.contains(banned) {
                errors.push(format!(
                    "{loc}: matched banned privacy/positioning phrase `{banned}`"
                ));
            }
        }
    }
}
