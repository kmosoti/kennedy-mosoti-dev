//! HTML document shell wrapper, Leptos SSR owner runner, and error view helpers.

use crate::router::AssetManifest;
use crate::views::layout::{NavSection, SiteShell};
use leptos::prelude::*;
use site_content::markdown::{escape_attr, escape_html};

#[derive(Clone, Copy)]
pub(crate) enum ScriptKind {
    None,
    Island,
    WikiSearch,
}

pub(crate) fn render_view<F, V>(f: F) -> String
where
    F: FnOnce() -> V,
    V: IntoView,
{
    let owner = Owner::new();
    owner.with(|| f().to_html())
}

pub(crate) fn wrap_html_document(
    positioning: &str,
    title: &str,
    script_kind: ScriptKind,
    assets: &AssetManifest,
    body_html: &str,
) -> String {
    let extra_script = match script_kind {
        ScriptKind::Island => format!(
            "\n  <script type=\"module\" src=\"{}\" defer></script>",
            escape_attr(&assets.island_boot_src)
        ),
        ScriptKind::WikiSearch => format!(
            "\n  <script type=\"module\" src=\"{}\" defer></script>",
            escape_attr(&assets.wiki_search_src)
        ),
        ScriptKind::None => String::new(),
    };

    format!(
        "<!doctype html>\n\
<html lang=\"en\">\n\
<head>\n\
  <meta charset=\"utf-8\">\n\
  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1\">\n\
  <meta name=\"color-scheme\" content=\"light dark\">\n\
  <meta name=\"description\" content=\"{desc}\">\n\
  <title>{title_esc}</title>\n\
  <link rel=\"stylesheet\" href=\"{css_href}\">\n\
  <link rel=\"manifest\" href=\"/manifest.webmanifest\">\n\
  <link rel=\"icon\" href=\"/icons/icon.svg\" type=\"image/svg+xml\">\n\
  <link rel=\"alternate\" type=\"application/atom+xml\" title=\"Kennedy Mosoti Blog\" href=\"/feed.xml\">{extra_script}\n\
</head>\n\
<body>\n\
{body_html}\n\
</body>\n\
</html>\n",
        desc = escape_attr(positioning),
        title_esc = escape_html(title),
        css_href = escape_attr(&assets.css_href),
    )
}

pub(crate) fn bad_request_page(message: &str) -> (u16, String, ScriptKind, String) {
    let msg = message.to_owned();
    let html = render_view(move || {
        view! {
            <SiteShell section=NavSection::Wiki>
                <div class="page-container app-surface">
                    <h1>"400 — Bad Request"</h1>
                    <p class="surface-lead">{msg}</p>
                    <div class="stage-toolbar">
                        <a class="btn primary" href="/wiki">"Return to Wiki"</a>
                        <a class="btn" href="/">"Return to Cockpit"</a>
                    </div>
                </div>
            </SiteShell>
        }
    });
    (
        400,
        "Bad Request — kennedy.mosoti.dev".to_owned(),
        ScriptKind::None,
        html,
    )
}

pub(crate) fn not_found_page(route: &str) -> (u16, String, ScriptKind, String) {
    let route_s = route.to_owned();
    let html = render_view(move || {
        view! {
            <SiteShell section=NavSection::Home>
                <div class="page-container app-surface">
                    <h1>"404 — Route Not Found"</h1>
                    <p class="surface-lead">{format!("No published page exists at `{route_s}`.")}</p>
                    <div class="stage-toolbar">
                        <a class="btn primary" href="/">"Return to Cockpit"</a>
                        <a class="btn" href="/projects">"Projects"</a>
                        <a class="btn" href="/wiki">"Wiki"</a>
                        <a class="btn" href="/blog">"Blog"</a>
                    </div>
                </div>
            </SiteShell>
        }
    });
    (
        404,
        "Not Found — kennedy.mosoti.dev".to_owned(),
        ScriptKind::None,
        html,
    )
}
