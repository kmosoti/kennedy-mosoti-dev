//! Browser DOM in-place keyed SVG `<polygon>`/`<polyline>` attribute patcher,
//! topological `<g>` reordering, node/inspector selection sync, and pointer helpers.
//! Zero `format!()` / `core::fmt` calls in `wasm32` to keep the WASM binary tiny.

#[cfg(target_arch = "wasm32")]
use crate::dto::SceneGeometry;
#[cfg(target_arch = "wasm32")]
use crate::views::iso_math::{
    OrbitProjector, fmt_pts, fmt_t1, format_stage_caption, from_centi, push_u32, zoom_viewbox,
};
#[cfg(target_arch = "wasm32")]
use crate::views::iso_project::{project_box, project_route, topological_box_order};

#[cfg(target_arch = "wasm32")]
const SHORTCUTS_DIALOG_ID: &str = "cockpit-shortcuts-dialog";

/// Deterministic DOM id prefix for a bay node slug.
#[must_use]
pub fn node_dom_id(slug: &str) -> String {
    let mut out = String::with_capacity(slug.len() + 9);
    out.push_str("iso-node-");
    for ch in slug.chars() {
        out.push(if ch.is_ascii_alphanumeric() { ch } else { '-' });
    }
    out
}

/// Opens or closes the native `<dialog closedby="any">` keyboard shortcuts reference.
pub fn toggle_shortcuts_dialog(open: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        if let Some(doc) = web_sys::window().and_then(|w| w.document())
            && let Some(el) = doc.get_element_by_id(SHORTCUTS_DIALOG_ID)
            && let Ok(dlg) = el.dyn_into::<web_sys::HtmlDialogElement>()
        {
            if open {
                let _ = dlg.show_modal();
            } else {
                dlg.close();
            }
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = open;
}

#[cfg(target_arch = "wasm32")]
fn fmt_num_id<'a>(buf: &'a mut String, prefix: &str, n: usize, suffix: &str) -> &'a str {
    buf.clear();
    buf.push_str(prefix);
    push_u32(buf, n as u32);
    buf.push_str(suffix);
    buf.as_str()
}

#[cfg(target_arch = "wasm32")]
fn fmt_suf<'a>(buf: &'a mut String, base: &str, suffix: &str) -> &'a str {
    buf.clear();
    buf.push_str(base);
    buf.push_str(suffix);
    buf.as_str()
}

