# Security Policy

`kennedy-mosoti-dev` is a public Axum SSR + Leptos Islands web application and static content showcase for `kennedy.mosoti.dev`. It serves public engineering articles, project architecture views, and sanitized biography records with zero server-side user accounts, zero runtime databases, and zero secrets.

## Supported Versions

| Version | Supported |
| --- | --- |
| `main` | Yes |
| Latest `v0.1.x` tag | Yes |
| Earlier tags or forks | No |

## Reporting a Vulnerability or Privacy Leak

**Do not open public issues or pull requests for security vulnerabilities or accidental private/NDA data exposure.**

Report privately through GitHub [Private Vulnerability Reporting](https://github.com/kmosoti/kennedy-mosoti-dev/security/advisories/new) or email `kennedy.rmosoti@gmail.com`.

Include:

- affected route, asset, crate, or deployment unit
- reproduction steps or failing CSP/header/link/privacy check
- impact summary (XSS, path traversal, header bypass, or inadvertent sensitive disclosure)

## Response Expectations

- Acknowledgement within **5 business days**.
- Remediation plan within **14 days** for confirmed vulnerabilities or privacy exposures.
- Coordinated fix and tagged release before public disclosure.

## Scope

In scope:

- Axum HTTP server (`crates/site-server`), Markdown/RSS parsing, static file routing, and CSP/security headers
- Leptos SSR/CSR rendering (`crates/site-app`) and WASM island bundle
- Deployment manifests (`deploy/Caddyfile`, `deploy/kennedy-mosoti-dev.service`)
- Content privacy & claim-boundary guardrails (`CONTRACT.md` §6)
