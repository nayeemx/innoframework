# Architecture Decision Log

Format: each ADR has a status. `Proposed` = waiting for owner approval.
`Accepted` = approved and binding. `Superseded` = replaced by a later ADR.
Changing an `Accepted` ADR requires a new ADR that supersedes it.

| ID | Title | Status | Owner decision needed |
|---|---|---|---|
| ADR-01 | Rust as the only core language | Accepted | — |
| ADR-02 | Tauri 2 for desktop and mobile; no in-house renderer in v1 | Accepted | — |
| ADR-03 | Dioxus 0.7 for the web UI build; same WASM output hosted by the Tauri shell | Accepted | — |
| ADR-04 | Default-deny security: capabilities, auth guard on every RPC, CSP, build-time secret boundary | Accepted | — |
| ADR-05 | Platform capabilities behind Rust traits; concrete implementations are adapters | Accepted | — |
| ADR-06 | Single signed Rust CLI; no Node.js required on the user machine | Accepted | — |
| ADR-07 | Local llama.cpp (GGUF) as default AI provider; remote providers server-side only | Accepted | — |
| ADR-08 | Docs site is static output deployed to Netlify with strict headers | Accepted | — |
| ADR-09 | Release signing | Open | Windows certificate: owner has none yet. Apple: owner has no developer account. Mobile store submission deferred (see ADR-13). |
| ADR-10 | Data adapters | Accepted (scoped) | — |
| ADR-11 | Product name: Innoframework | Accepted (search pending) | Formal trademark clearance still required |
| ADR-12 | Reference hardware | Accepted (partial) | Low tier = owner machine i5-8350U, 16 GB, Intel UHD 620, Win11. Mid and high tiers still needed. |

## ADR-01: Rust as the only core language
- Context: the owner's existing stacks run on Node.js, PHP, Python, and Go. Several recent advisories affected Node and PHP. One language for the core reduces the attack surface and the number of runtimes the user must install.
- Decision: all framework crates, the CLI, and the build orchestrator are Rust. Generated web assets are static files and WASM. Users need no runtime.
- Consequences: contributors need Rust. The UI compiles to WASM, so the web build depends on the Rust toolchain at build time only.
- Revisit if: a required platform API has no usable Rust binding after P5.

## ADR-02: Tauri 2 for desktop and mobile
- Context: Dioxus Native (Blitz) is beta. Its README says it is "usable … if you are an early adopter." Blitz does not support JavaScript, WebSockets, or localStorage. Tauri 2 is stable and supports desktop, iOS, and Android. It has a capability and permission model that matches ADR-04.
- Decision: desktop and mobile shells are Tauri 2. The framework does not implement a renderer in v1.
- Consequences: rendering uses the system webview. Webview differences between OS versions remain and must be documented as minimum OS requirements.
- Revisit if: Blitz reaches a stable release and P5 benchmarks show a material advantage.

## ADR-03: Dioxus 0.7 for the UI
- Context: the owner's Rust projects already use Dioxus (innology-code uses 0.7, NetworkFix uses 0.6). Dioxus 0.7 has a WASM web renderer and a fullstack server-function model.
- Decision: UI components are written with Dioxus 0.7 and compiled to WASM for the web target. Desktop and mobile load the same web build inside Tauri.
- Consequences: a version bump requires a green conformance suite. Dioxus is wrapped behind the `Renderer` trait (ADR-05).
- Revisit if: the P3 bundle budget fails (plan section 5.4, stop condition G-P3).

## ADR-04: Default-deny security
- Decision: (1) every platform API call requires a capability in the manifest; (2) every RPC route requires an auth guard, enforced by type, not by convention; (3) the CSP preset has no exception slot; (4) the build fails if server secrets appear in client output.
- Source: SECURITY_CHECKLIST_V1.md and SECURITY_CHECKLIST_V2.md.

## ADR-05: Ports and adapters
- Decision: `Renderer`, `Storage`, `Secrets`, `AiProvider`, and `Notifier` are traits in the core. Each implementation is a separate crate behind a Cargo feature. Each adapter must pass the shared conformance suite for its trait.

## ADR-06: Single signed CLI
- Decision: the `audonomy` CLI is one Rust binary. Installation is a signed download with a published checksum. Build-time JavaScript tooling, if needed, is limited to what the Tauri CLI requires and is pinned.
- Revisit if: the Tauri CLI requires a runtime that cannot be bundled.

## ADR-07: Local-first AI
- Decision: llama.cpp runs as a managed child process bound to 127.0.0.1. Remote providers (Gemini, OpenAI) are optional, configured server-side, and their keys are never in client output.
- Source: FRAMEWORK_V2_SPEC.md, SECURITY_CHECKLIST_V2.md (AI-11, AI-12).

## ADR-08: Docs on Netlify
- Decision: docs are a static build deployed to Netlify. Headers follow the pattern in the owner's portfolio `netlify.toml` (CSP, frame-ancestors none, HSTS, Permissions-Policy).

## ADR-09 to ADR-12
Open. See `ARCHITECTURE_PLAN.md` section 7. These are decisions for the owner. Each will be written as an ADR once answered.


## ADR-13: Mobile store submission
Status: Accepted. Owner has no Apple or Google Play developer accounts. Decision: mobile builds are produced and tested (sideload/simulator), but store publishing is out of scope until the owner has accounts. This does not block building mobile apps; it only blocks publishing them.

## ADR-10 (scoped): Data adapters
Status: Accepted for development. Firebase is the development backend because the owner's current projects use it and the owner will supply credentials. Production backends are Postgres or SQLite, chosen when the owner moves to a VPS. Both paths go through the same storage trait (ADR-05); Firebase is a plugin, not a core dependency.

## ADR-11 (name) — Innoframework
Status: Accepted, pending formal trademark search. Name follows the owner's pattern (innofinance = inno + domain-word). Searches found many unrelated 'Framework' marks but no exact 'Innoframework' mark in the results checked; this is not a clearance. A formal USPTO / EUIPO / WIPO search is required before public launch. Domain innologybd is the brand root; the repo name is nayeemx/innoframework until the owner moves it to an organization.

## ADR-12 (hardware) — partial
Low-tier reference confirmed on the owner's machine (Intel Core i5-8350U 1.7 GHz, 4C/8T, 15.8 GB RAM, Intel UHD 620, Windows 11 Pro). Mid and high reference machines are still needed before any benchmark is published.

