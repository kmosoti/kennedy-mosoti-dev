//! 3D prism and orthogonal conduit projections plus Kahn topological painter's ordering.

use crate::dto::{BoxGeometry, RouteGeometry};
use crate::views::iso_math::{
    OrbitProjector, STAGE_H, STAGE_W, clamp_f32, fast_sin_cos, fmt_pts, from_centi,
};

/// Projected fixed-slot SVG polygon strings for one 3D hardware box.
#[derive(Clone, PartialEq)]
pub struct ProjectedBox {
    pub ring: String,
    pub shadow: String,
    pub side_a: String,
    pub side_b: String,
    pub top: String,
    pub accent: String,
    pub led: (f32, f32),
    pub callout: (f32, f32, f32),
}

/// Projects `node` into shaded 2.5D prism polygons and callout coordinates.
#[must_use]
pub fn project_box(node: &BoxGeometry, proj: OrbitProjector) -> ProjectedBox {
    let [u0, u1, v0, v1, z1] = node.bounds.map(from_centi);
    let ring = fmt_pts(&[
        proj.project(u0 - 0.14, v0 - 0.14, 0.01),
        proj.project(u1 + 0.14, v0 - 0.14, 0.01),
        proj.project(u1 + 0.14, v1 + 0.14, 0.01),
        proj.project(u0 - 0.14, v1 + 0.14, 0.01),
    ]);
    let (su, sv) = (0.34 * z1, -0.14 * z1);
    let shadow = fmt_pts(&[
        proj.project(u0 + su, v0 + sv, 0.0),
        proj.project(u1 + su, v0 + sv, 0.0),
        proj.project(u1 + su, v1 + sv, 0.0),
        proj.project(u0 + su, v1 + sv, 0.0),
    ]);
    let (sy, cy) = fast_sin_cos(proj.yaw);
    let u_front = if sy >= 0.0 { u1 } else { u0 };
    let v_front = if cy >= 0.0 { v1 } else { v0 };
    let side_a = fmt_pts(&[
        proj.project(u_front, v0, 0.0),
        proj.project(u_front, v1, 0.0),
        proj.project(u_front, v1, z1),
        proj.project(u_front, v0, z1),
    ]);
    let side_b = fmt_pts(&[
        proj.project(u0, v_front, 0.0),
        proj.project(u1, v_front, 0.0),
        proj.project(u1, v_front, z1),
        proj.project(u0, v_front, z1),
    ]);
    let top = fmt_pts(&[
        proj.project(u0, v0, z1),
        proj.project(u1, v0, z1),
        proj.project(u1, v1, z1),
        proj.project(u0, v1, z1),
    ]);
    let inset_u = (u1 - u0) * 0.18;
    let inset_v = (v1 - v0) * 0.18;
    let accent = fmt_pts(&[
        proj.project(u0 + inset_u, v0 + inset_v, z1 + 0.01),
        proj.project(u1 - inset_u, v0 + inset_v, z1 + 0.01),
        proj.project(u1 - inset_u, v1 - inset_v, z1 + 0.01),
        proj.project(u0 + inset_u, v1 - inset_v, z1 + 0.01),
    ]);
    let led = proj.project(u_front, (v0 + v1) * 0.5, z1 * 0.80);
    let anchor = proj.project((u0 + u1) * 0.5, v1 + 0.28, 0.0);
    let cw = clamp_f32((node.title.len() as f32) * 7.2 + 24.0, 92.0, 174.0);
    ProjectedBox {
        ring,
        shadow,
        side_a,
        side_b,
        top,
        accent,
        led,
        callout: (
            clamp_f32(anchor.0 - cw * 0.5, 10.0, STAGE_W - cw - 10.0),
            clamp_f32(anchor.1 + 8.0, 10.0, STAGE_H - 42.0),
            cw,
        ),
    }
}

/// Projected conduit polyline layers and directional chevron polygon.
#[derive(Clone, PartialEq)]
pub struct ProjectedRoute {
    pub shadow: String,
    pub base: String,
    pub core: String,
    pub crest: String,
    pub chevron: String,
}

/// Projects an orthogonal conduit route into shadow, base, core, crest, and directional chevron.
#[must_use]
pub fn project_route(route: &RouteGeometry, proj: OrbitProjector) -> ProjectedRoute {
    let mut shad = Vec::with_capacity(route.points.len());
    let mut base = Vec::with_capacity(route.points.len());
    let mut core = Vec::with_capacity(route.points.len());
    let mut crest = Vec::with_capacity(route.points.len());
    for &(uc, vc) in &route.points {
        let (u, v) = (from_centi(uc), from_centi(vc));
        shad.push(proj.project(u + 0.05, v - 0.03, 0.0));
        base.push(proj.project(u, v, 0.024));
        core.push(proj.project(u, v, 0.060));
        crest.push(proj.project(u, v, 0.082));
    }
    let chevron = route_chevron(&core, (f32::from(route.chevron_milli)) * 0.001);
    ProjectedRoute {
        shadow: fmt_pts(&shad),
        base: fmt_pts(&base),
        core: fmt_pts(&core),
        crest: fmt_pts(&crest),
        chevron,
    }
}

