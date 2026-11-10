# Innoframework Launch Readiness Checklist

Purpose: a go/no-go checklist for when this project is treated as a product for public release. It separates **current development evidence** from **release-gate work**.

> Status note (2026-11-09): the project is in active development with no guarantee of success and no decision to publish. Trademark, license/publication choices, and signing/distribution steps are **intentionally deferred** until a public-release decision is made.

## Phase status (ARCHITECTURE_PLAN P0-P10)

| Phase | Scope (summary) | Status | Evidence (what is actually proven) | Not yet proven / gaps |
|---|---|---|---|---|
| **P0** | Repo skeleton, CI, toolchain, threat model, benchmark design, ADRs | Done (dev) | CI green on `nayeemx/innoframework` (`fmt`, `clippy -D warnings`, `test`, `cargo-deny`, `cargo-audit`). Threat model/benchmark/ADR docs exist. | Benchmark code not run; docs live in mixed locations (root `docs/` + `innoframework/docs/`). |
| **P1** | Core runtime: errors, validation, capability/secret boundary, manifest | Done (dev) | Automated tests pass in CI/workspace (core/manifest/golden/negative cases). Fail-closed authZ/secret checks covered by tests. TR-1.3 negative compile-fail added via `innoframework-rpc-macro` trybuild (`tests/ui/missing_guard.rs`). | Verify trybuild case stays in CI; no other known P1 gate gap. |
| **P2** | RPC + auth: macro, router, CSRF, error mapping, body limits | Done (dev) | 401/403/200/422/404 integration-style tests pass; real handler logic exercised with constructed inputs/session tokens. | Not proven as a deployed/field HTTP service under load; full axum server E2E in production-like env not recorded here. |
| **P3** | CLI + wizard + web build (`inno init/dev/build web`), CSP | Mostly done (dev) | `inno init -y` scaffolds (local E2E). `inno build web` runs real `dx build --platform web --release` and writes `csp-header.txt` (local E2E). `inno dev` serves CSP + `X-Content-Type-Options: nosniff` on the wire (local E2E, HTTP verified). CLI tests pass; CI green for recent commits. | **TR-2.1 fresh Windows VM install not run** on a clean machine (plan: `docs/TR_2_1_INSTALL.md`) - owner action. Bundle/perf budgets not measured. Desktop/mobile build paths are stubs/guidance. |
| **P4** | UI kit v1, themes, a11y/visual tests | In progress (dev) | `innoframework-ui` (Dioxus 0.7): tokens, Web+Windows themes/CSS vars, 3 layouts, TR-4.1 catalog components, TR-4.4 motion+reduced-motion, TR-4.5 contrast/touch-target checks (incl. danger) and CSS contract tests. CSP-safe `inno init` web template emits `assets/inno-ui.css` via shared UI CSS + `asset!`/`Stylesheet`. Axe-core/Playwright spike in `tools/ui-a11y-snapshots` (not in Rust CI). UI 39 / workspace 133 tests, clippy green. Core `Storage` trait/id validation added as ADR-16 P6 foundation; `crates/storage-local` JSON file-backed adapter, `crates/storage-sqlite` sqlx SQLite adapter (in-memory/file, testable offline), `crates/storage-postgres` sqlx Postgres adapter (feature-gated behind `postgres`, needs live DB for live tests) added for local dev/tests (Firebase adapter deferred). See `docs/P4_UI_STATUS.md`. | **Gate not complete / owner-run evidence needed:** axe-core/AccessKit not run green in CI/pinned env; visual no-overflow snapshots not baselined; full TR-4.1-4.5 enforcement/behavior hardening/responsive coverage incomplete. Not field/browser-proven. |
| **P5** | Desktop shell (Tauri 2), capabilities, signed installer | In progress (dev) | `inno build desktop` now generates `src-tauri/tauri.conf.json` and capability JSONs from manifest capabilities (fs.read, fs.write, notify.send, keychain, dialog, shell), maps them to Tauri permissions/allowlist (fail closed), then runs `cargo tauri build`. Error codes AUD-3009/3010/3011 for Tauri toolchain missing, no desktop target, and build failure. 12 CLI tests pass, clippy green. Signing: deferred (ADR-09) — unsigned bundle. | Code-signing certificate (Windows cert) not available — ADR-09 deferred. Desktop bundle not yet tested end-to-end on a real machine. |
| **P6** | Data layer (Postgres/SQLite adapters, migrations, conformance) | In progress (dev) | `crates/storage-local` (JSON file-backed), `crates/storage-sqlite` (sqlx SQLite, in-memory/file), `crates/storage-postgres` (sqlx Postgres, feature-gated) all implement the `Storage` trait from `innoframework-core` with id validation, typed get/put, and fail-closed error codes (AUD-1600). SQLite round-trip/upsert/bad-id tests pass (3). Postgres stub compiles without feature; live DB tests skipped without `INNOFRAMEWORK_POSTGRES_TEST_URL`. | SQLite/Postgres adapters done (P6 storage). Migrations beyond CREATE TABLE, conformance suite against both backends, Firebase adapter, and backup/restore tests remain. Postgres live tests need a DB and `INNOFRAMEWORK_POSTGRES_TEST_URL`. |
| **P7** | AI agent runtime (tools, injection screen, llama.cpp/Gemini) | Done (dev) | `crates/agent` adds `ExecutorRegistry` (allowlist-only tool registration with handlers), `execute_agent()` bounded step loop with injection screening, `TOOL_CALL: {...}` parsing, registry allowlist check, input/output validation, output sanitization, `max_steps`/`timeout_ms`/`max_output_chars` enforcement. `crates/core` adds `injection.rs` (prompt-injection screening), `sanitizer.rs` (HTML sanitization), `provider.rs` (`ModelProvider` trait + `MockModelProvider`). All tests pass (12 tests total). | llama.cpp/Gemini concrete providers not yet implemented (mock only); no live model benchmark on reference hardware. |
| **P8** | Docs site + showcase #1 | In progress (dev) | `crates/docs-gen` parses Rust source with `syn` and generates markdown documentation from public items + doc comments (3 tests). `inno docs` CLI subcommand scans a directory and writes/prints markdown. Product homepage at `docs/site/index.html` (react.dev-style: hero, features, platforms, how-it-works, get-started) with CSP-safe self-hosted CSS. Showcase #1 at `showcases/showcase1/` — complete Dioxus 0.7 web app (inno.toml manifest, Cargo.toml, src/main.rs with #[server] typed server function, assets/inno-ui.css). | Showcase app not yet built/run end-to-end with `dx build`. Docs site not yet deployed to Netlify. |
| **P9** | Mobile (Tauri iOS/Android), signing wizard, device tests | In progress (dev) | `inno build mobile` now parses the manifest, validates the mobile target is declared, requires a signing identity (AUD-3013, ADR-09 deferred), generates `src-tauri/tauri.conf.json` and capability JSONs for mobile (Platform::Mobile), emits `src-tauri/mobile/target.txt`, and runs `cargo tauri build --target <bundle-target>`. `MobileTarget` enum supports Android (aarch64/armv7/x86_64-linux-android, android-aab) and iOS (aarch64/x86_64-apple-ios, ios-ipa). New error codes: AUD-3012 (no mobile target), AUD-3013 (signing required). Manifest gains `signing-identity` and `mobile-target` fields with secret boundary scanning. Desktop build uses `write_tauri_project_desktop`, mobile uses `write_tauri_project_mobile`. | No Apple/Google developer accounts (ADR-13 deferred) — no signed bundles, no store publishing. No physical iOS/Android devices for device tests. Mobile bundle not yet end-to-end tested. |
| **P10** | Showcases #2-3, hardening, external security review, public beta | In progress (dev) | Showcase #1 (web) at `showcases/showcase1/`, showcase #2 (desktop) at `showcases/showcase2/`, showcase #3 (mobile) at `showcases/showcase3/`. Each has `inno.toml` manifest, Cargo.toml, Dioxus 0.7 src/main.rs with `#[server]` typed server functions, CSP-safe assets/inno-ui.css. Showcase #2 declares `signing-identity` + desktop capabilities (fs.read/write, notify.send). Showcase #3 declares `signing-identity` + `mobile-target` + mobile capabilities (keychain, dialog). | Showcases not yet built/run end-to-end with `dx build`/`cargo tauri build`. External security review not performed. Public beta not launched (trademark/license/signing deferred). |

