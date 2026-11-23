//! Table-driven field-coverage tests proving `validate_all` scans EVERY public field
//! of `SiteContent` (via canonical serialization + feeds + search JSON), detects structural
//! secrets/paths/IPs, and never echoes matched prohibited text in error messages.

use site_content::model::{Discipline, HeadingOutline, OperatingStage, ProfileLink, SiteContent};
use site_content::privacy::contains_operational_or_secret_leak;
use site_content::{load_from_dir, validate_all};
use std::path::PathBuf;

fn content_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content")
}

#[test]
fn table_driven_negative_privacy_tests_cover_every_public_struct_field() {
    let base = load_from_dir(&content_dir()).expect("content loads cleanly");
    let trigger = ["com", "pensation"].join("");
    let trigger_phrase = ["senior", "sre"].join(" ");

    type FieldMutator = (&'static str, fn(&mut SiteContent, &str));
    let mutators: Vec<FieldMutator> = vec![
        // Identity.*
        ("content.identity.name", |c, v| c.identity.name = v.into()),
        ("content.identity.headline", |c, v| {
            c.identity.headline = v.into()
        }),
        ("content.identity.positioning", |c, v| {
            c.identity.positioning = v.into()
        }),
        ("content.identity.bio", |c, v| c.identity.bio = v.into()),
        ("content.identity.location", |c, v| {
            c.identity.location = v.into()
        }),
        ("content.identity.links", |c, v| {
            c.identity.links.push(ProfileLink {
                label: v.into(),
                href: "https://example.com".into(),
                kind: "web".into(),
            })
        }),
        ("content.identity.links", |c, v| {
            c.identity.links.push(ProfileLink {
                label: "ext".into(),
                href: format!("https://example.com/{v}"),
                kind: "web".into(),
            })
        }),
        ("content.identity.links", |c, v| {
            c.identity.links.push(ProfileLink {
                label: "ext".into(),
                href: "https://example.com".into(),
                kind: v.into(),
            })
        }),
        ("content.identity.disciplines", |c, v| {
            c.identity.disciplines[0].id = v.into()
        }),
        ("content.identity.disciplines", |c, v| {
            c.identity.disciplines[0].title = v.into()
        }),
        ("content.identity.disciplines", |c, v| {
            c.identity.disciplines[0].summary = v.into()
        }),
        ("content.identity.disciplines", |c, v| {
            c.identity.disciplines.push(Discipline {
                id: "d".into(),
                title: "t".into(),
                summary: "s".into(),
                bays: vec![v.into()],
            })
        }),
        ("content.identity.operating_pattern", |c, v| {
            c.identity.operating_pattern[0].stage = v.into()
        }),
        ("content.identity.operating_pattern", |c, v| {
            c.identity.operating_pattern.push(OperatingStage {
                stage: "s".into(),
                line: v.into(),
            })
        }),
        ("content.identity.body_md", |c, v| {
            c.identity.body_md = v.into()
        }),
        ("content.identity.body_html", |c, v| {
            c.identity.body_html = v.into()
        }),
        ("content.identity.internal_links", |c, v| {
            c.identity.internal_links.push(format!("/{v}"))
        }),
        // Skills.*
        ("content.skills.skills", |c, v| {
            c.skills.skills[0].tag = v.into()
        }),
        ("content.skills.skills", |c, v| {
            c.skills.skills[0].label = v.into()
        }),
        ("content.skills.skills", |c, v| {
            c.skills.skills[0].records.push(v.into())
        }),
        ("content.skills.skills", |c, v| {
            c.skills.skills[0].boundary = Some(v.into())
        }),
        ("content.skills.body_md", |c, v| c.skills.body_md = v.into()),
        ("content.skills.body_html", |c, v| {
            c.skills.body_html = v.into()
        }),
        // Records.*
        ("content.records[0].slug", |c, v| {
            c.records[0].slug = v.into()
        }),
        ("content.records[0].title", |c, v| {
            c.records[0].title = v.into()
        }),
        ("content.records[0].machines", |c, v| {
            c.records[0].machines.push(v.into())
        }),
        ("content.records[0].tags", |c, v| {
            c.records[0].tags.push(v.into())
        }),
        ("content.records[0].period", |c, v| {
            c.records[0].period = Some(v.into())
        }),
        ("content.records[0].summary", |c, v| {
            c.records[0].summary = v.into()
        }),
        ("content.records[0].claim_boundary", |c, v| {
            c.records[0].claim_boundary = Some(v.into())
        }),
        ("content.records[0].links", |c, v| {
            c.records[0].links.push(ProfileLink {
                label: v.into(),
                href: "https://example.com".into(),
                kind: "ref".into(),
            })
        }),
        ("content.records[0].body_md", |c, v| {
            c.records[0].body_md = v.into()
        }),
        ("content.records[0].body_html", |c, v| {
            c.records[0].body_html = v.into()
        }),
        ("content.records[0].internal_links", |c, v| {
            c.records[0].internal_links.push(format!("/{v}"))
        }),
        // Projects.*
        ("content.projects[0].slug", |c, v| {
            c.projects[0].slug = v.into()
        }),
        ("content.projects[0].name", |c, v| {
            c.projects[0].name = v.into()
        }),
        ("content.projects[0].tagline", |c, v| {
            c.projects[0].tagline = v.into()
        }),
        ("content.projects[0].version", |c, v| {
            c.projects[0].version = v.into()
        }),
        ("content.projects[0].repo", |c, v| {
            c.projects[0].repo = format!("https://github.com/kmosoti/{v}")
        }),
        ("content.projects[0].bay_id", |c, v| {
            c.projects[0].bay_id = v.into()
        }),
        ("content.projects[0].role", |c, v| {
            c.projects[0].role = v.into()
        }),
        ("content.projects[0].summary", |c, v| {
            c.projects[0].summary = v.into()
        }),
        ("content.projects[0].claim_boundary", |c, v| {
            c.projects[0].claim_boundary = v.into()
        }),
        ("content.projects[0].highlights", |c, v| {
            c.projects[0].highlights.push(v.into())
        }),
        ("content.projects[0].body_md", |c, v| {
            c.projects[0].body_md = v.into()
        }),
        ("content.projects[0].body_html", |c, v| {
            c.projects[0].body_html = v.into()
        }),
        ("content.projects[0].internal_links", |c, v| {
            c.projects[0].internal_links.push(format!("/{v}"))
        }),
        // Wiki.* (including category, updated, tags, slug)
        ("content.wiki[0].slug", |c, v| c.wiki[0].slug = v.into()),
        ("content.wiki[0].title", |c, v| c.wiki[0].title = v.into()),
        ("content.wiki[0].category", |c, v| {
            c.wiki[0].category = v.into()
        }),
        ("content.wiki[0].updated", |c, v| {
            c.wiki[0].updated = v.into()
        }),
        ("content.wiki[0].summary", |c, v| {
            c.wiki[0].summary = v.into()
        }),
        ("content.wiki[0].tags", |c, v| c.wiki[0].tags.push(v.into())),
        ("content.wiki[0].body_md", |c, v| {
            c.wiki[0].body_md = v.into()
        }),
        ("content.wiki[0].body_html", |c, v| {
            c.wiki[0].body_html = v.into()
        }),
        ("content.wiki[0].headings", |c, v| {
            c.wiki[0].headings.push(HeadingOutline {
                level: 2,
                id: v.into(),
                text: "heading".into(),
            })
        }),
        ("content.wiki[0].headings", |c, v| {
            c.wiki[0].headings.push(HeadingOutline {
                level: 2,
                id: "id".into(),
                text: v.into(),
            })
        }),
        ("content.wiki[0].internal_links", |c, v| {
            c.wiki[0].internal_links.push(format!("/{v}"))
        }),
        // Blog.* (including author, date, tags, slug)
        ("content.blog[0].slug", |c, v| c.blog[0].slug = v.into()),
        ("content.blog[0].title", |c, v| c.blog[0].title = v.into()),
        ("content.blog[0].date", |c, v| c.blog[0].date = v.into()),
        ("content.blog[0].author", |c, v| c.blog[0].author = v.into()),
        ("content.blog[0].summary", |c, v| {
            c.blog[0].summary = v.into()
        }),
        ("content.blog[0].tags", |c, v| c.blog[0].tags.push(v.into())),
        ("content.blog[0].body_md", |c, v| {
            c.blog[0].body_md = v.into()
        }),
        ("content.blog[0].body_html", |c, v| {
            c.blog[0].body_html = v.into()
        }),
        ("content.blog[0].headings", |c, v| {
            c.blog[0].headings.push(HeadingOutline {
                level: 2,
                id: v.into(),
                text: "heading".into(),
            })
        }),
        ("content.blog[0].internal_links", |c, v| {
            c.blog[0].internal_links.push(format!("/{v}"))
        }),
    ];

    for (idx, (expected_prefix, mutate)) in mutators.iter().enumerate() {
        let payload = if idx % 2 == 0 {
            &trigger
        } else {
            &trigger_phrase
        };
        let mut mutated = base.clone();
        mutate(&mut mutated, payload);
        let errs = validate_all(&mutated)
            .expect_err("privacy gate must reject prohibited input in every public field");
        assert!(
            errs.iter().any(|e| e.starts_with(expected_prefix)
                && e.ends_with(": matched prohibited privacy rule")),
            "missing expected privacy rule error for {expected_prefix}; got {errs:?}"
        );
        for err in &errs {
            assert!(
                !err.contains(payload),
                "privacy error `{err}` must never echo matched string"
            );
        }
    }
}

#[test]
fn generic_structural_detectors_catch_paths_private_ips_ula_and_key_headers() {
    let synthetic_home = ["", "home", "example_user"].join("/");
    let synthetic_ipv4_10 = [10, 42, 0, 7].map(|n| n.to_string()).join(".");
    let synthetic_ipv4_172 = [172, 20, 4, 9].map(|n| n.to_string()).join(".");
    let synthetic_ipv4_192 = [192, 168, 12, 34].map(|n| n.to_string()).join(".");
    let synthetic_ula = format!("{prefix}12:3456:789a::1", prefix = "fd");
    let synthetic_age_sec = format!("{2}-{1}-{0}-1EXAMPLE", "KEY", "SECRET", "AGE");
    let synthetic_age_pub = format!("age1{}", "q".repeat(58));
    let synthetic_pem = format!("-----{2} RSA {1} {0}-----", "KEY", "PRIVATE", "BEGIN");
    let synthetic_ssh_pub = format!("ssh-ed25519 {}", "A".repeat(32));

    for sample in [
        synthetic_home,
        synthetic_ipv4_10,
        synthetic_ipv4_172,
        synthetic_ipv4_192,
        synthetic_ula,
        synthetic_age_sec,
        synthetic_age_pub,
        synthetic_pem,
        synthetic_ssh_pub,
    ] {
        assert!(
            contains_operational_or_secret_leak(&sample),
            "structural detector failed to flag synthetic sample"
        );
    }

    for benign in [
        "127.0.0.1:3000",
        "0.1.1-alpha.1",
        "Rust 1.100.0 edition 2024",
        "https://github.com/kmosoti/kennedy-mosoti-dev",
    ] {
        assert!(
            !contains_operational_or_secret_leak(benign),
            "false positive on benign string `{benign}`"
        );
    }
}
