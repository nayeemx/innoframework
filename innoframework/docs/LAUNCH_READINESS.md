# Innoframework Launch Readiness Checklist

Purpose: a go/no-go checklist for when this project is treated as a product for public release. It separates **current development evidence** from **release-gate work**.

> Status note (2026-11-09): the project is in active development with no guarantee of success and no decision to publish. Trademark, license/publication choices, and signing/distribution steps are **intentionally deferred** until a public-release decision is made.

## Phase status (ARCHITECTURE_PLAN P0–P10)

| Phase | Scope (summary) | Status | Evidence (what is actually proven) | Not yet proven / gaps |
|---|---|---|---|---|
| **P0** | Repo skeleton, CI, toolchain, threat model, benchmark design, ADRs | Done (dev) | CI green on `nayeemx/innoframework` (`fmt`, `clippy -D warnings`, `test`, `cargo-deny`, `cargo-audit`). Threat model/benchmark/ADR docs exist. | Benchmark code not run; docs live in mixed locations (root `docs/` + `innoframework/docs/`). |
| **P1** | Core runtime: errors, validation, capability/secret boundary, manifest | Done (dev) | Automated tests pass in CI/workspace (core/manifest/golden/negative cases). Fail-closed authZ/secret checks covered by tests. TR-1.3 negative compile-fail added via `innoframework-rpc-macro` trybuild (`tests/ui/missing_guard.rs`). | Verify trybuild case stays in CI; no other known P1 gate gap. |
| **P2** | RPC + auth: macro, router, CSRF, error mapping, body limits | Done (dev) | 401/403/200/422/404 integration-style tests pass; real handler logic exercised with constructed inputs/session tokens. | Not proven as a deployed/field HTTP service under load; full axum server E2E in production-like env not recorded here. |
| **P3** | CLI + wizard + web build (`inno init/dev/build web`), CSP | Mostly done (dev) | `inno init -y` scaffolds (local E2E). `inno build web` runs real `dx build --platform web --release` and writes `csp-header.txt` (local E2E). `inno dev` serves CSP + `X-Content-Type-Options: nosniff` on the wire (local E2E, HTTP verified). 8 CLI tests + workspace total 85 tests pass; CI green for recent commits. | **TR-2.1 fresh Windows VM install not run** on a clean machine (plan: `docs/TR_2_1_INSTALL.md`). Bundle/perf budgets not measured. `dev`/`build` behavior proven on the dev machine, not across clean/field environments. Desktop/mobile build paths are stubs/guidance. |
| **P4** | UI kit v1, themes, a11y/visual tests | In progress (dev) | `innoframework-ui` (Dioxus 0.7): tokens, Web+Windows themes/CSS vars, 3 layouts, TR-4.1 catalog components, TR-4.4 motion+reduced-motion, TR-4.5 contrast/touch-target checks. CSP-safe `inno init` web template emits `assets/inno-ui.css` via shared UI CSS + `asset!`/`Stylesheet`. Axe-core/Playwright spike in `tools/ui-a11y-snapshots` (not in Rust CI). `cargo test -p innoframework-ui` **28 pass**, `cargo test --workspace` **114 pass**, clippy clean. See `docs/P4_UI_STATUS.md`. | **Gate not complete:** axe-core/AccessKit not run green in CI/pinned env; visual no-overflow snapshots not baselined; full TR-4.1–4.5 enforcement/behavior hardening/responsive coverage incomplete. Not field/browser-proven. |
| **P5** | Desktop shell (Tauri 2), capabilities, signed installer | Not started | — | All deliverables/gates open. |
| **P6** | Data layer (Postgres/SQLite adapters, migrations, conformance) | Not started | — | All deliverables/gates open. |
| **P7** | AI agent runtime (tools, injection screen, llama.cpp/Gemini) | Not started | — | All deliverables/gates open. |
| **P8** | Docs site + showcase #1 | Not started | — | All deliverables/gates open. |
| **P9** | Mobile (Tauri iOS/Android), signing wizard, device tests | Not started | — | All deliverables/gates open (publishing accounts out of scope per ADR-13). |
| **P10** | Showcases #2–3, hardening, external security review, public beta | Not started | — | All deliverables/gates open. |

## Honest evidence levels (what “works” means here)

- **CI/automated tests (strongest current evidence):** code compiles, lints clean (`-D warnings`), unit/integration tests pass, `cargo-deny`/`cargo-audit` pass on CI runners. This proves behavior under the tested cases, not all real-world cases.
- **Local E2E (developer machine):** `inno init`/`dx build web`/`inno dev` + CSP headers were exercised on this Windows dev environment. This is real execution, but **not field validation** (different machines, clean installs, network/proxy/toolchain variance, real users).
- **Not proven:** fresh-VM install (TR-2.1), performance/bundle budgets, production HTTP deployment, desktop/mobile packaging/signing, security review, and any claim of “ready for public use.”

**Conclusion:** completed phases (P0–P2, most of P3) are **implemented and verified by automated tests + limited local E2E**; they are **not** claimed to be field-proven. Treat remaining P3 gates and P4–P10 as required before calling it production-ready.

## Pre-release checklist (use only if/when you decide to publish)

### 1) Product/phase gates
- [ ] Close P3 gates: run TR-2.1 on a clean Windows VM/image; confirm `inno init`/`build web`/`dev` (CSP) from zero toolchain to running app; record results.
- [ ] Resolve known doc/plan issues: ARCHITECTURE_PLAN §6 vs INNOFRAMEWORK_ARCHITECTURE §2.3 budget conflict (owner decision); confirm P1 compile-fail/trybuild gate or accept current evidence.
- [ ] Complete P4–P10 phases and their gate criteria (UI, Tauri desktop/mobile, data, AI, docs/showcases, hardening).
- [ ] Measure performance/bundle budgets on reference hardware (no unmeasured claims).

### 2) Security & reliability
- [ ] Re-run full CI (`fmt`, `clippy`, `test`, `cargo-deny`, `cargo-audit`) green on release commit/tag.
- [ ] Review threat model controls vs implementation; add/verify negative tests for in-scope threats.
- [ ] External security review (P10) with findings triaged/fixed/accepted.
- [ ] Validate CSP/auth/CSRF/secret-boundary behavior in a realistic deployment (not only unit/integration harness).
- [ ] Dependency/SBOM review; confirm update/vulnerability process.

### 3) Legal, naming, and distribution (intentionally deferred now)
- [ ] **License:** choose and add a license (and verify dependency license compatibility) before public distribution.
- [ ] **Trademark/naming:** clear `Innoframework`/branding (e.g., USPTO/EUIPO/WIPO as applicable) before public launch/marketing.
- [ ] **ADR-09 signing:** obtain/configure code-signing (Windows cert; Apple/Google as needed for P5/P9); verify signed installers/apps.
- [ ] **Store/publishing accounts (ADR-13):** only if/when distributing to app stores; not required to build/test locally.
- [ ] Confirm third-party notices, attribution, and acceptable-use/AI/provider terms where applicable.

### 4) Packaging, docs, and support
- [ ] Produce release artifacts (versioned, reproducible where practical), checksums, and install/upgrade docs.
- [ ] Run TR-2.1/fresh-install flow against the actual release artifacts (not just a dev build).
- [ ] Publish docs/getting-started that match the shipped CLI/behavior; label experimental areas clearly.
- [ ] Define support/channel for bug reports and security disclosures.

## Owner-deferred (explicit, current decision)
- Trademark clearance, license/publication, ADR-09 signing, and public-launch actions are **deferred by owner choice** during development. Revisit only when you decide this is a product to publish.
