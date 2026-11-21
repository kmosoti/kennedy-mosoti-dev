//! Architectural 2.5D bay topology definitions for the 5 Moiric open-source projects.

use crate::dto::{BayEdgeDto, BayNodeDto, ProjectBayDto};
use crate::views::iso_model::compile_bay_scene;
use site_content::SiteContent;

/// Builds the 5 Moiric project bay DTOs with pre-compiled `SceneGeometry` for SSR and WASM islands.
pub fn build_project_bays(content: &SiteContent) -> Vec<ProjectBayDto> {
    content
        .projects
        .iter()
        .map(|p| {
            let (nodes, edges) = bay_topology(&p.slug);
            let scene = compile_bay_scene(&p.slug, &p.name, &nodes, &edges);
            ProjectBayDto {
                slug: p.slug.clone(),
                name: p.name.clone(),
                tagline: p.tagline.clone(),
                status: p.status.label().to_owned(),
                status_tone: p.status.tone().to_owned(),
                version: p.version.clone(),
                repo: p.repo.clone(),
                summary: p.summary.clone(),
                claim_boundary: p.claim_boundary.clone(),
                nodes,
                edges,
                scene,
            }
        })
        .collect()
}

fn bay_topology(slug: &str) -> (Vec<BayNodeDto>, Vec<BayEdgeDto>) {
    match slug {
        "nomos" => (
            vec![
                node(
                    "canon",
                    "Canon",
                    "Desired State",
                    "Lock",
                    "Healthy",
                    "Compiled host specification",
                    (0, 0),
                ),
                node(
                    "core",
                    "nomos-core",
                    "Pure Planner",
                    "Chip",
                    "Healthy",
                    "no_std Variance & Warp DAG",
                    (1, 0),
                ),
                node(
                    "cell",
                    "Cell",
                    "Host Substrate",
                    "Rack",
                    "Healthy",
                    "Applies & re-observes Linux state",
                    (2, 0),
                ),
                node(
                    "loom",
                    "Loom",
                    "Fleet Control",
                    "Balancer",
                    "Active",
                    "Staged rollouts & generation fencing",
                    (2, 1),
                ),
            ],
            vec![
                edge("canon", "core", "Compare"),
                edge("core", "cell", "Warp Plan"),
                edge("loom", "cell", "WireGuard/QUIC"),
            ],
        ),
        "fabrico11y" => (
            vec![
                node(
                    "probes",
                    "Linux Probes",
                    "Cgroup & Proc",
                    "Chip",
                    "Healthy",
                    "Attributes CPU, memory, and disk pressure",
                    (0, 0),
                ),
                node(
                    "spindle",
                    "Spindle",
                    "Custody Ledger",
                    "Drum",
                    "Healthy",
                    "WAL receipt custody before query visibility",
                    (1, 0),
                ),
                node(
                    "server",
                    "Fabric Server",
                    "Canonical Store",
                    "Rack",
                    "Healthy",
                    "Per-origin projection isolation",
                    (2, 0),
                ),
                node(
                    "query",
                    "fabric query",
                    "Projection CLI",
                    "Laptop",
                    "Healthy",
                    "Reads verified watermarks & memory blame",
                    (3, 0),
                ),
            ],
            vec![
                edge("probes", "spindle", "Receipts"),
                edge("spindle", "server", "Custody Forward"),
                edge("server", "query", "Projection"),
            ],
        ),
        "metron" => (
            vec![
                node(
                    "provider",
                    "ModelProvider",
                    "Proposal Stream",
                    "Cloud",
                    "Active",
                    "Emits untrusted stochastic proposals",
                    (0, 0),
                ),
                node(
                    "harness",
                    "metron-harness",
                    "Authority & SQLite",
                    "Lock",
                    "Active",
                    "Enforces budgets & ResourceReceipt journal",
                    (1, 0),
                ),
                node(
                    "lane-a",
                    "Wasmtime AOT",
                    "Lane A (Zero-JIT)",
                    "Pod",
                    "Active",
                    "Sealed .cwasm + fuel & memory limits",
                    (2, 0),
                ),
                node(
                    "lane-b",
                    "Landlock + Seccomp",
                    "Lane B Kernel Jail",
                    "Firewall",
                    "Active",
                    "Direct memfd exec without bwrap",
                    (2, 1),
                ),
            ],
            vec![
                edge("provider", "harness", "Propose"),
                edge("harness", "lane-a", "Admit WASM"),
                edge("harness", "lane-b", "Admit Binary"),
            ],
        ),
        "jacquard" => (
            vec![
                node(
                    "draft",
                    "Draft",
                    "Intent & Budgets",
                    "Laptop",
                    "Active",
                    "Design rules & OKLCH tokens",
                    (0, 0),
                ),
                node(
                    "iso",
                    "iso-diagram",
                    "2.5D SVG Engine",
                    "Chip",
                    "Healthy",
                    "Pure-Rust hardware sprites & Manhattan router",
                    (1, 0),
                ),
                node(
                    "leptos",
                    "cockpit-leptos",
                    "Leptos 0.8 Islands",
                    "Pod",
                    "Active",
                    "Zero-JS SSR + lightweight island hydration",
                    (2, 0),
                ),
                node(
                    "checks",
                    "Checks",
                    "Executable Gates",
                    "Firewall",
                    "Active",
                    "WCAG contrast, slop-lint & WASM byte receipts",
                    (3, 0),
                ),
            ],
            vec![
                edge("draft", "iso", "Compile Scene"),
                edge("iso", "leptos", "Render SVG"),
                edge("leptos", "checks", "Emit Receipt"),
            ],
        ),
        _ => (
            vec![
                node(
                    "jj",
                    "jj Client",
                    "jj-lib 0.47",
                    "Laptop",
                    "Active",
                    "Content-addressed workspaces & OpLogs",
                    (0, 0),
                ),
                node(
                    "quic",
                    "hephaestus-server",
                    "Idempotent QUIC",
                    "Router",
                    "Active",
                    "Tracks (repo, stream, blake3) tuples",
                    (1, 0),
                ),
                node(
                    "logs",
                    "Decision & Obs Logs",
                    "Orthogonal Streams",
                    "Drum",
                    "Active",
                    "Hash-chained decisions & verifier receipts",
                    (2, 0),
                ),
                node(
                    "cas",
                    "object_store",
                    "GRNB0002 Zstd CAS",
                    "Crate",
                    "Active",
                    "LocalFS, S3/R2, GCS, and Drive adapters",
                    (3, 0),
                ),
            ],
            vec![
                edge("jj", "quic", "QUIC Stream"),
                edge("quic", "logs", "WAL Commit"),
                edge("quic", "cas", "Packed CAS"),
            ],
        ),
    }
}

fn node(
    slug: &str,
    label: &str,
    role: &str,
    asset_kind: &str,
    status: &str,
    mechanism: &str,
    pos: (u8, u8),
) -> BayNodeDto {
    BayNodeDto {
        slug: slug.into(),
        label: label.into(),
        role: role.into(),
        asset_kind: asset_kind.into(),
        status: status.into(),
        mechanism: mechanism.into(),
        col: pos.0,
        row: pos.1,
    }
}

fn edge(from: &str, to: &str, label: &str) -> BayEdgeDto {
    BayEdgeDto {
        from: from.into(),
        to: to.into(),
        label: label.into(),
    }
}
