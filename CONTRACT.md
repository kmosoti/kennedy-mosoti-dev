# `kennedy.mosoti.dev`: Site Contract

This document defines the behavioral, narrative, privacy, accessibility, and performance contract for `kennedy-mosoti-dev` — an interactive Hallmark × Leptos `0.8.23` Islands/SSR + pure-Rust `iso-diagram` 2.5D cockpit showcasing Kennedy Mosoti's open-source Moiric work, career records, and public Wiki/Blog.

## Narrative & Mechanism

1. **Architecture over decoration.** Every 2.5D isometric bay, node, conduit, and inspector field models a documented system mechanism across the five Moiric repositories (**Nomos**, **FabricO11y**, **Metron**, **Jacquard**, **Hephaestus**) or the core career loop. Decorative elements without technical meaning are omitted.
2. **Result, consequence, evidence, mechanism.** Inspector drawers and records present what the system does, why it matters, the supporting evidence (and boundary), and the underlying mechanism.
3. **Request success is not system state.** System visualizations and project architecture models distinguish accepted proposals from verified postconditions.

## Claims & Evidence Discipline

4. **Non-blended evidence classes.**
   - Career records retain four separate evidence classes: `production`, `operational-exposure`, `developing`, and `independent`. Every case study, role, and numerical statement carries an explicit `claim_boundary`.
   - Open-source project showcases state their verified lifecycle class explicitly (`Architecture`, `Released Alpha`, `In Progress`) and link directly to their source repositories. Unverified benchmarks are never presented as live production measurements.
5. **Positioning.** The headline is `"Platform, Reliability & Systems Automation Engineer"`. Never `"Splunk Engineer"` or `"senior SRE"`.
6. **Private material never ships (Strict).** Zero client names, zero client environment figures, zero client security/compliance posture indicators, zero compensation/assessment/interview material, zero internal GCP project/ULA IPv6 prefixes, zero private age/SSH keys, and zero personal workstation home paths.

## Visual, Zero-JS SSR & Accessibility Constraints

7. **Hallmark Dual-Material Aesthetic.** Warm archival paper daylight (`Atelier-Draft`) paired with deep neutral night (`Oxide-Monolith`) via OKLCH tokens, CSS `@layer` cascade, `@scope`, subgrid KPI strips, native `<dialog closedby="any">` / `[popover]`, and `0 Hz` settled idle state when no user interaction is active.
8. **Zero-JS SSR baseline + Leptos Islands.** All pages (`Home/Cockpit`, `Projects`, `Wiki`, `Blog`, `Biography`, `Archive`, `/feed.xml`) render full semantic HTML on the server with zero JavaScript required for reading, navigation, search bots, or screen readers. Only interactive 2.5D cockpit controls hydrate as Leptos `#[island]` components on `wasm32`.
9. **Mobile & Keyboard Completeness (`390×844` and Desktop).** Every route, project bay, wiki article, blog post, and biography record is reachable at `390×844` with zero horizontal scroll, `>= 24×24 CSS px` interactive targets, WCAG 2.2 AA contrast (`>= 4.5:1` text, `>= 3:1` UI/SVG), and full keyboard navigation (`Tab`, arrow orbit, `+`/`-`/`0` zoom, `/` or `k` shortcut hints).

## Performance

10. **Payload budgets (`budgets.toml`).** Compressed WASM island bundle, CSS, variable WOFF2 fonts, and SSR HTML stay within `budgets.toml` ceilings, verified automatically in Rust integration tests.
