# `content/`

Public, reviewed Markdown and TOML-frontmatter sources rendered by `kennedy-mosoti-dev` at build and server startup. Every file in this tree passes automated claim-discipline and privacy checks before serving or committing (`CONTRACT.md` §4–§6).

## Layout

| Path | Purpose | Primary Routes |
| --- | --- | --- |
| `identity.md` | Name, positioning headline (`Platform, Reliability & Systems Automation Engineer`), discipline summaries, and six-stage operating pattern | `/`, `/biography` |
| `skills.md` | Production and independent skills, each tied strictly to records of its own evidence class | `/biography` |
| `records/<slug>.md` | Curated career records (`production`, `operational-exposure`, `developing`, `independent`) with explicit `claim_boundary` metadata | `/biography`, `/archive`, `/archive/<slug>` |
| `projects/<slug>.md` | Five open-source Moiric project showcases (`nomos`, `fabrico11y`, `metron`, `jacquard`, `hephaestus`) framed by lifecycle status (`Architecture`, `Released Alpha`, `In Progress`) | `/`, `/projects`, `/projects/<slug>` |
| `wiki/<slug>.md` | Public engineering wiki notes with Mermaid architecture diagrams and explicit cross-links | `/wiki`, `/wiki/<slug>` |
| `blog/<slug>.md` | Public technical blog posts (dated, tagged, published via `/feed.xml` RSS/Atom) | `/blog`, `/blog/<slug>`, `/feed.xml` |

## Rules

1. **Evidence classes are never blended.** Production skills cite only `production` records; independent engineering stays cleanly labeled `independent` (records) or `Architecture` / `Released Alpha` / `In Progress` (projects).
2. **Claim boundaries are explicit.** Every record carrying an outcome, role scope, or numerical figure includes a `claim_boundary` stating what it asserts and what it excludes.
3. **Zero private or NDA content.** No client names, client environment counts/percentages/cadences, client compliance/audit specifics, internal GCP project/ULA IPv6 prefixes, private keys, or personal workstation paths.
4. **Voice and spelling.** Third person or impersonal prose in profile/record/project copy, using American spelling.
