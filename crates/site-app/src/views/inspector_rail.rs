//! SSR 3-column Cockpit workspace wrapping left rail, `CockpitStageIsland`, and inspector rail.

use crate::dto::ProjectBayDto;
use crate::views::icons::StatusPill;
use crate::views::island_cockpit::CockpitStageIsland;
use crate::views::iso_stage::IsoStageSvg;
use crate::views::stage_dom::node_dom_id;
use leptos::prelude::*;

const SHORTCUTS_DIALOG_ID: &str = "cockpit-shortcuts-dialog";
const STAGE_POPOVER_ID: &str = "cockpit-stage-actions";

/// Renders the full 3-column SSR Cockpit workspace for `initial_bay`.
/// Only the interactive 2.5D stage viewport hydrates in `wasm32`, keeping the WASM payload minimal.
#[component]
pub fn CockpitWorkspace(bays: Vec<ProjectBayDto>, initial_bay: String) -> impl IntoView {
    let active_bay = bays
        .iter()
        .find(|b| b.slug == initial_bay)
        .or_else(|| bays.first())
        .cloned()
        .expect("at least one project bay");

    let active_slug = active_bay.slug.clone();
    let repo_href = active_bay.repo.clone();
    let bay_for_svg = active_bay.clone();
    let bay_nodes_rail = active_bay.nodes.clone();
    let bay_nodes_panels = active_bay.nodes.clone();

    view! {
        <section class="cockpit-workspace" aria-label="Interactive 2.5D Moiric Systems Cockpit">
            <aside class="rail-left" aria-label="Moiric Project Bays">
                <div class="rail-section">
                    <div class="rail-heading">"Moiric Bays"</div>
                    <ul class="rail-list" role="list" aria-label="Select project bay">
                        {bays
                            .into_iter()
                            .map(|bay| {
                                let is_active = bay.slug == active_slug;
                                let item_class = if is_active { "rail-item is-active" } else { "rail-item" };
                                view! {
                                    <li class=item_class>
                                        <a
                                            class="rail-btn"
                                            href=format!("/projects/{}", bay.slug)
                                            aria-current=if is_active { Some("page") } else { None }
                                        >
                                            <span>{bay.name}</span>
                                            <StatusPill label=bay.status tone=bay.status_tone />
                                        </a>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>

                <div class="rail-section">
                    <div class="rail-heading">"Bay Nodes"</div>
                    <ul class="rail-list">
                        {bay_nodes_rail
                            .into_iter()
                            .enumerate()
                            .map(|(idx, node)| {
                                let base = node_dom_id(&node.slug);
                                let is_first = idx == 0;
                                view! {
                                    <li
                                        id=format!("{base}-rail-li")
                                        class=if is_first { "rail-item is-active" } else { "rail-item" }
                                    >
                                        <button
                                            id=format!("{base}-rail-btn")
                                            type="button"
                                            class="rail-btn"
                                            data-rail-node=node.slug
                                            aria-pressed=if is_first { "true" } else { "false" }
                                        >
                                            <span>{node.label}</span>
                                            <span class="kbd-hint">{node.asset_kind}</span>
                                        </button>
                                    </li>
                                }
                            })
                            .collect_view()}
                    </ul>
                </div>
            </aside>

            <div class="stage-pane">
                <div class="stage-header">
                    <div>
                        <h1 class="view-heading">
                            {format!("{} — {}", active_bay.name, active_bay.tagline)}
                        </h1>
                    </div>
                    <div class="stage-toolbar">
                        <StatusPill label=active_bay.status.clone() tone=active_bay.status_tone.clone() />
                        <a class="btn" href=format!("/projects/{}", active_bay.slug)>
                            "Open Project Page"
                        </a>
                    </div>
                </div>

                <figure class="stage-viewport">
                    <div
                        id="cockpit-stage-wrap"
                        class="stage-svg-wrap"
                        tabindex="0"
                        role="region"
                        aria-label="2.5D Isometric Stage (Arrow keys orbit, +/- zoom, 0 resets)"
                    >
                        <IsoStageSvg bay=bay_for_svg />
                    </div>

                    <figcaption id="cockpit-stage-caption" class="stage-caption num">
                        "Orbit 35° · Pitch 30° · Zoom 1.00× · 2.5D Isometric · 0 Hz Idle"
                    </figcaption>

                    <div id="cockpit-stage-controls" class="stage-toolbar stage-controls" role="toolbar" aria-label="Stage Controls">
                        <div class="seg-group" role="group" aria-label="Projection mode">
                            <button id="cockpit-btn-25d" type="button" class="seg-btn" aria-pressed="true" data-stage-act="25d">"2.5D"</button>
                            <button id="cockpit-btn-2d" type="button" class="seg-btn" aria-pressed="false" data-stage-act="2d">"2D"</button>
                        </div>
                        <button type="button" class="btn quiet" aria-label="Zoom out" data-stage-act="zout">"-"</button>
                        <button type="button" class="btn quiet" aria-label="Zoom in" data-stage-act="zin">"+"</button>
                        <button type="button" class="btn quiet" data-stage-act="reset">"Reset"</button>
                        <button type="button" class="btn quiet" data-stage-act="keys">"Shortcuts (?)"</button>
                        <button type="button" class="btn stage-popover-trigger" popovertarget=STAGE_POPOVER_ID>"Stage Info"</button>
                        <div id=STAGE_POPOVER_ID popover="auto" class="stage-popover">
                            <p class="brand-sub">"Drag background or press Arrow keys to orbit; Ctrl+Scroll or +/- to zoom; 0 to reset."</p>
                            <a class="btn quiet" href="/wiki/cockpit-25d-without-threejs">"How 2.5D SVG Works"</a>
                        </div>
                    </div>
                </figure>
                <CockpitStageIsland />
            </div>

            <aside class="inspector-rail" aria-label="System & Node Inspector">
                <div class="inspector-block">
                    <h2>{active_bay.name}</h2>
                    <p class="brand-sub">{active_bay.summary}</p>
                    <dl class="inspector-meta">
                        <dt>"Status"</dt>
                        <dd>{format!("{} ({})", active_bay.status, active_bay.version)}</dd>
                        <dt>"Repository"</dt>
                        <dd><a href=repo_href>{active_bay.repo}</a></dd>
                    </dl>
                </div>

                {bay_nodes_panels
                    .into_iter()
                    .enumerate()
                    .map(|(idx, n)| {
                        let base = node_dom_id(&n.slug);
                        view! {
                            <div
                                id=format!("{base}-panel")
                                class="inspector-block"
                                hidden=idx != 0
                            >
                                <h3>{format!("Selected Node: {}", n.label)}</h3>
                                <dl class="inspector-meta">
                                    <dt>"Role"</dt>
                                    <dd>{n.role}</dd>
                                    <dt>"Component"</dt>
                                    <dd>{n.asset_kind}</dd>
                                    <dt>"State"</dt>
                                    <dd>{n.status}</dd>
                                    <dt>"Mechanism"</dt>
                                    <dd>{n.mechanism}</dd>
                                </dl>
                            </div>
                        }
                    })
                    .collect_view()}

                <div class="inspector-block">
                    <h3>"Claim Boundary"</h3>
                    <p class="brand-sub">{active_bay.claim_boundary}</p>
                </div>

                <dialog id=SHORTCUTS_DIALOG_ID class="cockpit-dialog" closedby="any">
                    <div class="dialog-header">
                        <strong>"Cockpit Keyboard Controls"</strong>
                        <form method="dialog">
                            <button type="submit" class="btn quiet">"Close"</button>
                        </form>
                    </div>
                    <div class="dialog-body">
                        <p>"Arrows: orbit · +/-: zoom · 0: reset · Tab + Enter: inspect bay nodes · ?: shortcuts"</p>
                    </div>
                </dialog>
            </aside>
        </section>
    }
}
