//! Paired geometric SVG status badges and brand marks (zero emojis).

use leptos::prelude::*;

/// Isometric cube brand mark for the top navigation bar.
#[component]
pub fn BrandCubeMark() -> impl IntoView {
    view! {
        <svg class="status-glyph" viewBox="0 0 16 16" width="16" height="16" aria-hidden="true">
            <polygon points="8,1 14,4.5 14,11.5 8,15 2,11.5 2,4.5" fill="none" stroke="currentColor" stroke-width="1.4" />
            <polyline points="2,4.5 8,8 14,4.5" fill="none" stroke="currentColor" stroke-width="1.2" />
            <line x1="8" y1="8" x2="8" y2="15" stroke="currentColor" stroke-width="1.2" />
        </svg>
    }
}

/// Paired geometric glyph + text status pill (WCAG 1.4.1 non-color-only indicator).
#[component]
pub fn StatusPill(#[prop(into)] label: String, #[prop(into)] tone: String) -> impl IntoView {
    let glyph = match tone.as_str() {
        "ok" => view! {
            <svg class="status-glyph" viewBox="0 0 12 12" width="10" height="10" aria-hidden="true">
                <circle cx="6" cy="6" r="4.5" fill="currentColor" />
            </svg>
        }
        .into_any(),
        "info" => view! {
            <svg class="status-glyph" viewBox="0 0 12 12" width="10" height="10" aria-hidden="true">
                <polygon points="6,1.2 10.8,6 6,10.8 1.2,6" fill="currentColor" />
            </svg>
        }
        .into_any(),
        "warn" => view! {
            <svg class="status-glyph" viewBox="0 0 12 12" width="10" height="10" aria-hidden="true">
                <polygon points="6,1.5 11,10.2 1,10.2" fill="currentColor" />
            </svg>
        }
        .into_any(),
        _ => view! {
            <svg class="status-glyph" viewBox="0 0 12 12" width="10" height="10" aria-hidden="true">
                <rect x="2" y="2" width="8" height="8" rx="1.5" fill="currentColor" />
            </svg>
        }
        .into_any(),
    };

    view! {
        <span class="status-pill" data-tone=tone>
            {glyph}
            <span>{label}</span>
        </span>
    }
}
