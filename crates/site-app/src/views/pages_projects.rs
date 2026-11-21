//! SSR views for Home/Cockpit, Projects index, and individual Moiric project pages.

use crate::views::icons::StatusPill;
use crate::views::inspector_rail::CockpitWorkspace;
use crate::views::layout::{NavSection, SiteShell};
use crate::views::project_bays::build_project_bays;
use leptos::prelude::*;
use site_content::{Project, SiteContent};

/// Home Cockpit landing view combining interactive 2.5D island + editorial index strips.
#[component]
pub fn HomePage(content: SiteContent) -> impl IntoView {
    let bays = build_project_bays(&content);
    let identity = content.identity.clone();
    let recent_posts = content.blog.iter().take(3).cloned().collect::<Vec<_>>();
    let wiki_items = content.wiki.iter().take(4).cloned().collect::<Vec<_>>();

    view! {
        <SiteShell section=NavSection::Home>
            <CockpitWorkspace bays=bays initial_bay="nomos".to_owned() />

            <div class="page-container app-surface">
                <section aria-labelledby="identity-heading">
                    <div class="card-header">
                        <h2 id="identity-heading">{identity.headline}</h2>
                        <a class="btn" href="/biography">"Full Biography & 20 Records"</a>
                    </div>
                    <p class="surface-lead">{identity.positioning}</p>
                    <div class="kpi-strip">
                        {identity
                            .disciplines
                            .into_iter()
                            .map(|d| view! {
                                <article class="kpi-cell">
                                    <h3>{d.title}</h3>
                                    <p class="brand-sub">{d.summary}</p>
                                    <span class="kbd-hint">{d.id}</span>
                                </article>
                            })
                            .collect_view()}
                    </div>
                </section>

                <section aria-labelledby="writing-heading">
                    <div class="card-header">
                        <h2 id="writing-heading">"Public Wiki & Engineering Blog"</h2>
                        <div class="stage-toolbar">
                            <a class="btn" href="/wiki">"Browse Wiki"</a>
                            <a class="btn" href="/blog">"All Posts"</a>
                        </div>
                    </div>
                    <div class="card-grid">
                        {recent_posts
                            .into_iter()
                            .map(|p| view! {
                                <article class="panel-card">
                                    <div class="card-header">
                                        <span class="kbd-hint num">{p.date}</span>
                                        <span class="kbd-hint">"Blog"</span>
                                    </div>
                                    <h3><a href=format!("/blog/{}", p.slug)>{p.title}</a></h3>
                                    <p class="brand-sub">{p.summary}</p>
                                </article>
                            })
                            .collect_view()}
                        {wiki_items
                            .into_iter()
                            .map(|w| view! {
                                <article class="panel-card">
                                    <div class="card-header">
                                        <span class="kbd-hint">{w.category}</span>
                                        <span class="kbd-hint">"Wiki"</span>
                                    </div>
                                    <h3><a href=format!("/wiki/{}", w.slug)>{w.title}</a></h3>
                                    <p class="brand-sub">{w.summary}</p>
                                </article>
                            })
                            .collect_view()}
                    </div>
                </section>
            </div>
        </SiteShell>
    }
}

/// Projects directory page (`/projects`).
#[component]
pub fn ProjectsIndexPage(content: SiteContent) -> impl IntoView {
    view! {
        <SiteShell section=NavSection::Projects>
            <div class="page-container app-surface">
                <header>
                    <h1 class="view-heading">"Open-Source Moiric Projects"</h1>
                    <p class="surface-lead">
                        "Five standalone Apache-2.0 Rust repositories separated along strict dependency and verification boundaries. Each project explicitly states its current lifecycle class (`Released Alpha`, `In Progress`, or `Architecture`)."
                    </p>
                </header>

                <div class="card-grid">
                    {content
                        .projects
                        .into_iter()
                        .map(|p| view! {
                            <article class="panel-card">
                                <div class="card-header">
                                    <h2><a href=format!("/projects/{}", p.slug)>{p.name}</a></h2>
                                    <StatusPill label=p.status.label() tone=p.status.tone() />
                                </div>
                                <p class="brand-sub">{p.tagline}</p>
                                <p>{p.summary}</p>
                                <div class="boundary-callout">
                                    <strong>"Claim boundary: "</strong>
                                    {p.claim_boundary}
                                </div>
                                <div class="stage-toolbar">
                                    <a class="btn primary" href=format!("/projects/{}", p.slug)>"Inspect Architecture"</a>
                                    <a class="btn" href=p.repo>"GitHub"</a>
                                </div>
                            </article>
                        })
                        .collect_view()}
                </div>
            </div>
        </SiteShell>
    }
}

/// Single Moiric project detail page (`/projects/{slug}`) with 2.5D island + prose.
#[component]
pub fn ProjectDetailPage(content: SiteContent, project: Project) -> impl IntoView {
    let bays = build_project_bays(&content);
    let slug = project.slug.clone();

    view! {
        <SiteShell section=NavSection::Projects>
            <CockpitWorkspace bays=bays initial_bay=slug />

            <div class="page-container app-surface">
                <article class="prose">
                    <div class="card-header">
                        <StatusPill label=project.status.label() tone=project.status.tone() />
                        <span class="kbd-hint num">{project.version}</span>
                    </div>
                    <h1>{format!("{} — {}", project.name, project.tagline)}</h1>
                    <p class="surface-lead">{project.summary}</p>
                    <div class="boundary-callout">
                        <strong>"Claim boundary: "</strong>
                        {project.claim_boundary}
                    </div>
                    <ul>
                        {project
                            .highlights
                            .into_iter()
                            .map(|h| view! { <li>{h}</li> })
                            .collect_view()}
                    </ul>
                    <div inner_html=project.body_html></div>
                </article>
            </div>
        </SiteShell>
    }
}
