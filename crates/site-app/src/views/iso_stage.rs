//! SSR-only static SVG emitter for the 2.5D isometric bay stage.
//! Emits deterministic keyed `id`s on every zone, conduit, node `<polygon>`, and callout
//! plus a compact `data-scene` fixed-point geometry attribute for zero-teardown WASM orbit.

use crate::dto::ProjectBayDto;
use crate::views::iso_math::{OrbitProjector, fmt_pts, fmt_t1, from_centi, tint_var, zoom_viewbox};
use crate::views::iso_project::{project_box, project_route, topological_box_order};
use crate::views::stage_dom::node_dom_id;
use leptos::prelude::*;

/// Renders the initial SSR `<svg id="iso-stage-svg">` for `bay` using pre-compiled `SceneGeometry`.
#[component]
pub fn IsoStageSvg(bay: ProjectBayDto) -> impl IntoView {
    let scene = bay.scene;
    let encoded_scene = scene.encode();
    let proj = OrbitProjector::fit_orbit(&scene, 35.0, 30.0, false);
    let viewbox = zoom_viewbox(1.0);

    let [u0c, v0c, u1c, v1c] = scene.grid_extent;
    let u0 = (from_centi(u0c) - 3.0).floor() as i32;
    let u1 = (from_centi(u1c) + 3.0).ceil() as i32;
    let v0 = (from_centi(v0c) - 3.0).floor() as i32;
    let v1 = (from_centi(v1c) + 3.0).ceil() as i32;
    let mut grid_lines = Vec::new();
    for u in u0..=u1 {
        grid_lines.push((
            proj.project(u as f32, v0 as f32, 0.0),
            proj.project(u as f32, v1 as f32, 0.0),
        ));
    }
    for v in v0..=v1 {
        grid_lines.push((
            proj.project(u0 as f32, v as f32, 0.0),
            proj.project(u1 as f32, v as f32, 0.0),
        ));
    }

    let zones_view = scene
        .zones
        .iter()
        .map(|zone| {
            let [zu0, zv0, zu1, zv1] = zone.bounds;
            let pts = fmt_pts(&[
                proj.project_c(zu0, zv0, 0.0),
                proj.project_c(zu1, zv0, 0.0),
                proj.project_c(zu1, zv1, 0.0),
                proj.project_c(zu0, zv1, 0.0),
            ]);
            let anchor = proj.project_c((zu0 + zu1) / 2, zv1 + 36, 0.0);
            let col = tint_var(&zone.tint);
            let pid = format!("iso-zone-p-{}", zone.id);
            let tid = format!("iso-zone-t-{}", zone.id);
            let title = zone.title.clone();
            view! {
                <g>
                    <polygon id=pid points=pts fill=col fill-opacity="0.10" stroke=col stroke-width="1.6" />
                    <text id=tid class="iso-zone-label" x=fmt_t1(anchor.0) y=fmt_t1(anchor.1 + 4.0) text-anchor="middle">{title}</text>
                </g>
            }
        })
        .collect_view();

    let edges_view = scene
        .routes
        .iter()
        .map(|route| {
            let pr = project_route(route, proj);
            let col = tint_var(&route.tint);
            let dash = match route.style.as_str() {
                "dashed" => "9 5",
                "dotted" => "2.4 4.8",
                _ => "",
            };
            let rid = route.id;
            view! {
                <g>
                    <polyline id=format!("iso-edge-{rid}-s") class="iso-edge-shadow" points=pr.shadow stroke-width="5.6" stroke-dasharray=dash />
                    <polyline id=format!("iso-edge-{rid}-b") class="iso-edge-base" points=pr.base stroke-width="4.6" stroke-dasharray=dash />
                    <polyline id=format!("iso-edge-{rid}-c") class="iso-edge-core" points=pr.core stroke=col stroke-width="3.0" stroke-dasharray=dash />
                    <polyline id=format!("iso-edge-{rid}-k") class="iso-edge-crest" points=pr.crest stroke-width="1.0" stroke-dasharray=dash />
                    <polygon id=format!("iso-edge-{rid}-h") points=pr.chevron fill=col stroke="var(--iso-outline)" stroke-width="0.9" />
                </g>
            }
        })
        .collect_view();

    let first_slug = scene.boxes.first().map(|b| b.slug.clone());
    let order = topological_box_order(&scene.boxes, proj);

    let nodes_view = order
        .iter()
        .map(|&idx| {
            let node = &scene.boxes[idx];
            let pb = project_box(node, proj);
            let base = node_dom_id(&node.slug);
            let is_sel = first_slug.as_deref() == Some(node.slug.as_str());
            let flag = if is_sel { "true" } else { "false" };
            let aria = format!("{}: {} ({})", node.title, node.subtitle, node.status);
            let tint = node.tint.clone();
            let state = node.status.clone();
            let slug = node.slug.clone();
            view! {
                <g
                    id=base.clone()
                    class="iso-node"
                    role="button"
                    tabindex="0"
                    aria-label=aria
                    aria-pressed=flag
                    data-node-slug=slug
                    data-tint=tint
                    data-state=state
                    data-selected=flag
                >
                    <polygon id=format!("{base}-r") class="iso-select-ring" points=pb.ring />
                    <polygon id=format!("{base}-s") points=pb.shadow fill="var(--iso-shadow)" stroke="none" />
                    <polygon id=format!("{base}-a") points=pb.side_a fill="var(--iso-left)" stroke="var(--iso-outline)" stroke-width="1.1" stroke-linejoin="round" />
                    <polygon id=format!("{base}-b") points=pb.side_b fill="var(--iso-right)" stroke="var(--iso-outline)" stroke-width="1.1" stroke-linejoin="round" />
                    <polygon id=format!("{base}-t") points=pb.top fill="var(--iso-top)" stroke="var(--iso-outline)" stroke-width="1.1" stroke-linejoin="round" />
                    <polygon id=format!("{base}-c") points=pb.accent fill="var(--node-tint)" stroke="var(--iso-outline)" stroke-width="0.9" stroke-linejoin="round" />
                    <circle id=format!("{base}-l") cx=fmt_t1(pb.led.0) cy=fmt_t1(pb.led.1) r="3.2" fill="var(--node-led)" stroke="var(--iso-outline)" stroke-width="0.8" />
                </g>
            }
        })
        .collect_view();

    let callouts_view = scene
        .boxes
        .iter()
        .map(|node| {
            let pb = project_box(node, proj);
            let base = node_dom_id(&node.slug);
            let is_sel = first_slug.as_deref() == Some(node.slug.as_str());
            let (cx, cy, cw) = pb.callout;
            let title = node.title.clone();
            let sub = node.subtitle.clone();
            let slug = node.slug.clone();
            view! {
                <g class="iso-callout" data-node-slug=slug>
                    <rect
                        id=format!("{base}-cb")
                        class="iso-callout-box"
                        data-selected=if is_sel { "true" } else { "false" }
                        x=fmt_t1(cx)
                        y=fmt_t1(cy)
                        width=fmt_t1(cw)
                        height="34.0"
                        rx="3"
                    />
                    <text id=format!("{base}-ct") class="iso-callout-title" x=fmt_t1(cx + cw * 0.5) y=fmt_t1(cy + 14.0) text-anchor="middle">{title}</text>
                    <text id=format!("{base}-cs") class="iso-callout-sub" x=fmt_t1(cx + cw * 0.5) y=fmt_t1(cy + 27.5) text-anchor="middle">{sub}</text>
                </g>
            }
        })
        .collect_view();

    view! {
        <svg
            id="iso-stage-svg"
            class="stage-svg"
            role="img"
            aria-roledescription="interactive 2.5D iso-diagram"
            viewBox=viewbox
            data-scene=encoded_scene
        >
            <title>{format!("{} — 2.5D architecture bay", bay.name)}</title>
            <desc>{bay.summary}</desc>
            <g class="iso-grid-lines" aria-hidden="true">
                {grid_lines
                    .into_iter()
                    .enumerate()
                    .map(|(i, (a, b))| view! {
                        <line
                            id=format!("iso-grid-{i}")
                            x1=fmt_t1(a.0)
                            y1=fmt_t1(a.1)
                            x2=fmt_t1(b.0)
                            y2=fmt_t1(b.1)
                        />
                    })
                    .collect_view()}
            </g>
            <g class="iso-zones-layer">{zones_view}</g>
            <g class="iso-edges-layer" aria-hidden="true">{edges_view}</g>
            <g id="iso-nodes-layer" class="iso-nodes-layer">{nodes_view}</g>
            <g class="iso-callouts-layer">{callouts_view}</g>
        </svg>
    }
}
