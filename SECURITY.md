# Security Policy

`kennedy-mosoti-dev` is a public Axum SSR + Leptos Islands web application and static content showcase for `kennedy.mosoti.dev`. It serves public engineering articles, project architecture views, and sanitized biography records with zero server-side user accounts, zero runtime databases, and zero secrets.

## Supported Versions

| Version | Supported |
| --- | --- |
| `main` | Yes |
| `v0.1.1-alpha.1` (latest `v0.1.x`) | Yes |
| `v0.1.0-alpha.1` & earlier tags/forks | No (superseded) |

## Release & Runtime Hardening Controls (`v0.1.1-alpha.1+`)

- **Release ancestry & version binding (`.github/workflows/release.yml`):** Both `build` and `publish` jobs run full-history checkout (`fetch-depth: 0`), fetch protected `origin/main`, require `git merge-base --is-ancestor "${GITHUB_SHA}" refs/remotes/origin/main`, verify `${GITHUB_REF_NAME#v}` matches `[workspace.package] version` in `Cargo.toml`, pin every action to a 40-hex commit SHA, and verify `SHA256SUMS` + `RELEASE-EVIDENCE.json` binding prior to provenance attestation and release creation.
- **Dynamic wiki search bounds (`/wiki`, `/wiki/search`):** Raw query strings (`<= 256` bytes) and decoded `q` parameters (`<= 128` bytes, `<= 16` terms, valid UTF-8) are validated on the stack before heap allocation, returning `400 Bad Request` with full CSP/security headers. SSR `/wiki?q=...` reuses the startup-built `WikiSearchIndex`, skips BLAKE3 `ETag` hashing on dynamic `no-cache` responses, and enforces bounded concurrency and execution timeouts (`503` / `504`).
- **Complete content & privacy validation (`CONTRACT.md` §5–§6):** `validate_all` scans canonical `serde_json` serialization of every public content model field, plus generated Atom/RSS feeds, sitemap XML, and client search JSON. Privacy checks use irreversible domain-separated `blake3` digests (`km-privacy-v1:`) and generic structural detectors (`/home/<user>`, private IPv4 and IPv6 ULA ranges, PEM/SSH/age private-key headers and tokens) with redacted errors (`{loc}: matched prohibited privacy rule`).

## Reporting a Vulnerability or Privacy Leak

**Do not open public issues or pull requests for security vulnerabilities or accidental private/NDA data exposure.**

Report privately through GitHub [Private Vulnerability Reporting](https://github.com/kmosoti/kennedy-mosoti-dev/security/advisories/new) or email `kennedy.rmosoti@gmail.com`.

Include:

- affected route, asset, crate, or deployment unit
- reproduction steps or failing CSP/header/link/privacy check
- impact summary (XSS, path traversal, header bypass, DoS, or inadvertent sensitive disclosure)

## Response Expectations

- Acknowledgement within **5 business days**.
- Remediation plan within **14 days** for confirmed vulnerabilities or privacy exposures.
- Coordinated fix and tagged release before public disclosure.

## Scope

In scope:

- Axum HTTP server (`crates/site-server`), Markdown/RSS parsing, static file routing, dynamic search bounds, and CSP/security headers
- Leptos SSR/CSR rendering (`crates/site-app`) and WASM island bundle
- Release workflow gates (`.github/workflows/release.yml`, `scripts/build-bundle.sh`)
- Deployment manifests (`deploy/Caddyfile`, `deploy/kennedy-mosoti-dev.service`)
- Content privacy & claim-boundary guardrails (`CONTRACT.md` §5–§6)
