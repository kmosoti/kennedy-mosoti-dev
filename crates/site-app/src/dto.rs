//! Lightweight DTOs and compact fixed-point `SceneGeometry` shared between SSR
//! compilation and the hydrated WASM 2.5D orbit island (zero `serde_json` in `wasm32`).

#[cfg(feature = "ssr")]
use std::fmt::Write as _;

/// Single hardware/system node inside a project's 2.5D architectural bay.
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BayNodeDto {
    pub slug: String,
    pub label: String,
    pub role: String,
    pub asset_kind: String,
    pub status: String,
    pub mechanism: String,
    #[serde(skip)]
    pub col: u8,
    #[serde(skip)]
    pub row: u8,
}

/// Directed architectural edge between two bay nodes (used during SSR layout compilation).
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct BayEdgeDto {
    pub from: String,
    pub to: String,
    pub label: String,
}

/// Compact fixed-point (`100 == 1.00` grid unit) floor zone geometry.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(Debug, serde::Serialize, serde::Deserialize))]
pub struct ZoneGeometry {
    pub id: usize,
    pub title: String,
    pub tint: String,
    /// Floor bounds `[u_min, v_min, u_max, v_max]` in centi-units.
    pub bounds: [i32; 4],
}

/// Compact fixed-point 3D prism node geometry compiled from `iso_diagram::geometry::BoxGeom`.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(Debug, serde::Serialize, serde::Deserialize))]
pub struct BoxGeometry {
    pub id: usize,
    pub slug: String,
    pub title: String,
    pub subtitle: String,
    pub asset_kind: String,
    pub tint: String,
    pub status: String,
    /// `[u_min, u_max, v_min, v_max, z_max]` in centi-units.
    pub bounds: [i32; 5],
}

/// Compact fixed-point routed orthogonal conduit compiled from `iso_diagram::geometry::RouteGeom`.
#[derive(Clone, PartialEq, Eq)]
#[cfg_attr(feature = "ssr", derive(Debug, serde::Serialize, serde::Deserialize))]
pub struct RouteGeometry {
    pub id: usize,
    pub style: String,
    pub tint: String,
    /// Normalized chevron parameter in milli-units (`0..=1000`).
    pub chevron_milli: u16,
    /// Orthogonal floor waypoints `(u, v)` in centi-units.
    pub points: Vec<(i32, i32)>,
}

/// Pre-compiled 2.5D scene geometry passed from SSR into `CockpitStageIsland`.
/// Keeps `iso-diagram` layout and orthogonal routing strictly server-side (`ssr` only).
#[derive(Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "ssr", derive(Debug, serde::Serialize, serde::Deserialize))]
pub struct SceneGeometry {
    pub title: String,
    /// Floor grid extent `[u_min, v_min, u_max, v_max]` in centi-units.
    pub grid_extent: [i32; 4],
    pub zones: Vec<ZoneGeometry>,
    pub boxes: Vec<BoxGeometry>,
    pub routes: Vec<RouteGeometry>,
}

impl SceneGeometry {
    /// Encodes `self` into a compact ASCII string (`extent|zones|boxes|routes`) on SSR.
    #[cfg(feature = "ssr")]
    #[must_use]
    pub fn encode(&self) -> String {
        let mut s = String::with_capacity(512);
        let [a, b, c, d] = self.grid_extent;
        let _ = write!(s, "{a},{b},{c},{d}|");
        for (i, z) in self.zones.iter().enumerate() {
            if i > 0 {
                s.push(';');
            }
            let [u0, v0, u1, v1] = z.bounds;
            let _ = write!(s, "{},{u0},{v0},{u1},{v1}", z.id);
        }
        s.push('|');
        for (i, bx) in self.boxes.iter().enumerate() {
            if i > 0 {
                s.push(';');
            }
            let [u0, u1, v0, v1, z1] = bx.bounds;
            let _ = write!(
                s,
                "{},{},{},{u0},{u1},{v0},{v1},{z1}",
                bx.id,
                bx.slug,
                bx.title.len()
            );
        }
        s.push('|');
        for (i, r) in self.routes.iter().enumerate() {
            if i > 0 {
                s.push(';');
            }
            let _ = write!(s, "{},{}", r.id, r.chevron_milli);
            for &(u, v) in &r.points {
                let _ = write!(s, ",{u},{v}");
            }
        }
        s
    }

    /// Decodes a compact ASCII string produced by [`Self::encode`] without `serde_json`.
    #[must_use]
    pub fn decode(raw: &str) -> Self {
        let mut parts = raw.split('|');
        let mut ext_it = parts
            .next()
            .unwrap_or("")
            .split(',')
            .filter_map(|x| x.parse::<i32>().ok());
        let grid_extent = [
            ext_it.next().unwrap_or(0),
            ext_it.next().unwrap_or(0),
            ext_it.next().unwrap_or(600),
            ext_it.next().unwrap_or(400),
        ];
        let zones = parts
            .next()
            .unwrap_or("")
            .split(';')
            .filter(|r| !r.is_empty())
            .filter_map(|row| {
                let mut it = row.split(',');
                Some(ZoneGeometry {
                    id: it.next()?.parse().ok()?,
                    title: String::new(),
                    tint: String::new(),
                    bounds: [
                        it.next()?.parse().ok()?,
                        it.next()?.parse().ok()?,
                        it.next()?.parse().ok()?,
                        it.next()?.parse().ok()?,
                    ],
                })
            })
            .collect();
        let boxes = parts
            .next()
            .unwrap_or("")
            .split(';')
            .filter(|r| !r.is_empty())
            .filter_map(|row| {
                let mut it = row.split(',');
                let id = it.next()?.parse().ok()?;
                let slug = it.next()?.to_owned();
                let tlen: usize = it.next()?.parse().ok()?;
                Some(BoxGeometry {
                    id,
                    slug,
                    title: " ".repeat(tlen),
                    subtitle: String::new(),
                    asset_kind: String::new(),
                    tint: String::new(),
                    status: String::new(),
                    bounds: [
                        it.next()?.parse().ok()?,
                        it.next()?.parse().ok()?,
                        it.next()?.parse().ok()?,
                        it.next()?.parse().ok()?,
                        it.next()?.parse().ok()?,
                    ],
                })
            })
            .collect();
        let routes = parts
            .next()
            .unwrap_or("")
            .split(';')
            .filter(|r| !r.is_empty())
            .filter_map(|row| {
                let mut it = row.split(',');
                let id = it.next()?.parse().ok()?;
                let chevron_milli = it.next()?.parse().ok()?;
                let mut points = Vec::new();
                while let (Some(us), Some(vs)) = (it.next(), it.next()) {
                    if let (Ok(u), Ok(v)) = (us.parse(), vs.parse()) {
                        points.push((u, v));
                    }
                }
                Some(RouteGeometry {
                    id,
                    style: String::new(),
                    tint: String::new(),
                    chevron_milli,
                    points,
                })
            })
            .collect();
        Self {
            title: String::new(),
            grid_extent,
            zones,
            boxes,
            routes,
        }
    }
}

/// Interactive 2.5D bay model for one Moiric open-source project (SSR compilation).
#[cfg(feature = "ssr")]
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ProjectBayDto {
    pub slug: String,
    pub name: String,
    pub tagline: String,
    pub status: String,
    pub status_tone: String,
    pub version: String,
    pub repo: String,
    pub summary: String,
    pub claim_boundary: String,
    pub nodes: Vec<BayNodeDto>,
    #[serde(skip)]
    pub edges: Vec<BayEdgeDto>,
    pub scene: SceneGeometry,
}
