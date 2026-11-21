//! Outer semantic HTML document shell, accessible navigation bar, and footer.

use crate::views::icons::BrandCubeMark;
use leptos::prelude::*;

/// Active top-level navigation section for `aria-current="page"` highlights.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavSection {
    Home,
    Projects,
    Wiki,
    Blog,
    Biography,
}

/// Complete page wrapper with skip link, sticky header, single `<main>`, and footer.
#[component]
pub fn SiteShell(section: NavSection, children: Children) -> impl IntoView {
    let cur = |s: NavSection| if section == s { Some("page") } else { None };
    view! {
        <div class="site-shell">
            <a href="#main-content" class="skip-link">"Skip to main content"</a>

            <header class="site-topbar">
                <a class="topbar-brand" href="/">
                    <BrandCubeMark />
                    <span class="brand-mark">
                        "Kennedy Mosoti"
                        <span class="brand-sub">"· Platform, Reliability & Systems Automation"</span>
                    </span>
                </a>

                <nav class="site-nav" aria-label="Primary navigation">
                    <a class="nav-link" href="/" aria-current=cur(NavSection::Home)>"Home / Cockpit"</a>
                    <a class="nav-link" href="/projects" aria-current=cur(NavSection::Projects)>"Projects"</a>
                    <a class="nav-link" href="/wiki" aria-current=cur(NavSection::Wiki)>"Wiki"</a>
                    <a class="nav-link" href="/blog" aria-current=cur(NavSection::Blog)>"Blog"</a>
                    <a class="nav-link" href="/biography" aria-current=cur(NavSection::Biography)>"Biography"</a>
                    <a class="nav-link" href="/feed.xml" type="application/atom+xml">"RSS"</a>
                </nav>
            </header>

            <main id="main-content" class="site-main" tabindex="-1">
                {children()}
            </main>

            <footer class="site-footer">
                <span>
                    "Kennedy Mosoti · Apache-2.0 · Zero-JS SSR + Pure-Rust 2.5D Cockpit"
                </span>
                <div class="stage-toolbar">
                    <a href="https://github.com/kmosoti">"GitHub"</a>
                    <a href="/archive">"Archive"</a>
                    <a href="/feed.xml">"Atom Feed"</a>
                    <a href="/sitemap.xml">"Sitemap"</a>
                </div>
            </footer>
        </div>
    }
}
