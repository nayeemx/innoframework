# Innoframework — Architecture Plan (Planning Phase, no implementation code)

Status: DRAFT for review. Supersedes the marketing-level blueprint in
`INNOFRAMEWORK_FRAMEWORK_ARCHITECTURE.md` wherever the two disagree. Every claim about
third-party technology below was checked against sources dated Nov 2026
(Dioxus docs 0.7, Blitz README, Tauri 2 docs). Where a claim could not be
verified it is marked UNVERIFIED.

---

## 0. Decisions made in this phase (ADR summary)

| ID | Decision | Why | Reversible? |
|---|---|---|---|
| ADR-01 | Core language: **Rust** | Memory safety removes the class of bugs behind the PHP/Node CVE stream the owner cited; single static binary satisfies "no Node.js install"; matches existing projects (innology-code, InnologyLock, NetworkFix). | No (foundation) |
| ADR-02 | **No renderer invented in v1.** Desktop + mobile shell = **Tauri 2** (system webview). Web = standard WASM/HTML output. | Tauri 2 is stable with iOS/Android targets and a capability ACL. Dioxus Native/Blitz is beta (its README: "usable … if you are an early adopter"); it has no JavaScript, no WebSocket, no localStorage. Building a renderer first would consume the whole schedule. | Yes — renderer is behind a trait (ADR-05) |
| ADR-03 | UI authoring model: **Rust components compiled to web via Dioxus 0.7 first; desktop/mobile via Tauri shell hosting the same WASM web build** | One UI codebase, two targets, no second renderer to maintain. | Yes |
| ADR-04 | Security default-deny: capability manifest for every target, auth guard on every RPC, CSP preset, build-time secret boundary | Required by the owner and by the SECURITY_CHECKLIST_V1/V2 studies already in repo. | No |
| ADR-05 | Every external capability (renderer, DB, AI provider, storage, auth backend) sits behind a Rust trait; concrete impls are plugins | Lets Tauri/Dioxus/webview be swapped and keeps the core testable. | No (architectural) |
| ADR-06 | The "installer" is a **signed Rust binary** plus an optional `inno` self-update channel. No Node.js requirement for the user. | Owner requirement. Generated web assets are prebuilt, so the user does not need Node to run the app. Build-time JS tooling (if any) is vendored into the binary's toolchain or delivered as WASM. | Yes |
| ADR-07 | Native AI: **local GGUF via llama.cpp** as default provider; cloud providers optional and server-side only | Matches FRAMEWORK_V2_SPEC and AI-11/AI-12 in SECURITY_CHECKLIST_V2. | Yes |
| ADR-08 | Docs site: static generator output deployed to **Netlify**; no server required for docs | Owner hosts on Netlify today (PROJECT_STACK_SURVEY). | Yes |

**Explicit non-goals for v1:** our own renderer, our own JS engine, our own
compiler frontend, mobile native-view renderer, multimedia generation models
(image/video generation is a provider integration, not a framework feature),
Office-format parsing beyond CSV/XLSX read-write.

---

## 1. Requirement analysis

The owner's request decomposes into 6 top-level requirements. Each is broken
into testable technical requirements (TR). "Testable" means a command or test
can return pass/fail.

### R1 — Core architecture & security
- TR-1.1 Single Rust workspace produces the CLI, runtime libs, and build tools. Verify: `cargo build --workspace` on Windows, macOS, Linux.
- TR-1.2 No Node.js/npm/Python on the end user's machine for install, run, or build of a generated app. Verify: run the CLI in a clean VM image with no runtimes; scaffold + build + run succeeds.
- TR-1.3 Default-deny: an RPC endpoint without an auth guard fails to compile (type-level), not only at runtime. Verify: compile-fail test.
- TR-1.4 Every security item in SECURITY_CHECKLIST_V1 (CSP-1..10, XSS-1..9, CSRF-1..8, SEC-BUILD-1..3, SEC-RPC-1..4) has a negative test. Verify: checklist-to-test traceability matrix, CI gate.
- TR-1.5 Performance budgets (set in §4 below), measured in CI on a reference machine.