## Honest evidence levels (what "works" means here)

- **CI/automated tests (strongest current evidence):** code compiles, lints clean (`-D warnings`), unit/integration tests pass, `cargo-deny`/`cargo-audit` pass on CI runners. This proves behavior under the tested cases, not all real-world cases.
- **Local E2E (developer machine):** `inno init`/`dx build web`/`inno dev` + CSP headers were exercised on this Windows dev environment. This is real execution, but **not field validation** (different machines, clean installs, network/proxy/toolchain variance, real users).
- **Not proven:** fresh-VM install (TR-2.1), performance/bundle budgets, production HTTP deployment, desktop/mobile packaging/signing, security review, and any claim of "ready for public use."

**Conclusion:** completed phases (P0-P2, most of P3) are **implemented and verified by automated tests + limited local E2E**; they are **not** claimed to be field-proven. Treat remaining P3/P4 gates and P5-P10 as required before calling it production-ready.

## Pre-release checklist (use only if/when you decide to publish)

### 1) Product/phase gates
- [ ] Close P3 gates: run TR-2.1 on a clean Windows VM/image (owner action); record results.
- [ ] Resolve known doc/plan issues: ARCHITECTURE_PLAN §6 vs INNOFRAMEWORK_ARCHITECTURE §2.3 budget conflict (owner decision).
- [ ] Complete P4-P10 phases and their gate criteria (UI, Tauri desktop/mobile, data, AI, docs/showcases, hardening).
- [ ] Measure performance/bundle budgets on reference hardware (no unmeasured claims).

