# Contributing to `kennedy-mosoti-dev`

Thank you for contributing to `kennedy.mosoti.dev`. Contributions should improve correctness, accessibility, performance budgets, or factual accuracy while preserving strict privacy and claim-discipline boundaries.

## Standards & Toolchain

- **Language:** Rust `1.100.0` (Edition 2024), pinned in [`rust-toolchain.toml`](rust-toolchain.toml), with `wasm32-unknown-unknown` installed.
- **No Python, no Node/Three.js:** All rendering, content parsing, RSS generation, and verification gates run in Rust or POSIX shell.
- **Workspace hygiene:** Keep `.rs` files modular (`<= 350` lines), avoid `unsafe` (`#![forbid(unsafe_code)]`), and maintain Hallmark CSS cascade discipline (`@layer`, `@scope`, OKLCH tokens, zero raw color literals).

## Pre-PR Verification

Run all gates before opening or updating a pull request:

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo check -p site-app --no-default-features --features csr --target wasm32-unknown-unknown
```

## Workflow (Single-Branch `main` + Stacked `jj`)

1. Work from a clean Jujutsu (`jj`) stack rooted at `main` using descriptive bookmarks (`stack/<topic>/01-...`).
2. Keep commits small, linear, and focused (one logical layer per commit; avoid squashing away `jj` Change-IDs).
3. Ensure all automated tests — including broken internal link checks, Markdown/RSS parse checks, WCAG contrast checks, WASM budget checks, and privacy/NDA regex guards — pass before review by `@kmosoti`.

## License

All contributions are licensed under the [Apache License, Version 2.0](LICENSE).
