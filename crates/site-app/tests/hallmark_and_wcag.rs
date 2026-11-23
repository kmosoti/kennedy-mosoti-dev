//! Hallmark CSS cascade, WCAG 2.2 AA OKLCH contrast, zero-slop, and SSR landmark tests.

use site_app::css::{COMPONENTS_CSS, LAYOUT_CSS, SITE_CSS, TOKENS_CSS, combined_stylesheet};
use site_app::router::{
    AssetManifest, QueryParamError, parse_bounded_query_param, render_route,
    render_wiki_search_page,
};
use site_content::{WikiSearchIndex, load_from_dir};
use std::collections::HashMap;
use std::path::PathBuf;

fn content_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../content")
}

fn parse_oklch(spec: &str) -> Option<(f64, f64, f64)> {
    let inner = spec.trim().strip_prefix("oklch(")?.strip_suffix(')')?;
    let base = inner.split('/').next()?.trim();
    let mut parts = base.split_whitespace();
    let l: f64 = parts.next()?.parse().ok()?;
    let c: f64 = parts.next()?.parse().ok()?;
    let h: f64 = parts.next()?.parse().ok()?;
    Some((l, c, h))
}

fn oklch_relative_luminance(l: f64, c: f64, h_deg: f64) -> f64 {
    let hr = h_deg.to_radians();
    let a = c * hr.cos();
    let b = c * hr.sin();

    let l_ = l + 0.396_337_777_4 * a + 0.215_803_757_3 * b;
    let m_ = l - 0.105_561_345_8 * a - 0.063_854_172_8 * b;
    let s_ = l - 0.089_484_177_5 * a - 1.291_485_548_0 * b;

    let l3 = l_.powi(3);
    let m3 = m_.powi(3);
    let s3 = s_.powi(3);

    let r = (4.076_741_662_1 * l3 - 3.307_711_591_3 * m3 + 0.230_969_929_2 * s3).clamp(0.0, 1.0);
    let g = (-1.268_438_004_6 * l3 + 2.609_757_401_1 * m3 - 0.341_319_396_5 * s3).clamp(0.0, 1.0);
    let b_ch =
        (-0.004_196_086_3 * l3 - 0.703_418_614_7 * m3 + 1.707_614_701_0 * s3).clamp(0.0, 1.0);

    0.2126 * r + 0.7152 * g + 0.0722 * b_ch
}

fn contrast_ratio(a: (f64, f64, f64), b: (f64, f64, f64)) -> f64 {
    let ya = oklch_relative_luminance(a.0, a.1, a.2);
    let yb = oklch_relative_luminance(b.0, b.1, b.2);
    let (hi, lo) = if ya >= yb { (ya, yb) } else { (yb, ya) };
    (hi + 0.05) / (lo + 0.05)
}

fn extract_oklch_tokens(css: &str) -> HashMap<String, (f64, f64, f64)> {
    let mut map = HashMap::new();
    for line in css.lines() {
        let trimmed = line.trim();
        if let Some((name, val)) = trimmed.split_once(':') {
            let name = name.trim();
            let val = val.trim().trim_end_matches(';');
            if name.starts_with("--")
                && let Some(ok) = parse_oklch(val)
            {
                map.insert(name.to_owned(), ok);
            }
        }
    }
    map
}

#[test]
fn wcag_22_aa_contrast_holds_for_atelier_light_and_oxide_dark() {
    let tokens = extract_oklch_tokens(TOKENS_CSS);
    let pairs = [
        ("--l-ink", "--l-canvas", 4.5),
        ("--l-ink", "--l-surface", 4.5),
        ("--l-ink-muted", "--l-canvas", 4.5),
        ("--l-ink-muted", "--l-surface", 4.5),
        ("--l-accent-ink", "--l-accent", 4.5),
        ("--l-status-ok", "--l-status-ok-bg", 4.5),
        ("--l-status-warn", "--l-status-warn-bg", 4.5),
        ("--l-status-info", "--l-status-info-bg", 4.5),
        ("--d-ink", "--d-canvas", 4.5),
        ("--d-ink", "--d-surface", 4.5),
        ("--d-ink-muted", "--d-canvas", 4.5),
        ("--d-ink-muted", "--d-surface", 4.5),
        ("--d-accent-ink", "--d-accent", 4.5),
        ("--d-status-ok", "--d-status-ok-bg", 4.5),
        ("--d-status-warn", "--d-status-warn-bg", 4.5),
        ("--d-status-info", "--d-status-info-bg", 4.5),
    ];

    for (fg, bg, min_ratio) in pairs {
        let f = tokens
            .get(fg)
            .unwrap_or_else(|| panic!("missing token {fg}"));
        let b = tokens
            .get(bg)
            .unwrap_or_else(|| panic!("missing token {bg}"));
        let ratio = contrast_ratio(*f, *b);
        assert!(
            ratio >= min_ratio,
            "WCAG contrast failure: {fg} on {bg} = {ratio:.2}:1 (required >= {min_ratio:.1}:1)"
        );
    }
}

