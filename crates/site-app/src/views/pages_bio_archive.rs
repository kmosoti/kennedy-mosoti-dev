//! SSR views for Biography (`/biography`), Records Archive (`/archive`),
//! and single record pages (`/archive/{slug}`).

use crate::views::icons::StatusPill;
use crate::views::layout::{NavSection, SiteShell};
use leptos::prelude::*;
use site_content::{EvidenceClass, Record, SiteContent};

/// Kennedy Mosoti biography, disciplines, 6-stage operating pattern, skills, and record shelves.
#[component]
pub fn BiographyPage(content: SiteContent) -> impl IntoView {
    let id = content.identity.clone();
    let skills = content.skills.skills.clone();
    let records = content.records.clone();

    view! {
        <SiteShell section=NavSection::Biography>
            <div class="page-container app-surface">
                <header>
                    <h1 class="view-heading">{format!("{} — {}", id.name, id.headline)}</h1>
                    <p class="surface-lead">{id.bio}</p>
                    <div class="stage-toolbar">
                        <span class="kbd-hint">{id.location}</span>
                        {id.links
                            .into_iter()
                            .map(|l| view! { <a class="btn" href=l.href>{l.label}</a> })
                            .collect_view()}
                    </div>
                </header>

                <section aria-labelledby="operating-pattern-heading">
                    <h2 id="operating-pattern-heading">"Six-Stage Operating Pattern"</h2>
                    <div class="card-grid">
                        {id.operating_pattern
                            .into_iter()
                            .enumerate()
                            .map(|(i, s)| view! {
                                <article class="panel-card">
                                    <span class="kbd-hint num">{format!("Stage 0{}", i + 1)}</span>
                                    <h3>{s.stage}</h3>
                                    <p class="brand-sub">{s.line}</p>
                                </article>
                            })
                            .collect_view()}
                    </div>
                </section>

                <section aria-labelledby="skills-heading">
                    <h2 id="skills-heading">"Skills by Evidence Class"</h2>
                    <p class="brand-sub">
                        "Production skills cite only production records; independent Rust, Arrow/Parquet, and agent-runtime work are never claimed as production experience."
                    </p>
                    <div class="card-grid">
                        {skills
                            .into_iter()
                            .map(|s| view! {
                                <article class="panel-card">
                                    <div class="card-header">
                                        <h3>{s.label}</h3>
                                        <StatusPill label=s.evidence.label() tone=s.evidence.tone() />
                                    </div>
                                    {s.boundary.map(|b| view! {
                                        <div class="boundary-callout">{b}</div>
                                    })}
                                    <div class="tag-row">
                                        {s.records
                                            .into_iter()
                                            .map(|slug| {
                                                let href = format!("/archive/{slug}");
                                                view! {
                                                    <a class="kbd-hint" href=href>{slug}</a>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                </article>
                            })
                            .collect_view()}
                    </div>
                </section>

                <section aria-labelledby="records-shelf-heading">
                    <div class="card-header">
                        <h2 id="records-shelf-heading">"Career & Engineering Records (20)"</h2>
                        <a class="btn" href="/archive">"Open Full Archive Index"</a>
                    </div>
                    <div class="card-grid">
                        {records
                            .into_iter()
                            .map(render_record_card)
                            .collect_view()}
                    </div>
                </section>
            </div>
        </SiteShell>
    }
}

/// Full Records Archive index grouped by the four non-blended evidence classes (`/archive`).
#[component]
pub fn ArchiveIndexPage(content: SiteContent) -> impl IntoView {
    let classes = [
        EvidenceClass::Production,
        EvidenceClass::OperationalExposure,
        EvidenceClass::Developing,
        EvidenceClass::Independent,
    ];

    view! {
        <SiteShell section=NavSection::Biography>
            <div class="page-container app-surface">
                <header>
                    <h1 class="view-heading">"Evidence Archive"</h1>
                    <p class="surface-lead">
                        "Every record carries one evidence class (`Production`, `Operational exposure`, `Developing`, or `Independent`) and an explicit claim boundary stating what it asserts and what it excludes."
                    </p>
                </header>

                {classes
                    .into_iter()
                    .map(|cls| {
                        let matching: Vec<Record> = content
                            .records
                            .iter()
                            .filter(|r| r.evidence == cls)
                            .cloned()
                            .collect();
                        view! {
                            <section>
                                <div class="card-header">
                                    <h2>{cls.label()}</h2>
                                    <StatusPill label=cls.label() tone=cls.tone() />
                                </div>
                                <p class="brand-sub">{cls.gloss()}</p>
                                <div class="card-grid">
                                    {matching.into_iter().map(render_record_card).collect_view()}
                                </div>
                            </section>
                        }
                    })
                    .collect_view()}
            </div>
        </SiteShell>
    }
}

/// Single career/project archive record page (`/archive/{slug}`).
#[component]
pub fn ArchiveRecordPage(record: Record) -> impl IntoView {
    view! {
        <SiteShell section=NavSection::Biography>
            <div class="page-container app-surface">
                <nav aria-label="Breadcrumb" class="stage-toolbar">
                    <a class="btn quiet" href="/archive">"← Evidence Archive"</a>
                    <a class="btn quiet" href="/biography">"Biography"</a>
                </nav>

                <article class="prose">
                    <div class="card-header">
                        <StatusPill label=record.evidence.label() tone=record.evidence.tone() />
                        <span class="kbd-hint">{record.kind.label()}</span>
                        {record.period.map(|p| view! { <span class="kbd-hint">{p}</span> })}
                    </div>
                    <h1 class="view-heading">{record.title}</h1>
                    <p class="surface-lead">{record.summary}</p>
                    <p class="brand-sub">{record.evidence.gloss()}</p>
                    {record.claim_boundary.map(|b| view! {
                        <div class="boundary-callout">
                            <strong>"Claim boundary: "</strong>
                            {b}
                        </div>
                    })}
                    <div inner_html=record.body_html></div>
                </article>
            </div>
        </SiteShell>
    }
}

fn render_record_card(r: Record) -> impl IntoView {
    view! {
        <article class="panel-card">
            <div class="card-header">
                <StatusPill label=r.evidence.label() tone=r.evidence.tone() />
                <span class="kbd-hint">{r.kind.label()}</span>
            </div>
            <h3><a href=format!("/archive/{}", r.slug)>{r.title}</a></h3>
            <p class="brand-sub">{r.summary}</p>
            {r.claim_boundary.map(|b| view! {
                <div class="boundary-callout">{b}</div>
            })}
        </article>
    }
}