/// Patches SVG `viewBox`, caption, projection buttons, grid lines, zones, routed conduits,
/// keyed node `<polygon>` `points`, callouts, and topological `<g>` painter's order in place.
#[cfg(target_arch = "wasm32")]
pub fn patch_svg_in_place(
    scene: &SceneGeometry,
    proj: OrbitProjector,
    yaw: f32,
    pitch: f32,
    zoom: f32,
    is_2d: bool,
) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let set_attr = |id: &str, name: &str, val: &str| {
        if let Some(el) = doc.get_element_by_id(id) {
            let _ = el.set_attribute(name, val);
        }
    };

    if let Some(cap) = doc.get_element_by_id("cockpit-stage-caption") {
        cap.set_text_content(Some(&format_stage_caption(yaw, pitch, zoom, is_2d)));
    }
    set_attr(
        "cockpit-btn-25d",
        "aria-pressed",
        if !is_2d { "true" } else { "false" },
    );
    set_attr(
        "cockpit-btn-2d",
        "aria-pressed",
        if is_2d { "true" } else { "false" },
    );
    set_attr("iso-stage-svg", "viewBox", &zoom_viewbox(zoom));

    let mut id = String::with_capacity(48);

    let [u0c, v0c, u1c, v1c] = scene.grid_extent;
    let u0 = (from_centi(u0c) - 3.0).floor() as i32;
    let u1 = (from_centi(u1c) + 3.0).ceil() as i32;
    let v0 = (from_centi(v0c) - 3.0).floor() as i32;
    let v1 = (from_centi(v1c) + 3.0).ceil() as i32;
    let mut gidx = 0usize;
    for u in u0..=u1 {
        let a = proj.project(u as f32, v0 as f32, 0.0);
        let b = proj.project(u as f32, v1 as f32, 0.0);
        let gid = fmt_num_id(&mut id, "iso-grid-", gidx, "");
        set_attr(gid, "x1", &fmt_t1(a.0));
        set_attr(gid, "y1", &fmt_t1(a.1));
        set_attr(gid, "x2", &fmt_t1(b.0));
        set_attr(gid, "y2", &fmt_t1(b.1));
        gidx += 1;
    }
    for v in v0..=v1 {
        let a = proj.project(u0 as f32, v as f32, 0.0);
        let b = proj.project(u1 as f32, v as f32, 0.0);
        let gid = fmt_num_id(&mut id, "iso-grid-", gidx, "");
        set_attr(gid, "x1", &fmt_t1(a.0));
        set_attr(gid, "y1", &fmt_t1(a.1));
        set_attr(gid, "x2", &fmt_t1(b.0));
        set_attr(gid, "y2", &fmt_t1(b.1));
        gidx += 1;
    }

    for zone in &scene.zones {
        let [zu0, zv0, zu1, zv1] = zone.bounds;
        let pts = fmt_pts(&[
            proj.project_c(zu0, zv0, 0.0),
            proj.project_c(zu1, zv0, 0.0),
            proj.project_c(zu1, zv1, 0.0),
            proj.project_c(zu0, zv1, 0.0),
        ]);
        set_attr(
            fmt_num_id(&mut id, "iso-zone-p-", zone.id, ""),
            "points",
            &pts,
        );
        let lbl = proj.project_c((zu0 + zu1) / 2, zv1 + 36, 0.0);
        let tid = fmt_num_id(&mut id, "iso-zone-t-", zone.id, "");
        set_attr(tid, "x", &fmt_t1(lbl.0));
        set_attr(tid, "y", &fmt_t1(lbl.1 + 4.0));
    }

    for route in &scene.routes {
        let pr = project_route(route, proj);
        set_attr(
            fmt_num_id(&mut id, "iso-edge-", route.id, "-s"),
            "points",
            &pr.shadow,
        );
        set_attr(
            fmt_num_id(&mut id, "iso-edge-", route.id, "-b"),
            "points",
            &pr.base,
        );
        set_attr(
            fmt_num_id(&mut id, "iso-edge-", route.id, "-c"),
            "points",
            &pr.core,
        );
        set_attr(
            fmt_num_id(&mut id, "iso-edge-", route.id, "-k"),
            "points",
            &pr.crest,
        );
        set_attr(
            fmt_num_id(&mut id, "iso-edge-", route.id, "-h"),
            "points",
            &pr.chevron,
        );
    }

    let layer = doc.get_element_by_id("iso-nodes-layer");
    for idx in topological_box_order(&scene.boxes, proj) {
        let bx = &scene.boxes[idx];
        let pb = project_box(bx, proj);
        let base = node_dom_id(&bx.slug);
        set_attr(fmt_suf(&mut id, &base, "-r"), "points", &pb.ring);
        set_attr(fmt_suf(&mut id, &base, "-s"), "points", &pb.shadow);
        set_attr(fmt_suf(&mut id, &base, "-a"), "points", &pb.side_a);
        set_attr(fmt_suf(&mut id, &base, "-b"), "points", &pb.side_b);
        set_attr(fmt_suf(&mut id, &base, "-t"), "points", &pb.top);
        set_attr(fmt_suf(&mut id, &base, "-c"), "points", &pb.accent);
        let lid = fmt_suf(&mut id, &base, "-l");
        set_attr(lid, "cx", &fmt_t1(pb.led.0));
        set_attr(lid, "cy", &fmt_t1(pb.led.1));

        let (cx, cy, cw) = pb.callout;
        let cb = fmt_suf(&mut id, &base, "-cb");
        set_attr(cb, "x", &fmt_t1(cx));
        set_attr(cb, "y", &fmt_t1(cy));
        set_attr(cb, "width", &fmt_t1(cw));
        let ct = fmt_suf(&mut id, &base, "-ct");
        set_attr(ct, "x", &fmt_t1(cx + cw * 0.5));
        set_attr(ct, "y", &fmt_t1(cy + 14.0));
        let cs = fmt_suf(&mut id, &base, "-cs");
        set_attr(cs, "x", &fmt_t1(cx + cw * 0.5));
        set_attr(cs, "y", &fmt_t1(cy + 27.5));

        if let (Some(parent), Some(node_el)) = (&layer, doc.get_element_by_id(&base)) {
            let _ = parent.append_child(&node_el);
        }
    }
}

