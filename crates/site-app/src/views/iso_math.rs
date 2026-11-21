//! Pure-math 2:1 orthographic projector and zero-`core::fmt` integer-tenths SVG formatters.

use crate::dto::SceneGeometry;

/// Logical SVG coordinate width of the 2.5D stage canvas.
pub const STAGE_W: f32 = 960.0;
/// Logical SVG coordinate height of the 2.5D stage canvas.
pub const STAGE_H: f32 = 540.0;

#[inline]
#[must_use]
pub fn from_centi(v: i32) -> f32 {
    (v as f32) * 0.01
}

/// Panic-free `f32` clamp that avoids pulling `core::fmt::float` into `wasm32`.
#[inline]
#[must_use]
pub fn clamp_f32(x: f32, lo: f32, hi: f32) -> f32 {
    if x < lo {
        lo
    } else if x > hi {
        hi
    } else {
        x
    }
}

/// Fast branchless `(sin, cos)` approximation avoiding `libm` trig tables in `wasm32`.
#[inline]
#[must_use]
pub fn fast_sin_cos(rad: f32) -> (f32, f32) {
    fn sin_approx(r: f32) -> f32 {
        const PI: f32 = std::f32::consts::PI;
        const TAU: f32 = std::f32::consts::TAU;
        let x = r - (r / TAU).round() * TAU;
        let b = 4.0 / PI;
        let c = -4.0 / (PI * PI);
        let y = b * x + c * x * x.abs();
        0.225 * (y * y.abs() - y) + y
    }
    (
        sin_approx(rad),
        sin_approx(rad + std::f32::consts::FRAC_PI_2),
    )
}

/// Pure-math orthographic 2.5D orbit projector shared across SSR and `wasm32`.
#[derive(Clone, Copy, PartialEq)]
pub struct OrbitProjector {
    pub origin_x: f32,
    pub origin_y: f32,
    pub tile_w: f32,
    pub scale_z: f32,
    pub yaw: f32,
    pub pitch: f32,
    pub pivot_u: f32,
    pub pivot_v: f32,
}

impl OrbitProjector {
    /// Fits `scene` inside `STAGE_W x STAGE_H` and applies `(yaw_deg, pitch_deg, flat)`.
    #[must_use]
    pub fn fit_orbit(scene: &SceneGeometry, yaw_deg: f32, pitch_deg: f32, flat: bool) -> Self {
        let [u0, v0, u1, v1] = scene.grid_extent.map(from_centi);
        let span_u = (u1 - u0).max(4.0);
        let span_v = (v1 - v0).max(4.0);
        let span_x = (span_u + span_v).max(6.0);
        let span_y = ((span_u + span_v) * 0.5 + 3.0).max(4.5);
        let tile_w = clamp_f32(
            ((STAGE_W - 84.0) / span_x).min((STAGE_H - 92.0) / span_y),
            12.0,
            62.0,
        );
        let (yaw, pitch, scale_z) = if flat {
            (0.0, 0.72, tile_w * 0.14)
        } else {
            (
                yaw_deg.to_radians().rem_euclid(std::f32::consts::TAU),
                clamp_f32(pitch_deg / 70.528, 0.30, 0.72),
                tile_w * 0.92,
            )
        };
        Self {
            origin_x: STAGE_W * 0.5,
            origin_y: STAGE_H * 0.50,
            tile_w,
            scale_z,
            yaw,
            pitch,
            pivot_u: (u0 + u1) * 0.5,
            pivot_v: (v0 + v1) * 0.5,
        }
    }

    /// Projects campus `(u, v, z)` into SVG `(x, y)` coordinates.
    #[must_use]
    pub fn project(self, u: f32, v: f32, z: f32) -> (f32, f32) {
        let (du, dv) = (u - self.pivot_u, v - self.pivot_v);
        let (sy, cy) = fast_sin_cos(self.yaw);
        let rx = (du * cy - dv * sy) * std::f32::consts::SQRT_2;
        let ry = (du * sy + dv * cy) * std::f32::consts::SQRT_2;
        let z_fac = ((1.0 - self.pitch * self.pitch).max(0.18) / 0.75).sqrt();
        (
            self.origin_x + rx * self.tile_w,
            self.origin_y + ry * (self.tile_w * self.pitch) - z * (self.scale_z * z_fac),
        )
    }

    /// Projects centi-unit `(u_c, v_c)` at elevation `z`.
    #[must_use]
    pub fn project_c(self, u_c: i32, v_c: i32, z: f32) -> (f32, f32) {
        self.project(from_centi(u_c), from_centi(v_c), z)
    }