### R2 — Installation & UX
- TR-2.1 Install in one command per OS, no prerequisite software. Verify: fresh VM test.
- TR-2.2 `inno init` wizard asks: target platforms, data backend, AI provider, auth method. Each answer produces a concrete config diff shown before writing. Verify: golden-file tests for every answer combination.
- TR-2.3 Generated project builds and runs with zero manual edits. Verify: scaffold matrix in CI.
- TR-2.4 Errors are actionable: each error code maps to a fix sentence and doc link. Verify: lint over error catalog.

### R3 — Multi-platform single codebase
- TR-3.1 One UI source tree → web (WASM), desktop (Tauri 2), mobile (Tauri 2 iOS/Android). Verify: three build targets from one fixture app.
- TR-3.2 Desktop/mobile builds expose platform APIs (fs, dialog, notifications, biometric) only through declared capabilities. Verify: a call without a capability fails at runtime with a typed error and the build fails if the capability is missing from the manifest.
- TR-3.3 "Better than PWA" is a measurable claim, so it is defined as: cold start, p95 input latency, and access to native APIs the PWA cannot reach (file system, background tasks, biometrics). Verify: benchmark comparison against a PWA build of the same fixture (see §4).

### R4 — Component library, design system, themes
- TR-4.1 Component catalog of at least: Button, Input, Select, Checkbox, Radio, Switch, Dialog, Popover, Tooltip, Tabs, Table, Form, Toast, Card, Layout primitives. Each has an accessibility test (keyboard + ARIA). Verify: axe-core or AccessKit checks in CI.
- TR-4.2 Theme tokens (color, type, spacing, radius, motion) defined once; per-platform theme packs (macOS, Windows, Linux, iOS, Android, Web) only change tokens, not component code. Verify: token diff tests.
- TR-4.3 Layout templates (dashboard, settings, auth, data table, landing) render at 3 breakpoints without overflow. Verify: visual regression snapshots.
- TR-4.4 Animation engine: declarative tokens (duration, easing) + reduced-motion respect. Verify: `prefers-reduced-motion` test.
- TR-4.5 Design principles are a document with checkable rules (contrast ≥ 4.5:1, touch target ≥ 44px on mobile). Verify: lint rules.

**Scope honesty:** Radix-equivalent accessibility for ~30 components is a
multi-year effort for a team. v1 ships ~15 components. The rest are roadmap items,
and the docs must say so. Shipping fewer components that are correct beats
shipping many that are not.

### R5 — AI and agentic capabilities
- TR-5.1 Agent runtime: tool calls go through an allowlist registry with zod-equivalent (serde + schema) input/output validation. Verify: AI-1..AI-3 tests from SECURITY_CHECKLIST_V2.
- TR-5.2 Local provider (llama.cpp via a managed child process bound to 127.0.0.1) and remote providers behind one trait. Verify: conformance tests run against both (a mock server for remote).
- TR-5.3 Prompt-injection screening and output sanitization (AI-4..AI-9). Verify: negative test corpus.
- TR-5.4 Office automation: read/write CSV and XLSX; DOCX read/write is **UNVERIFIED** for v1 and deferred until a parser library is evaluated.
- TR-5.5 Code editing tool: apply a patch to a file inside a declared workspace root only. Verify: path-traversal tests (`..`, symlinks, absolute paths).
- TR-5.6 Image/video generation: provider plugin only. The framework does not bundle a generative model. Verify: plugin contract test.

