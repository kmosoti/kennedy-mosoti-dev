# kennedy-mosoti-dev

Personal engineering cockpit, project showcase, biography, and public Wiki/Blog for [kennedy.mosoti.dev](https://kennedy.mosoti.dev) — built in Rust `1.100.0` (Edition 2024) on **Jacquard** (`iso-diagram`, `cockpit-wiki`, `cockpit-leptos`) and **Hallmark × Leptos `0.8.23` Islands/SSR** + **Axum**.

## Highlights

- **Pure-Rust 2.5D Architecture Bays (`iso-diagram`):** Interactive 2.5D isometric diagrams with orbit/zoom/2D schematic fallback for **Nomos**, **FabricO11y**, **Metron**, **Jacquard**, and **Hephaestus** — zero Three.js and zero WebGL bloat.
- **Zero-JS SSR Baseline + Leptos Islands:** Every page (`Home/Cockpit`, `Projects`, `Wiki`, `Blog`, `Biography`, `Archive`, `/feed.xml`) renders complete semantic HTML on the server while hydrating only interactive cockpit islands on `wasm32-unknown-unknown`.
- **Claim Discipline & Privacy (`CONTRACT.md`):** Non-blended evidence classes (`production`, `operational-exposure`, `developing`, `independent`), explicit project lifecycle statuses (`Released Alpha`, `In Progress`, `Architecture`), and automated privacy/NDA checks.

## Repository Layout

- `content/` — Sanitized biography (`identity.md`, `skills.md`), career `records/`, curated Moiric `projects/`, public `wiki/` notes, and `blog/` posts.
- `public/` — Self-hosted variable WOFF2 fonts, icons, `manifest.webmanifest`, and `robots.txt`.
- `CONTRACT.md` & `budgets.toml` — Site narrative, privacy, accessibility (`390×844` + WCAG 2.2 AA), and payload budget rules.

## License

Licensed under the [Apache License, Version 2.0](LICENSE). See [NOTICE](NOTICE).