/// Synchronizes `data-selected`, `aria-pressed`, left-rail active item, and right-rail
/// inspector panel visibility across SSR DOM nodes when selection changes.
#[cfg(target_arch = "wasm32")]
pub fn sync_node_selection(scene: &SceneGeometry, selected: Option<&str>) {
    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let active_slug = selected.or_else(|| scene.boxes.first().map(|b| b.slug.as_str()));
    let mut id = String::with_capacity(48);

    for bx in &scene.boxes {
        let is_sel = active_slug == Some(bx.slug.as_str());
        let flag = if is_sel { "true" } else { "false" };
        let base = node_dom_id(&bx.slug);
        if let Some(el) = doc.get_element_by_id(&base) {
            let _ = el.set_attribute("data-selected", flag);
            let _ = el.set_attribute("aria-pressed", flag);
        }
        if let Some(cb) = doc.get_element_by_id(fmt_suf(&mut id, &base, "-cb")) {
            let _ = cb.set_attribute("data-selected", flag);
        }
        if let Some(btn) = doc.get_element_by_id(fmt_suf(&mut id, &base, "-rail-btn")) {
            let _ = btn.set_attribute("aria-pressed", flag);
        }
        if let Some(li) = doc.get_element_by_id(fmt_suf(&mut id, &base, "-rail-li")) {
            let _ = li.set_attribute(
                "class",
                if is_sel {
                    "rail-item is-active"
                } else {
                    "rail-item"
                },
            );
        }
        if let Some(panel) = doc.get_element_by_id(fmt_suf(&mut id, &base, "-panel")) {
            if is_sel {
                let _ = panel.remove_attribute("hidden");
            } else {
                let _ = panel.set_attribute("hidden", "");
            }
        }
    }
}

/// Reads an attribute from a server-rendered DOM element by id on `wasm32`.
#[cfg(target_arch = "wasm32")]
pub fn read_dom_attr(id: &str, attr: &str) -> String {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id(id))
        .and_then(|el| el.get_attribute(attr))
        .unwrap_or_default()
}

/// Extracts `data-node-slug` from a clicked SVG node, callout, or rail button ancestor.
pub fn clicked_node_slug(ev: &leptos::ev::MouseEvent) -> Option<String> {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        ev.target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest("[data-node-slug]").ok().flatten())
            .and_then(|el| el.get_attribute("data-node-slug"))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = ev;
        None
    }
}

/// True if a pointer event originated on a node sprite or callout card.
pub fn hit_node_target(ev: &leptos::ev::PointerEvent) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        ev.target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            .and_then(|el| el.closest(".iso-node, .iso-callout").ok().flatten())
            .is_some()
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = ev;
        false
    }
}

/// Acquires or releases pointer capture on `wasm32`.
pub fn capture_ptr(ev: &leptos::ev::PointerEvent, capture: bool) {
    #[cfg(target_arch = "wasm32")]
    {
        use wasm_bindgen::JsCast;
        if let Some(el) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
        {
            let _ = if capture {
                el.set_pointer_capture(ev.pointer_id())
            } else {
                el.release_pointer_capture(ev.pointer_id())
            };
        }
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (ev, capture);
    }
}