fn route_chevron(pts: &[(f32, f32)], t: f32) -> String {
    if pts.len() < 2 {
        return String::new();
    }
    let mut total = 0.0_f32;
    for w in pts.windows(2) {
        let (dx, dy) = (w[1].0 - w[0].0, w[1].1 - w[0].1);
        total += (dx * dx + dy * dy).sqrt();
    }
    let target = total * clamp_f32(t, 0.15, 0.85);
    let mut acc = 0.0_f32;
    let (mut mid, mut dir) = (pts[0], (1.0_f32, 0.0_f32));
    for w in pts.windows(2) {
        let (dx, dy) = (w[1].0 - w[0].0, w[1].1 - w[0].1);
        let seg = (dx * dx + dy * dy).sqrt();
        if seg > 1e-3 && (acc + seg >= target || w[1] == *pts.last().unwrap_or(&w[1])) {
            let frac = clamp_f32((target - acc) / seg, 0.0, 1.0);
            mid = (w[0].0 + dx * frac, w[0].1 + dy * frac);
            dir = (dx / seg, dy / seg);
            break;
        }
        acc += seg;
    }
    let perp = (-dir.1, dir.0);
    let tip = (mid.0 + dir.0 * 6.0, mid.1 + dir.1 * 6.0);
    let back = (mid.0 - dir.0 * 5.2, mid.1 - dir.1 * 5.2);
    fmt_pts(&[
        tip,
        (back.0 + perp.0 * 5.0, back.1 + perp.1 * 5.0),
        (back.0 + dir.0 * 2.0, back.1 + dir.1 * 2.0),
        (back.0 - perp.0 * 5.0, back.1 - perp.1 * 5.0),
    ])
}

/// Kahn topological painter's sort over 3D AABB boxes so nearer prisms draw on top.
#[must_use]
pub fn topological_box_order(boxes: &[BoxGeometry], proj: OrbitProjector) -> Vec<usize> {
    let n = boxes.len();
    if n <= 1 {
        return (0..n).collect();
    }
    let (sy, cy) = fast_sin_cos(proj.yaw);
    let rects: Vec<[f32; 4]> = boxes
        .iter()
        .map(|b| box_screen_rect(b.bounds, proj))
        .collect();
    let depths: Vec<f32> = boxes
        .iter()
        .map(|b| {
            proj.depth(
                from_centi(b.bounds[0] + b.bounds[1]) * 0.5,
                from_centi(b.bounds[2] + b.bounds[3]) * 0.5,
            )
        })
        .collect();
    let mut adj = vec![Vec::new(); n];
    let mut indeg = vec![0usize; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if !(rects[i][0] < rects[j][2]
                && rects[i][2] > rects[j][0]
                && rects[i][1] < rects[j][3]
                && rects[i][3] > rects[j][1])
            {
                continue;
            }
            let ij = aabb_behind(boxes[i].bounds, boxes[j].bounds, sy, cy);
            let ji = aabb_behind(boxes[j].bounds, boxes[i].bounds, sy, cy);
            if ij && !ji {
                adj[i].push(j);
                indeg[j] += 1;
            } else if ji && !ij {
                adj[j].push(i);
                indeg[i] += 1;
            }
        }
    }
    let mut done = vec![false; n];
    let mut out = Vec::with_capacity(n);
    while out.len() < n {
        let next = (0..n)
            .filter(|&i| !done[i] && indeg[i] == 0)
            .min_by(|&a, &b| depths[a].total_cmp(&depths[b]).then_with(|| a.cmp(&b)))
            .or_else(|| {
                (0..n)
                    .filter(|&i| !done[i])
                    .min_by(|&a, &b| depths[a].total_cmp(&depths[b]).then_with(|| a.cmp(&b)))
            });
        let Some(u) = next else { break };
        done[u] = true;
        out.push(u);
        for &v in &adj[u] {
            indeg[v] = indeg[v].saturating_sub(1);
        }
    }
    out
}

fn box_screen_rect(bounds: [i32; 5], proj: OrbitProjector) -> [f32; 4] {
    let [u0, u1, v0, v1, z1] = bounds.map(from_centi);
    let (mut min_x, mut min_y, mut max_x, mut max_y) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
    for u in [u0, u1] {
        for v in [v0, v1] {
            for z in [0.0, z1] {
                let (x, y) = proj.project(u, v, z);
                (min_x, min_y, max_x, max_y) =
                    (min_x.min(x), min_y.min(y), max_x.max(x), max_y.max(y));
            }
        }
    }
    [min_x, min_y, max_x, max_y]
}

fn aabb_behind(a: [i32; 5], b: [i32; 5], sy: f32, cy: f32) -> bool {
    let u_ok = if sy >= 0.0 {
        a[1] <= b[0]
    } else {
        a[0] >= b[1]
    };
    let v_ok = if cy >= 0.0 {
        a[3] <= b[2]
    } else {
        a[2] >= b[3]
    };
    u_ok || v_ok
}
