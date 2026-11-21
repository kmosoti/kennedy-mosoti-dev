//! SSR-only translation from `ProjectBayDto` topology into pre-compiled `SceneGeometry`
//! via `Jacquard`'s `iso-diagram` (feeding explicit `(col, row)` placement hints).

use crate::dto::{BayEdgeDto, BayNodeDto, BoxGeometry, RouteGeometry, SceneGeometry, ZoneGeometry};
use iso_diagram::builder::DiagramBuilder;
use iso_diagram::geometry::compile_scene_geometry;
use iso_diagram::layout::compile_layout;
use iso_diagram::model::{Asset, Diagram, EdgeStyle, NodeStatus, Tint, ZoneStyle};
use std::collections::HashMap;

/// Compiles a project bay's nodes and edges into compact fixed-point [`SceneGeometry`] on SSR.
pub fn compile_bay_scene(
    slug: &str,
    name: &str,
    nodes: &[BayNodeDto],
    edges: &[BayEdgeDto],
) -> SceneGeometry {
    let diagram = bay_to_diagram(slug, name, nodes, edges);
    let layout = compile_layout(&diagram);
    let raw = compile_scene_geometry(&layout);

    SceneGeometry {
        title: raw.title,
        grid_extent: [
            to_centi(raw.grid_u_min),
            to_centi(raw.grid_v_min),
            to_centi(raw.grid_u_max),
            to_centi(raw.grid_v_max),
        ],
        zones: raw
            .zones
            .into_iter()
            .map(|z| ZoneGeometry {
                id: z.id,
                title: z.title,
                tint: z.tint,
                bounds: [
                    to_centi(z.u_min),
                    to_centi(z.v_min),
                    to_centi(z.u_max),
                    to_centi(z.v_max),
                ],
            })
            .collect(),
        boxes: raw
            .boxes
            .into_iter()
            .map(|b| BoxGeometry {
                id: b.id,
                slug: b.slug,
                title: b.title,
                subtitle: b.subtitle.unwrap_or_default(),
                asset_kind: b.asset_kind,
                tint: b.tint,
                status: b.status,
                bounds: [
                    to_centi(b.aabb.u_min),
                    to_centi(b.aabb.u_max),
                    to_centi(b.aabb.v_min),
                    to_centi(b.aabb.v_max),
                    to_centi(b.aabb.z_max),
                ],
            })
            .collect(),
        routes: raw
            .routes
            .into_iter()
            .map(|r| RouteGeometry {
                id: r.id,
                style: r.style,
                tint: r.tint,
                chevron_milli: (r.chevron_t.clamp(0.0, 1.0) * 1000.0).round() as u16,
                points: r
                    .points
                    .into_iter()
                    .map(|(u, v)| (to_centi(u), to_centi(v)))
                    .collect(),
            })
            .collect(),
    }
}

/// Converts bay metadata, explicit `(col, row)` node coordinates, and edges into an `iso_diagram::Diagram`.
pub fn bay_to_diagram(
    slug: &str,
    name: &str,
    bay_nodes: &[BayNodeDto],
    bay_edges: &[BayEdgeDto],
) -> Diagram {
    let mut b = DiagramBuilder::new(name);
    let (zone_title, zone_tint) = match slug {
        "nomos" => ("Host & Fleet Convergence", Tint::Pine),
        "fabrico11y" => ("Spindle Custody & Telemetry", Tint::Cobalt),
        "metron" => ("Two-Lane Kernel & WASM Jail", Tint::Amber),
        "jacquard" => ("Evidence-Gated UI Loom", Tint::Plum),
        _ => ("Authoritative VCS & QUIC CAS", Tint::Sage),
    };
    let z_main = b.add_zone(slug, zone_title, ZoneStyle::Solid, zone_tint, None);

    let mut nodes = HashMap::with_capacity(bay_nodes.len());
    for n in bay_nodes {
        let asset = map_asset(&n.asset_kind);
        let status = match n.status.as_str() {
            "Healthy" => NodeStatus::Healthy,
            "Active" => NodeStatus::Active,
            "Degraded" => NodeStatus::Degraded,
            "Failed" => NodeStatus::Failed,
            _ => NodeStatus::Pending,
        };
        let nid = b
            .insert_node(&n.slug, asset, &n.label, Some(z_main))
            .subtitle(&n.role)
            .tint(zone_tint)
            .status(status)
            .at_grid(usize::from(n.col), usize::from(n.row))
            .id();
        nodes.insert(n.slug.as_str(), nid);
    }

    for e in bay_edges {
        if let (Some(&from), Some(&to)) = (nodes.get(e.from.as_str()), nodes.get(e.to.as_str())) {
            let style = if e.label.contains("QUIC") || e.label.contains("WireGuard") {
                EdgeStyle::Dashed
            } else {
                EdgeStyle::Solid
            };
            let _ = b
                .edge(from, to)
                .style(style)
                .tint(zone_tint)
                .label(&e.label)
                .id();
        }
    }
    b.build()
}

fn to_centi(val: f32) -> i32 {
    (val * 100.0).round() as i32
}

fn map_asset(kind: &str) -> Asset {
    match kind {
        "Rack" => Asset::Rack {
            blades: 3,
            pedestal: true,
        },
        "Drum" => Asset::Drum { rings: 3 },
        "Crate" => Asset::Crate,
        "Firewall" => Asset::Firewall,
        "Switch" => Asset::Switch { ports: 8 },
        "Router" => Asset::Router,
        "Laptop" => Asset::Laptop,
        "Lock" => Asset::Lock,
        "Chip" => Asset::Chip,
        "Cloud" => Asset::Cloud,
        "Balancer" => Asset::Balancer,
        _ => Asset::Pod,
    }
}