#[test]
fn hallmark_css_cascade_disciplines_hold() {
    let full = combined_stylesheet();
    assert!(full.contains("@layer reset, tokens, base, layout, components, site, utilities;"));
    assert!(full.contains("@media (max-width: 64rem)"));
    assert!(full.contains("min-block-size: max(24px"));
    assert!(full.contains("font-variant-numeric: tabular-nums"));
    assert!(full.contains("@scope (.app-surface)"));
    assert!(full.contains("@media (prefers-reduced-motion: reduce)"));
    assert!(full.contains("@media (forced-colors: active)"));
    assert!(full.contains("@view-transition"));
    assert!(full.contains("navigation: auto"));
    assert!(full.contains("view-transition-name: cockpit-view-heading"));

    for (name, css) in [
        ("layout.css", LAYOUT_CSS),
        ("components.css", COMPONENTS_CSS),
        ("site.css", SITE_CSS),
    ] {
        assert!(
            !css.contains("!important"),
            "{name} must not use !important"
        );
        assert!(!css.contains('#'), "{name} must not hardcode hex colors");
        assert!(
            !css.contains("transition: all"),
            "{name} must not use transition: all"
        );
    }
}

#[test]
fn all_ssr_routes_render_zero_js_landmarks_without_emojis() {
    let site = load_from_dir(&content_dir()).expect("content loads");
    let routes = [
        "/",
        "/projects",
        "/projects/nomos",
        "/projects/fabrico11y",
        "/projects/metron",
        "/projects/jacquard",
        "/projects/hephaestus",
        "/wiki",
        "/wiki/moiric-architecture",
        "/blog",
        "/blog/replacing-threejs-with-pure-rust-cockpit",
        "/biography",
        "/archive",
        "/archive/repave-observer-not-waiting",
    ];

    for route in routes {
        let page = render_route(&site, route);
        assert_eq!(page.status, 200, "route {route} returned {}", page.status);
        assert_eq!(
            page.html.matches("<main").count(),
            1,
            "route {route} must have exactly 1 <main>"
        );
        for req in ["<header", "<nav", "<footer", "<h1", "skip-link", "viewport"] {
            assert!(
                page.html.contains(req),
                "route {route} missing required semantic markup `{req}`"
            );
        }
        for emoji in ["🟢", "🔴", "🟡", "⚠", "✅", "❌", "🚀", "🔥", "✨", "💎"] {
            assert!(
                !page.html.contains(emoji),
                "route {route} must not render emoji `{emoji}`"
            );
        }
    }

    let home = render_route(&site, "/");
    assert!(
        home.html.contains("id=\"iso-stage-svg\"") && home.html.contains("data-scene=\""),
        "home cockpit SSR must embed pre-compiled SceneGeometry data-scene"
    );
    assert!(
        home.html.contains("id=\"iso-node-canon-t\""),
        "home cockpit SSR must emit keyed SVG polygon ids for in-place patching"
    );

    let wiki_q = render_route(&site, "/wiki?q=moiric");
    assert_eq!(wiki_q.status, 200);
    assert!(
        wiki_q.html.contains("role=\"search\"") && wiki_q.html.contains("moiric-architecture"),
        "/wiki?q=moiric zero-JS fallback must render search form and ranked hit"
    );

    let index = WikiSearchIndex::build(&site.wiki);
    let prebuilt = render_wiki_search_page(
        &site.identity.positioning,
        &index,
        Some("moir"),
        &AssetManifest::default(),
    );
    assert_eq!(prebuilt.status, 200);
    assert!(prebuilt.html.contains("moiric-architecture"));

    assert_eq!(
        parse_bounded_query_param(&format!("q={}", "x".repeat(255)), "q"),
        Err(QueryParamError::RawQueryTooLong)
    );
    assert_eq!(
        parse_bounded_query_param(&format!("q={}{}", "%41".repeat(43), "x".repeat(86)), "q"),
        Err(QueryParamError::DecodedValueTooLong)
    );
}
