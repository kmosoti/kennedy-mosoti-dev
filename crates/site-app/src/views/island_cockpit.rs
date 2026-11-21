//! Ultra-lean Leptos `0.8.23` `#[island]` hydrating the SSR 2.5D Cockpit Stage.
//!
//! Pre-compiled fixed-point `SceneGeometry` (`data-scene` on `#iso-stage-svg`) is decoded
//! once on hydration (zero `iso-diagram`, zero `serde_json`, zero `core::fmt` in `wasm32`).
//! Pointer drag events are `rAF`-coalesced and patch keyed `<polygon>` `points`, caption,
//! projection toggle, and Kahn topological `<g>` painter's order in place.

use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy)]
struct OrbitState {
    yaw: f32,
    pitch: f32,
    zoom: f32,
    is_2d: bool,
}

#[cfg(target_arch = "wasm32")]
impl Default for OrbitState {
    fn default() -> Self {
        Self {
            yaw: 35.0,
            pitch: 30.0,
            zoom: 1.0,
            is_2d: false,
        }
    }
}

/// Interactive 2.5D Cockpit Orbit/Zoom Island (`#[island]`).
#[island]
pub fn CockpitStageIsland() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    hydrate_stage_controls();

    view! { <span class="sr-only" data-cockpit-hydrated="true"></span> }
}