    /// Camera depth along look axis (smaller = further back).
    #[must_use]
    pub fn depth(self, u: f32, v: f32) -> f32 {
        let (sy, cy) = fast_sin_cos(self.yaw);
        (u - self.pivot_u) * sy + (v - self.pivot_v) * cy
    }
}

/// Appends an unsigned integer in decimal without linking `core::fmt`.
pub fn push_u32(out: &mut String, mut n: u32) {
    let mut buf = [0u8; 10];
    let mut i = 10;
    loop {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        if n == 0 {
            break;
        }
    }
    if let Ok(s) = std::str::from_utf8(&buf[i..]) {
        out.push_str(s);
    }
}

/// Appends a signed integer in decimal without linking `core::fmt`.
pub fn push_i32(out: &mut String, n: i32) {
    if n < 0 {
        out.push('-');
    }
    push_u32(out, n.unsigned_abs());
}

/// Appends `v` formatted to 1 decimal place using pure integer arithmetic.
pub fn push_t1(out: &mut String, v: f32) {
    let t = (v * 10.0).round() as i32;
    if t < 0 {
        out.push('-');
    }
    let abs = t.unsigned_abs();
    push_u32(out, abs / 10);
    out.push('.');
    out.push((b'0' + (abs % 10) as u8) as char);
}

/// Formats `v` to 1 decimal place without linking `core::fmt`.
#[must_use]
pub fn fmt_t1(v: f32) -> String {
    let mut s = String::with_capacity(8);
    push_t1(&mut s, v);
    s
}

/// Appends `v` formatted to 2 decimal places using pure integer arithmetic.
pub fn push_t2(out: &mut String, v: f32) {
    let c = (v * 100.0).round() as i32;
    if c < 0 {
        out.push('-');
    }
    let abs = c.unsigned_abs();
    let frac = abs % 100;
    push_u32(out, abs / 100);
    out.push('.');
    out.push((b'0' + (frac / 10) as u8) as char);
    out.push((b'0' + (frac % 10) as u8) as char);
}

/// Formats the stage caption without pulling `core::fmt`.
#[must_use]
pub fn format_stage_caption(yaw: f32, pitch: f32, zoom: f32, is_2d: bool) -> String {
    let mut out = String::with_capacity(64);
    out.push_str("Orbit ");
    push_i32(&mut out, yaw.round() as i32);
    out.push_str("° · Pitch ");
    push_i32(&mut out, pitch.round() as i32);
    out.push_str("° · Zoom ");
    push_t2(&mut out, zoom);
    out.push_str(if is_2d {
        "× · 2D Schematic"
    } else {
        "× · 2.5D Isometric · 0 Hz Idle"
    });
    out
}

/// Formats SVG polygon/polyline coordinate pairs cleanly via integer tenths.
#[must_use]
pub fn fmt_pts(pts: &[(f32, f32)]) -> String {
    let mut out = String::with_capacity(pts.len() * 14);
    for (idx, &(x, y)) in pts.iter().enumerate() {
        if idx > 0 {
            out.push(' ');
        }
        push_t1(&mut out, x);
        out.push(',');
        push_t1(&mut out, y);
    }
    out
}

/// Computes an O(1) centered SVG `viewBox` string for `zoom`.
#[must_use]
pub fn zoom_viewbox(zoom: f32) -> String {
    let z = clamp_f32(zoom, 0.6, 2.0);
    let (vw, vh) = (STAGE_W / z, STAGE_H / z);
    let mut out = String::with_capacity(32);
    push_t1(&mut out, (STAGE_W - vw) * 0.5);
    out.push(' ');
    push_t1(&mut out, (STAGE_H - vh) * 0.5);
    out.push(' ');
    push_t1(&mut out, vw);
    out.push(' ');
    push_t1(&mut out, vh);
    out
}

/// Maps tint tag string to Hallmark CSS custom property.
#[must_use]
pub fn tint_var(tint: &str) -> &'static str {
    match tint {
        "pine" => "var(--iso-tint-pine)",
        "amber" => "var(--iso-tint-amber)",
        "crimson" => "var(--iso-tint-crimson)",
        "plum" => "var(--iso-tint-plum)",
        "sage" => "var(--iso-tint-sage)",
        "slate" => "var(--iso-tint-slate)",
        _ => "var(--iso-tint-cobalt)",
    }
}
