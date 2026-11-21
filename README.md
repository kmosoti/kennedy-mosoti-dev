# kennedy-mosoti-dev

Personal engineering cockpit, project showcase, biography, and public Wiki/Blog for [kennedy.mosoti.dev](https://kennedy.mosoti.dev) — built in **Rust `1.100.0`** (Edition 2024) on **Jacquard** (`iso-diagram`, `cockpit-wiki`) and **Hallmark × Leptos `0.8.23` Islands/SSR** + **Axum**.

## Architecture

- **Pure-Rust 2.5D Architecture Bays (`iso-diagram`):** Interactive 2.5D isometric diagrams with procedural hardware sprites, Manhattan conduit routing, 360-degree orbit, zoom, and 2D schematic fallback for **Nomos**, **FabricO11y**, **Metron**, **Jacquard**, and **Hephaestus** — zero Three.js and zero WebGL.
- **Zero-JS SSR Baseline + Leptos Islands:** Every page (`Home / Cockpit`, `Projects`, `Wiki`, `Blog`, `Biography`, `Archive`, `/feed.xml`, `/rss.xml`, `/sitemap.xml`) renders complete semantic HTML and SVG on the server. Only interactive cockpit controls hydrate on `wasm32-unknown-unknown`.
- **Claim Discipline & Privacy (`CONTRACT.md` §4–§6):** Non-blended evidence classes (`production`, `operational-exposure`, `developing`, `independent`), honest project lifecycle tags (`Released Alpha`, `In Progress`, `Architecture`), and automated privacy/NDA checks.

## Repository Layout

| Path | Purpose |
| --- | --- |
| `content/` | Sanitized biography (`identity.md`, `skills.md`), 20 `records/`, 5 Moiric `projects/`, public `wiki/` notes, and `blog/` posts |
| `crates/site-content/` | Path-safe loader, `cockpit-wiki` Markdown/Mermaid/Atom/RSS renderer, and Contract §4–§6 validator |
| `crates/site-app/` | Hallmark × Leptos `0.8.23` `#[island]` 2.5D cockpit, editorial SSR views, and WCAG contrast verification |
| `crates/site-server/` | Axum + Tower Brotli/Gzip server with strict CSP headers, `/health`, `/feed.xml`, `/rss.xml`, and automated crawl/privacy gates |
| `style/` | Hallmark OKLCH light (`Atelier-Draft`) / dark (`Oxide-Monolith`) `@layer` + `@scope` CSS cascade (`390×844` mobile reflow) |
| `deploy/` | Hardened `systemd` service unit, env example, and Caddy reverse proxy config |
| `scripts/build-bundle.sh` | Pure POSIX `sh` release bundler producing tarballs, `SHA256SUMS`, and `RELEASE-EVIDENCE.json` |

## Local Development & Verification

```sh
# 1. Start the local SSR server (http://127.0.0.1:3000)
cargo run -p site-server --bin site-server

# 2. Run all verification gates
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo check -p site-app --no-default-features --features csr --target wasm32-unknown-unknown --locked
cargo check -p site-app --no-default-features --features hydrate --target wasm32-unknown-unknown --locked

# 3. Build optimized WASM islands + server release bundle
cargo build --profile wasm-release --locked -p site-app --no-default-features --features hydrate --target wasm32-unknown-unknown
sh scripts/build-bundle.sh 0.1.0-alpha.1
```

> **Note (Pinned Jacquard Crates):** [`Cargo.toml`](Cargo.toml) pins `iso-diagram` and `cockpit-wiki` directly to `https://github.com/kmosoti/Jacquard.git` at verified revision `b2c837c68ade29a485969c67c2c54cc5a9126b8b`.

## Deployment (`systemd` + Caddy)

1. Unpack the release bundle under `/opt/kennedy-mosoti-dev/releases/<version>/` and point `/opt/kennedy-mosoti-dev/current` to it.
2. Install [`deploy/kennedy-mosoti-dev.env.example`](deploy/kennedy-mosoti-dev.env.example) to `/etc/kennedy-mosoti-dev/site.env` and [`deploy/kennedy-mosoti-dev.service`](deploy/kennedy-mosoti-dev.service) to `/etc/systemd/system/kennedy-mosoti-dev.service`.
3. Import [`deploy/Caddyfile`](deploy/Caddyfile) in `/etc/caddy/Caddyfile`, validate (`caddy validate`), and reload Caddy and `systemd`.

## License

Licensed under the [Apache License, Version 2.0](LICENSE). See [NOTICE](NOTICE) and [SECURITY.md](SECURITY.md).