#[cfg(target_arch = "wasm32")]
fn hydrate_stage_controls() {
    use crate::dto::SceneGeometry;
    use crate::views::iso_math::{OrbitProjector, clamp_f32};
    use crate::views::stage_dom::{
        capture_ptr, clicked_node_slug, hit_node_target, patch_svg_in_place, read_dom_attr,
        sync_node_selection, toggle_shortcuts_dialog,
    };
    use std::cell::Cell;
    use std::rc::Rc;
    use wasm_bindgen::JsCast;
    use wasm_bindgen::closure::Closure;

    let Some(doc) = web_sys::window().and_then(|w| w.document()) else {
        return;
    };
    let scene = Rc::new(SceneGeometry::decode(&read_dom_attr(
        "iso-stage-svg",
        "data-scene",
    )));
    let orbit = Rc::new(Cell::new(OrbitState::default()));
    let drag: Rc<Cell<Option<(i32, f32, f32, f32, f32)>>> = Rc::new(Cell::new(None));
    let pending: Rc<Cell<Option<(f32, f32)>>> = Rc::new(Cell::new(None));
    let raf_scheduled = Rc::new(Cell::new(false));

    let apply_now = {
        let scene = Rc::clone(&scene);
        let orbit = Rc::clone(&orbit);
        move || {
            let st = orbit.get();
            let proj = OrbitProjector::fit_orbit(&scene, st.yaw, st.pitch, st.is_2d);
            patch_svg_in_place(&scene, proj, st.yaw, st.pitch, st.zoom, st.is_2d);
        }
    };

    let raf_cb = {
        let pending = Rc::clone(&pending);
        let raf_scheduled = Rc::clone(&raf_scheduled);
        let orbit = Rc::clone(&orbit);
        let apply_now = apply_now.clone();
        Closure::<dyn FnMut()>::new(move || {
            raf_scheduled.set(false);
            if let Some((y, p)) = pending.take() {
                let mut st = orbit.get();
                st.yaw = y;
                st.pitch = p;
                orbit.set(st);
                apply_now();
            }
        })
    };
    let raf_cb = Rc::new(raf_cb);

    if let Some(wrap) = doc.get_element_by_id("cockpit-stage-wrap") {
        // pointerdown
        {
            let orbit = Rc::clone(&orbit);
            let drag = Rc::clone(&drag);
            let cb = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
                move |ev: web_sys::PointerEvent| {
                    if (ev.button() == 0 || ev.button() == 2) && !hit_node_target(&ev) {
                        capture_ptr(&ev, true);
                        let st = orbit.get();
                        drag.set(Some((
                            ev.pointer_id(),
                            ev.client_x() as f32,
                            ev.client_y() as f32,
                            st.yaw,
                            st.pitch,
                        )));
                    }
                },
            );
            let _ =
                wrap.add_event_listener_with_callback("pointerdown", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        // pointermove (rAF-coalesced)
        {
            let drag = Rc::clone(&drag);
            let pending = Rc::clone(&pending);
            let raf_scheduled = Rc::clone(&raf_scheduled);
            let raf_cb = Rc::clone(&raf_cb);
            let cb = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
                move |ev: web_sys::PointerEvent| {
                    if let Some((pid, sx, sy, base_yaw, base_pitch)) = drag.get()
                        && ev.pointer_id() == pid
                    {
                        let dx = (ev.client_x() as f32) - sx;
                        let dy = (ev.client_y() as f32) - sy;
                        let next_yaw = (base_yaw + dx * 0.4).rem_euclid(360.0);
                        let next_pitch = clamp_f32(base_pitch + dy * 0.2, 18.0, 56.0);
                        pending.set(Some((next_yaw, next_pitch)));
                        if !raf_scheduled.replace(true)
                            && let Some(win) = web_sys::window()
                            && win
                                .request_animation_frame(raf_cb.as_ref().as_ref().unchecked_ref())
                                .is_err()
                        {
                            raf_scheduled.set(false);
                        }
                    }
                },
            );
            let _ =
                wrap.add_event_listener_with_callback("pointermove", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        // pointerup + pointercancel
        {
            let drag = Rc::clone(&drag);
            let cb = Closure::<dyn FnMut(web_sys::PointerEvent)>::new(
                move |ev: web_sys::PointerEvent| {
                    if let Some((pid, ..)) = drag.get()
                        && ev.pointer_id() == pid
                    {
                        capture_ptr(&ev, false);
                        drag.set(None);
                    }
                },
            );
            let _ = wrap.add_event_listener_with_callback("pointerup", cb.as_ref().unchecked_ref());
            let _ =
                wrap.add_event_listener_with_callback("pointercancel", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        // wheel
        {
            let orbit = Rc::clone(&orbit);
            let apply_now = apply_now.clone();
            let cb =
                Closure::<dyn FnMut(web_sys::WheelEvent)>::new(move |ev: web_sys::WheelEvent| {
                    if ev.ctrl_key() || ev.meta_key() {
                        ev.prevent_default();
                        let factor = if ev.delta_y() > 0.0 { 0.92 } else { 1.08 };
                        let mut st = orbit.get();
                        st.zoom = clamp_f32(st.zoom * factor, 0.6, 2.0);
                        orbit.set(st);
                        apply_now();
                    }
                });
            let _ = wrap.add_event_listener_with_callback("wheel", cb.as_ref().unchecked_ref());
            cb.forget();
        }
        // keydown
        {
            let scene = Rc::clone(&scene);
            let orbit = Rc::clone(&orbit);
            let apply_now = apply_now.clone();
            let cb = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
                move |ev: web_sys::KeyboardEvent| {
                    let mut st = orbit.get();
                    let mut changed = true;
                    match ev.key().as_str() {
                        "ArrowLeft" => st.yaw = (st.yaw - 15.0).rem_euclid(360.0),
                        "ArrowRight" => st.yaw = (st.yaw + 15.0).rem_euclid(360.0),
                        "ArrowUp" => st.pitch = clamp_f32(st.pitch + 4.0, 18.0, 56.0),
                        "ArrowDown" => st.pitch = clamp_f32(st.pitch - 4.0, 18.0, 56.0),
                        "+" | "=" => st.zoom = clamp_f32(st.zoom * 1.12, 0.6, 2.0),
                        "-" | "_" => st.zoom = clamp_f32(st.zoom * 0.89, 0.6, 2.0),
                        "0" => st = OrbitState::default(),
                        "?" => {
                            changed = false;
                            ev.prevent_default();
                            toggle_shortcuts_dialog(true);
                        }
                        "Enter" | " " => {
                            changed = false;
                            if let Some(slug) = ev
                                .target()
                                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
                                .and_then(|el| el.closest("[data-node-slug]").ok().flatten())
                                .and_then(|el| el.get_attribute("data-node-slug"))
                            {
                                ev.prevent_default();
                                sync_node_selection(&scene, Some(&slug));
                            }
                        }
                        _ => changed = false,
                    }
                    if changed {
                        ev.prevent_default();
                        orbit.set(st);
                        apply_now();
                    }
                },
            );
            let _ = wrap.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref());
            cb.forget();
        }
    }

    // Delegated document click for SVG nodes, rail node buttons, and stage controls toolbar
    let click_cb =
        Closure::<dyn FnMut(web_sys::MouseEvent)>::new(move |ev: web_sys::MouseEvent| {
            if let Some(slug) = clicked_node_slug(&ev) {
                sync_node_selection(&scene, Some(&slug));
                return;
            }
            let Some(target) = ev
                .target()
                .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
            else {
                return;
            };
            if let Some(slug) = target
                .closest("[data-rail-node]")
                .ok()
                .flatten()
                .and_then(|el| el.get_attribute("data-rail-node"))
            {
                sync_node_selection(&scene, Some(&slug));
                return;
            }
            if let Some(act) = target
                .closest("[data-stage-act]")
                .ok()
                .flatten()
                .and_then(|el| el.get_attribute("data-stage-act"))
            {
                let mut st = orbit.get();
                match act.as_str() {
                    "25d" => st.is_2d = false,
                    "2d" => st.is_2d = true,
                    "zout" => st.zoom = clamp_f32(st.zoom * 0.88, 0.6, 2.0),
                    "zin" => st.zoom = clamp_f32(st.zoom * 1.14, 0.6, 2.0),
                    "reset" => st = OrbitState::default(),
                    "keys" => {
                        toggle_shortcuts_dialog(true);
                        return;
                    }
                    _ => return,
                }
                orbit.set(st);
                apply_now();
            }
        });
    let _ = doc.add_event_listener_with_callback("click", click_cb.as_ref().unchecked_ref());
    click_cb.forget();
}