### 2) Security & reliability
- [ ] Re-run full CI (`fmt`, `clippy`, `test`, `cargo-deny`, `cargo-audit`) green on release commit/tag.
- [ ] Review threat model controls vs implementation; add/verify negative tests for in-scope threats.
- [ ] External security review (P10) with findings triaged/fixed/accepted.
- [ ] Validate CSP/auth/CSRF/secret-boundary behavior in a realistic deployment (not only unit/integration harness).
- [ ] Dependency/SBOM review; confirm update/vulnerability process.

### 3) Legal, naming, and distribution (intentionally deferred now)
- [ ] **License:** choose and add a license (and verify dependency license compatibility) before public distribution.
- [ ] **Trademark/naming:** clear `Innoframework`/branding before public launch/marketing.
- [ ] **ADR-09 signing:** obtain/configure code-signing (Windows cert; Apple/Google as needed for P5/P9); verify signed installers/apps.
- [ ] **Store/publishing accounts (ADR-13):** only if/when distributing to app stores.
- [ ] Confirm third-party notices, attribution, and acceptable-use/AI/provider terms where applicable.

### 4) Packaging, docs, and support
- [ ] Produce release artifacts (versioned, reproducible where practical), checksums, and install/upgrade docs.
- [ ] Run TR-2.1/fresh-install flow against the actual release artifacts.
- [ ] Publish docs/getting-started that match shipped CLI/behavior; label experimental areas clearly.
- [ ] Define support/channel for bug reports and security disclosures.

## Owner-deferred (explicit, current decision)
- TR-2.1 clean-VM execution/evidence, browser axe-core/visual snapshot execution in a pinned env, AccessKit/native CI decision, trademark clearance, license/publication, ADR-09 signing, and public-launch actions are **deferred by owner choice** during development.