### R6 — Documentation & showcase
- TR-6.1 Static docs site built from the repo, deployed to Netlify, with strict CSP headers (matches portfolio's netlify.toml pattern from PROJECT_FRONTEND_STUDY). Verify: header check in CI.
- TR-6.2 Every public API item has a doc comment and a runnable example; examples are compiled in CI. Verify: `cargo test --doc`.
- TR-6.3 Three showcase apps, each built only with Innoframework public APIs, each with its own test suite. Verify: showcase CI.

---

## 2. System architecture

### 2.1 Layers

```
┌────────────────────────────────────────────────────────────────┐
│ L5  Product code (user app)                                    │
│     ui/ (components)  server/ (RPC handlers)  agents/          │
├────────────────────────────────────────────────────────────────┤
│ L4  Innoframework SDK (public crates)                               │
│     inno-ui   inno-rpc   inno-auth   inno-agent│
├────────────────────────────────────────────────────────────────┤
│ L3  Platform traits (stable interfaces, ADR-05)                │
│     Renderer   Storage   Secrets   AiProvider   Notifier       │
├────────────────────────────────────────────────────────────────┤
│ L2  Adapters / plugins (swappable)                             │
│     web(WASM)  tauri-desktop  tauri-mobile  sqlite  postgres   │
│     llama.cpp  gemini  openai  keychain  dpapi                 │
├────────────────────────────────────────────────────────────────┤
│ L1  Core runtime (no I/O, no unsafe unless justified)          │
│     signals  routing  serialization  capability checks         │
│     validation  error model  event bus                         │
├────────────────────────────────────────────────────────────────┤
│ L0  Toolchain: `inno` CLI, build orchestrator, scaffolder  │
└────────────────────────────────────────────────────────────────┘
```

Rule: a layer may depend only on layers below it. L5 never imports an L2
adapter directly; it uses L3 traits. This is enforced by a workspace lint
(see §3, "dependency rules").

### 2.2 Workspace layout (planned, not created)

```
inno/
  crates/
    inno-core/        L1 signals, errors, validation, capability model
    inno-rpc/         L4 typed RPC: macro + client stub + server router
    inno-auth/        L4 sessions, guards, role checks
    inno-ui/          L4 components + tokens (web build via Dioxus)
    inno-agent/       L4 agent runtime + tool registry
    inno-cli/         L0 `inno` binary
    inno-build/       L0 target orchestration (web/desktop/mobile)
    adapters/             L2 one crate per adapter (feature-gated)
  templates/              project templates (wizard outputs)
  themes/                 token packs per platform
  docs/                   docs site source
  showcases/              three showcase apps
  tests/                  cross-crate integration + security suites
```

### 2.3 Core data model (entities that must exist before code)

| Entity | Fields (minimum) | Invariants |
|---|---|---|
| `Capability` | id (`fs.read`, `notify.send`, …), platforms[], scope (path globs / none) | A platform API call is allowed only if a Capability with matching id and platform is in the manifest |
| `Manifest` (`inno.toml`) | name, version, targets[], capabilities[], data backend, auth config, ai config | Unknown capability id → build error. Secrets are never valid values (checked by the secret boundary) |
| `Session` | user_id, roles[], issued_at, expires_at, token_id | expires_at > issued_at; tokens are HMAC-signed; validation uses constant-time compare |
| `RpcRoute` | name, method, input_schema, output_schema, guard, handler | A route without a guard does not compile (typestate) |
| `Tool` | name, description, input_schema, output_schema, risk_level, handler | Registry rejects duplicate names; risk_level `high` requires explicit opt-in |
| `Agent` | model_ref, tools[], system_template, limits{max_steps, timeout_ms, max_output_chars} | tools ⊆ registry; limits are mandatory (no unbounded agent) |
| `ThemeTokens` | color, typography, spacing, radius, motion, platform | Contrast pairs validated at load time (WCAG AA) |
| `BuildTarget` | kind (web/desktop/mobile), triple, bundle format | Mobile requires signing identity config; missing → build error with fix text |

### 2.4 Key interaction patterns

**A. Client → server call (typed RPC)**
```
UI event → generated client stub (serialize input)
        → HTTP POST /rpc/<name>  (CSRF header + SameSite cookie + Sec-Fetch-Site check)
        → router: guard → validate input → handler → validate output → serialize
        → error mapper (non-leaky) → response
```
Each step is a separate function so each can be tested alone. Failure at any
step returns a typed error; no step is skipped on error.

**B. Desktop/mobile call to native API**
```
UI → inno-core::platform::fs::read(path)
   → capability check against manifest (compile-time where possible, runtime otherwise)
   → Tauri command (only the commands named in the capability file are exposed)
   → adapter (fs/keychain/notify) → result
```

**C. Agent tool call**
```
user prompt → injection screen → render system template (substitution only)
           → model.generate()
           → parse tool call → registry lookup (allowlist) → input validation
           → handler (inside workspace sandbox if file tool) → output validation
           → step counter / timeout check → loop or return sanitized output
```

**D. Build**
```
inno build <target>
  → read manifest → check capabilities + secrets (fail fast)
  → compile Rust per target (cargo, pinned toolchain via rustup)
  → web: wasm-bindgen + static assets + CSP header file
  → desktop/mobile: Tauri bundle with generated capability JSON
  → emit SBOM (cargo auditable) + checksum
```

### 2.5 Data & persistence

- Local-first: SQLite (via a Rust driver, UNVERIFIED which crate; to be chosen in
  Phase 1 with a benchmark) for desktop/mobile; IndexedDB for web (through a
  storage trait).
- Remote: Postgres adapter first (matches vps/web and my-vps stacks). Firebase and
  Supabase adapters are plugins, not core.
- Migrations: forward-only, checksummed, run by the CLI. Rollback is a new
  migration, not a down script (reduces untested paths).

### 2.6 Why this shape

It removes the three failure modes in the owner's stack survey:
1. Per-project re-implementation of auth/email/payments → one audited module each.
2. Mixed runtimes (Node + Python + Rust + Go) → one toolchain (Rust).
3. Security as an afterthought → security is in L1 and enforced by types and the build.

---

## 3. Design patterns and principles

| Principle / pattern | Where applied | Concrete rule |
|---|---|---|
| **Typestate** | RpcRoute (guard must be set), BuildTarget (signing set for mobile) | Builder methods return new types; `build()` exists only on the complete type |
| **Capability-based security** | Platform API access | No ambient authority: every native call names a capability |
| **Ports & Adapters (hexagonal)** | L3 traits / L2 adapters | Core depends on traits only; adapters are crates behind cargo features |
| **Parse, don't validate** | Inputs at every boundary | Raw input → typed value once at the edge; the core never sees unvalidated data |
| **Fail closed** | Auth, capabilities, tools | Unknown → deny. Default values are the safe ones |
| **Single source of truth** | Manifest → codegen (capabilities JSON, CSP header, route table) | Generated files are never hand-edited; CI checks they match |
| **Explicit state machines** | Build pipeline, mobile signing, agent loop | Each state has a named enum variant; illegal transitions are unrepresentable |
| **SOLID** | S: one crate per capability; O: new adapter without core change; L: all adapters pass the same conformance suite; I: small traits (Storage, Secrets split); D: core depends on abstractions | Enforced by review checklist and conformance tests |
| **DRY via codegen, not macros-everywhere** | Route/RPC stubs, capability files | Macros only where the type system cannot express it; otherwise build-time codegen |
| **Structured concurrency** | Agent loop, build orchestrator | Every spawned task has an owner and a cancel path |
| **Error model** | All crates | One `InnoframeworkError` enum with stable codes (`AUD-xxxx`); user messages and internal details are separate fields; internal details never cross the RPC boundary (SEC-RPC-4) |
| **Observability by default** | RPC, agent, build | `tracing` spans with request id; no secrets in spans (checked by a redaction test) |
| **Dependency rules** | Workspace | A `cargo-deny` + custom check fails if L5 imports L2, or if L1 has I/O |

**Anti-patterns to avoid (seen in the owner's existing repos):**
- Dual-state-library apps (Redux + TanStack Query for the same concern) → one state model in core.
- Unpinned dependencies (`latest`, `>=` ranges) → lock file committed; `cargo deny` bans wildcard versions.
- Swallowed install errors (`2>/dev/null ||`) → CLI never swallows errors.
- Docker socket mounted into app containers → not supported by the runtime; the CLI does not mount host sockets.
- Dynamic reverse-proxy config written by the app without validation → the proxy config is generated from the manifest only.
- Wide CSP exceptions (`unsafe-inline` on a whole origin) → CSP presets have no exception slot; overrides must be explicit, reviewed, and listed in the manifest.

---

## 4. Risk assessment

| # | Risk | Likelihood | Impact | Mitigation | Early signal |
|---|---|---|---|---|---|
| RK-1 | Scope explosion: "everything for everyone" never ships | High | Critical | Hard v1 scope (§5.1). Every new item needs a written trade-off entry. | Roadmap items added without removal |
| RK-2 | Dependence on Dioxus 0.7 API churn (0.6 → 0.7 changed API; NetworkFix and innology-code already diverge) | Medium | High | Pin exact version; wrap Dioxus behind L3 `Renderer` trait; upgrade only with a green conformance suite | Version bump PRs |
| RK-3 | Blitz/Dioxus Native not production-ready | High (current) | High | Not used in v1 (ADR-02). Re-evaluate at Phase 4 gate. | Blitz release notes |
| RK-4 | Tauri mobile platform friction (signing, iOS entitlements, Android SDK) | High | Medium | Mobile is Phase 4. Document signing setup as a required wizard step. Ship desktop first. | Failed device builds |
| RK-5 | Security regression from a new dependency | Medium | Critical | `cargo deny`, `cargo audit` in CI; pinned lock file; SBOM per release; capability review for every new crate | Advisory alerts |
| RK-6 | Local LLM performance too slow on target hardware | Medium | Medium | Benchmark on 3 reference machines in Phase 3; cloud provider as explicit fallback; document minimum hardware | Benchmark numbers |
| RK-7 | Office automation (DOCX) parser quality and licensing | Medium | Medium | v1 = CSV/XLSX only; DOCX gated on library evaluation | Library evaluation result |
| RK-8 | Claimed performance numbers not reproducible | High | High | No performance claim is published without a benchmark script in the repo and the machine spec | Missing benchmark artifact |
| RK-9 | Installer trust (unsigned binary blocked by Windows SmartScreen / macOS Gatekeeper) | High | High | Code-sign releases; publish checksums; SBOM | First user install report |
| RK-10 | Documentation drift from code | High | Medium | Doc examples compiled in CI (TR-6.2); API reference generated from source | Broken doc tests |
| RK-11 | Owner's own time: the work is large and multi-year for one person | High | Critical | Phased roadmap with a usable product at each gate; showcase apps reuse existing owner projects where possible | Phase gate slips twice |
| RK-12 | Prior art already solves parts (Tauri, Dioxus, Leptos, Loco) | High | Medium | Position Innoframework as an integrator with the security and capability model as its own value; do not reimplement what they do well — wrap it | Competing features |
| RK-13 | Supply-chain attack via build tools (the owner's stated concern) | Medium | Critical | Minimal build dependencies, vendored toolchain config, reproducible builds, no post-install scripts in generated projects | Advisory feed |

**Edge cases that must have tests before Phase 2:**
- RPC body larger than limit → 413, not a crash.
- Session token with valid signature but expired → 401.
- Two tokens with same payload different key → rejected.
- Route name collision between two modules → compile error.
- Capability declared for desktop but built for web → build warning becomes error in strict mode.
- Agent tool returns output that contains `<script>` → sanitized.
- Path traversal in file tool via symlink inside workspace pointing outside → denied.
- Build on a machine with a different Rust version than pinned → refuse with fix text.
- Locale/encoding: Bangla text (the owner's projects are bilingual) round-trips through RPC and PDF/CSV export.

**Technical debt we accept knowingly (documented, not hidden):**
- v1 uses Tauri's webview on desktop; this means webview version differences remain. Mitigation: pin Tauri and document minimum OS versions.
- Component count is intentionally small in v1.

---

## 5. Implementation roadmap

Principle: **foundation before features**. Each phase has an exit gate. A phase
does not start until the previous gate passes. No phase is "done" without tests
for its security requirements.

### 5.1 v1 scope (what ships first)

Web + desktop, one codebase. Auth, typed RPC, CSP, secret boundary, ~15
components, 2 themes (light/dark, Web and Windows), wizard, Postgres + SQLite,
docs site. AI, mobile, and the showcase apps come after.

### 5.2 Phases

| Phase | Name | Scope | Exit gate (all must pass) | Est. effort (1 senior eng, rough) |
|---|---|---|---|---|
| **P0** | Foundation & decisions | This plan approved; ADR log; repo skeleton; CI (fmt, clippy, test, cargo-deny, cargo-audit); threat model doc; traceability matrix template; benchmark harness design | Plan approved by owner; CI green on empty workspace; traceability template exists | 1–2 weeks |
| **P1** | Core runtime (L1) | Signals, error model, validation, capability model, manifest parser + schema, secret boundary check | Unit tests for every invariant in §2.3; compile-fail tests for typestate; manifest golden tests; secret-boundary negative tests | 4–6 weeks |
| **P2** | RPC + auth (L4 part 1) | Typed RPC macro/codegen, router, guard typestate, session tokens (HMAC, constant-time), CSRF checks, error mapper, body limits | TR-1.3, SEC-RPC-1..4, CSRF-1..8 negative tests all green; RPC call from a generated client to a running server over real HTTP; 401/403/413/422 tested | 4–6 weeks |
| **P3** | CLI + wizard + web build (L0, L5) | `inno init` wizard, `dev`, `build web`, templates, CSP header generation, WASM build | TR-2.1–2.4 pass; fresh-VM install test; generated app builds and serves with CSP header; bundle budget met (§4 perf) | 4–6 weeks |
| **P4** | UI kit v1 + themes (L4 UI) | ~15 components, token system, Windows + Web themes, 3 layout templates, accessibility checks | TR-4.1–4.5 for shipped items; axe/AccessKit CI green; visual snapshots for 3 breakpoints | 6–8 weeks |
| **P5** | Desktop shell (L2 Tauri) | Tauri 2 adapter, capability JSON generation, native fs/notify/keychain adapters, signed Windows installer | TR-3.1 (desktop), TR-3.2 negative tests; installer runs on clean Windows VM; benchmark vs PWA recorded | 4–6 weeks |
| **P6** | Data layer | Postgres + SQLite adapters, migrations, conformance suite run against both | Conformance suite passes on both; migration checksum tests; backup/restore test | 3–4 weeks |
| **P7** | AI agent runtime (L4 agent) | Tool registry, validation, injection screen, sanitizer, llama.cpp managed child process, Gemini provider behind trait, office CSV/XLSX tools | TR-5.1–5.5 green; path-traversal suite; local model benchmark on 3 reference machines published | 6–8 weeks |
| **P8** | Docs site + showcase #1 | Docs generator, Netlify deploy, strict headers, doc-test CI; one showcase (desktop tool) | TR-6.1–6.3 for showcase #1; docs deployed; docs examples compile | 3–4 weeks |
| **P9** | Mobile (Tauri iOS/Android) | Mobile adapter, signing wizard step, device tests | TR-3.1 mobile; device run on one iOS and one Android device; frame-time and cold-start budgets (§4) met | 6–8 weeks |
| **P10** | Showcases #2–3, hardening, public beta | Remaining showcases; external security review; SBOM + signed releases | External review findings all triaged; release signing verified; SmartScreen/Gatekeeper path documented | 4–6 weeks |

Total rough estimate: 9–12 months for one senior engineer at focused pace. The
owner should plan for longer if working part time. These are estimates, not
commitments, and they will be updated at each gate.

### 5.3 Why this order

- P1–P2 come before any UI because the security model and the RPC boundary are
  the parts that are hardest to retrofit. Retrofitting auth into a UI-first
  framework is exactly the failure pattern in the owner's existing repos.
- P3 (web) comes before desktop because the web build is the cheapest end-to-end
  proof that the whole pipeline works. Desktop reuses it.
- P7 (AI) comes after the RPC boundary because agent tools must reuse the same
  validation and auth path (TR-5.1, AI-13/AI-14).
- Mobile is last because it has the highest unknowns (signing, device variance)
  and it is the least reversible investment.

### 5.4 Gates that can stop the project

Each gate has a stop condition written now, so it is decided in advance and not
under sunk-cost pressure:
- **G-P2 stop:** if the typed RPC cannot be made to fail-compile on unguarded routes, the typestate approach is wrong; revisit before P3.
- **G-P3 stop:** if the web build exceeds 2× the plain Vite/React baseline for the fixture app, reconsider the WASM UI approach (fall back to JS UI with a Rust core).
- **G-P5 stop:** if the Tauri desktop build cannot pass TR-3.2 negative tests, do not ship desktop; ship web-only and say so.
- **G-P7 stop:** if no local model meets the documented latency budget on reference hardware, ship cloud-only AI and label it.

---

## 6. Performance budgets (targets, to be measured — not claims yet)

| Metric | Target | Measured how | Status |
|---|---|---|---|
| Web fixture first contentful paint | ≤ 1.5 s on mid-range laptop, 4G throttle | Lighthouse CI on fixture | not measured |
| Web bundle (gzipped, fixture app) | ≤ 2× plain Vite React fixture | build size script | not measured |
| Desktop cold start (fixture) | ≤ 1.5 s to first paint | scripted timing on reference Windows VM | not measured |
| Desktop idle RSS | ≤ 150 MB | process metric | not measured |
| RPC round trip (localhost, JSON, small payload) | p95 ≤ 10 ms | load script | not measured |
| Mobile cold start / frame time | per P9 gate | device lab | not measured |

Note: earlier documents in this repo claimed numbers such as "4584x faster"
and "180x slower". Those came from micro-benchmarks that did not measure what
the claims implied and are withdrawn. No performance claim is made until the
benchmark for it exists in the repo.

---

## 7. What must be decided by the owner before P1 starts

1. Confirm ADR-02: Tauri 2 for desktop/mobile instead of a self-written renderer.
2. Confirm ADR-01: Rust as the only core language.
3. Choose v1 component list (§4 TR-4.1) — the list is a proposal.
4. Choose the first two adapters: Postgres and SQLite? Or Firebase first to match existing projects?
5. Name and legal: "Innoframework" was used in the last blueprint without a trademark check. Confirm the name or pick another before public docs (see TR gap noted in Phase 0 of the earlier roadmap).
6. Who signs the release binaries (code-signing certificate for Windows and Apple developer account for macOS/iOS).
7. Reference hardware for benchmarks (one low, one mid, one high spec machine).

---

## 8. Honest gaps in this plan

- Tauri v2 mobile specifics (signing flow, entitlements) were verified at the
  capability-model level only. Device-level steps are UNVERIFIED until P9.
- The exact SQLite Rust driver, the llama.cpp binding crate, and the RPC codegen
  approach (proc-macro vs build-script) are open. Each is decided by a spike in
  its phase, with the decision recorded as an ADR.
- No component accessibility test tooling was evaluated yet (axe-core is web-only;
  AccessKit is Blitz's choice). This is a P0 spike.
- Competitive positioning vs Tauri/Dioxus/Leptos/Loco was not benchmarked. The
  claim "better than PWA" is only a hypothesis until P5's benchmark exists.
