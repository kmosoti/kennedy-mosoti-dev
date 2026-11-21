# AGENTS.md

Operational contract for coding agents working in `kennedy-mosoti-dev`. Human contributors follow the same rules via [CONTRIBUTING.md](CONTRIBUTING.md).

## Mission & Architecture

`kennedy-mosoti-dev` powers `https://kennedy.mosoti.dev` — an interactive **Hallmark × Leptos `0.8.23` Islands/SSR + pure-Rust `iso-diagram` 2.5D Cockpit** showcasing Kennedy Mosoti's five open-source Moiric repositories (**Nomos, FabricO11y, Metron, Jacquard, Hephaestus**), curated career biography/skills/records (`content/`), and a public **Wiki & Blog** (Markdown + Mermaid + RSS/Atom + zero-JS SSR baseline).

Three.js, TypeScript bundlers, and Python scripts are permanently removed.

## Hard Rules

1. **Rust `1.100.0`, Edition 2024, `#![forbid(unsafe_code)]`:** Toolchain is pinned in `rust-toolchain.toml`. Keep `.rs` modules focused and `<= 350` lines (target `<= 250–300`).
2. **ZERO Python anywhere:** No Python in source, verification, asset generation, or CI. Use Rust, Nix, or standard POSIX shell utilities (`sh`, `rg`, `jq`, `curl`).
3. **Zero Three.js / Node bundler bloat:** Interactive 2.5D diagrams render via `Jacquard`'s pure-Rust `iso-diagram` SVG/eDSL engine inside Leptos `0.8.23` `#[island]` components, keeping full zero-JS SSR readability for readers and crawlers.
4. **Privacy & Contract §6 (STRICT):** Never commit or serve client names, client environment figures, compliance/audit specifics, compensation, private assessments, interview material, internal GCP project/ULA IPv6 prefixes, private age/SSH keys, or personal workstation home paths.
5. **Claim & Evidence Discipline:**
   - Career records preserve their four distinct evidence classes (`production`, `operational-exposure`, `developing`, `independent`) and explicit `claim_boundary` statements.
   - Moiric project cards frame status honestly by lifecycle class (`Architecture`, `Released Alpha`, `In Progress`) and never claim unverified benchmarks as live.
   - Positioning headline remains `"Platform, Reliability & Systems Automation Engineer"`.
6. **VCS Workflow (Single-Branch `main` + Stacked `jj`):**
   - Colocated Jujutsu (`jj git init --colocate`). Use linear stacked bookmarks (`stack/<topic>/01-...`).
   - Do not force-push or push directly to `main`; `@kennmosoti-maker` pushes stacked branches for `@kmosoti` review (`CODEOWNERS: * @kmosoti`).

## Verification Commands

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo check -p site-app --no-default-features --features csr --target wasm32-unknown-unknown
```
