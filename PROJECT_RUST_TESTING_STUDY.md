# Project Study: Rust projects, testing setups, Python (DriveBackup)

Source: read-only survey of D:\projects (Nov 2026). No project files were modified. Where a fact was inferred from a file name or was not verified, it is marked as such.

---

## 1. innology-code (Rust + Dioxus, cross-platform IDE)

Path: D:\projects\code\innology-code
Purpose: "Innology Code", a cross-platform IDE. Android (arm64) is the primary target; Windows and Linux are also planned. Status: pre-alpha. README says Phase 0a is in progress.

### Status and docs
- docs/ROADMAP.md is the most current source: 0a done, most of 0b done (0b.6 emulator script and 0b.7 gate still open), Phase 1 partly underway (F1.2 file explorer in progress).
- PLAN.md section 8 ("Immediate Next Steps") is stale; it still lists the docs skeleton as next.
- ROADMAP adds Phase 1 features that PLAN.md does not list: search, git, LSP, mobile input, adaptive shell.
- ADR-0013 introduces an embedded llama.cpp engine.

### Architecture decisions (from PLAN.md)
- Control API on an app-private Unix domain socket, authenticated with SO_PEERCRED (SEC-001). TCP is only for preview and artifact servers, with token and Origin checks.
- Cloudflare tunnel fronts only preview and artifact vhosts, never the control API (SEC-002).
- Embedded glibc rootfs run through a proot-style launcher on Android.
- Key-proxy keeps raw API keys out of the rootfs (SEC-004).
- Hybrid builds: local for web and Rust/Go; GitHub Actions for APK and complex desktop bundles.

### Workspace
- Cargo workspace, members `crates/*` (13 crates). workspace.package: version 0.1.0, edition 2021.
- workspace.dependencies: dioxus 0.7 (desktop, document), ureq 2 (json), zip 2, serde 1 (derive), serde_json 1, sha2 0.10, hex 0.4, arboard 3, dirs 6, tokio 1 (rt, time), rfd 0.15, and path deps innology-files and innology-engine.
- Comment in the manifest: "Real per-feature dependencies are added by docs/plans/ feature plans (YAGNI)."

### Crates

| Crate | Responsibility (from manifest description) | Real deps today |
|---|---|---|
| innology-app | Dioxus app shell: UI, routing, design system, platform glue. Binary `innology-code`. | dioxus, ureq, zip, serde, serde_json, sha2, hex, dirs, tokio, innology-files, innology-engine; arboard and rfd on non-Android |
| innology-engine | Embedded inference: GGUF model store and llama.cpp generation (ADR-0013) | ureq, dirs, llama-cpp-2 0.1 (desktop uses openmp; Android uses android-shared-stdcxx) |
| innology-files | Workspace and file operations, filesystem watcher, ripgrep project search | none yet |
| innology-core | Control backend (axum), session auth, API contract, WebSocket hub | none yet |
| innology-proc | PTY sessions (portable-pty), process lifecycle, resource watchdogs | none yet |
| innology-env | Rootfs provisioning, proot launcher control, toolchain manager | none yet |
| innology-git | gitoxide or git2, provider auth, branch/diff/commit | none yet |
| innology-build | Local builders, project task detection, GitHub Actions client | none yet |
| innology-containers | Local OCI runtime (udocker style), remote daemon wizard | none yet |
| innology-secrets | Encrypted credential vault (Keystore/DPAPI), local key-proxy | none yet |
| innology-tunnel | cloudflared lifecycle, vhost allowlist, tunnel link signing | none yet |
| innology-share | Artifact library, static server, HMAC-signed expiring links | none yet |
| innology-ai | opencode client (serve API), skills manager, agent audit log | none yet |

Only innology-app and innology-engine have dependencies. The other 11 are empty scaffolds; the planned stack (axum, portable-pty, gitoxide, rusqlite) is not declared anywhere yet.

### Engineering takeaways
- The security model is designed before features: socket-level peer credentials, separate tunnel scope, key isolation. Reuse this pattern for any framework that exposes a local dev server.
- Crate-per-capability split mirrors the framework blueprint in FRAMEWORK_ENGINEERING_FUNDAMENTALS.md (core, files, build, env, share).

---

## 2. innologylock (Tauri 2, Windows locker app)

Path: D:\projects\code\innologylock\src-tauri
Purpose: Windows 10/11 app that locks folders and files. Version 0.0.39, by InnologyBD.

### Rust dependencies
- Local: innologylock-shared (path ../shared).
- Tauri: tauri 2.0 (tray-icon, image-png), tauri-plugin-updater 2, tauri-plugin-process 2, tauri-plugin-dialog 2, tauri-build 2.0.
- Crypto and auth: argon2 0.5 (zeroize), aes-gcm 0.10, totp-rs 5.5 (qr), sha1, sha2, hmac 0.12, zeroize 1.7, subtle 2.5, getrandom 0.2.
- Encoding and utility: serde, serde_json, base64 0.21, base32 0.4, url, uuid v4, urlencoding, open 5, once_cell.
- Async and network: tokio 1.35 (full), reqwest 0.12 (json, rustls-tls).
- System: sysinfo 0.30, windows-sys 0.61 (Foundation, Threading, ProcessStatus, Security, Storage_FileSystem, UI_WindowsAndMessaging, UI_Shell and more), windows 0.61 (UI_Shell, System_Com, Storage_Provider, Storage_Search).

### tauri.conf.json
- Plugins: only `updater` is configured. Endpoints are GitHub raw and GitHub Pages `latest.json`; a minisign public key is embedded.
- process and dialog are in Cargo but have no config entries. Not verified whether they need one.
- Build: beforeDevCommand `npm run dev`, devUrl localhost:1420, frontendDist `../dist`.
- Window: 1280x800, minimum 1024x680.
- CSP: `default-src 'self'`, but `script-src 'self' 'unsafe-inline'` and `style-src 'self' 'unsafe-inline'`. The inline script allowance weakens XSS protection and should be reviewed.
- Bundle: NSIS only, createUpdaterArtifacts true, installMode both. Bundled resources are `InnologyLock-svc.exe` and `InnologyLock-monitor.exe`, which implies a two-process service and watchdog design.
- Cosmetic bug: copyright string contains mojibake (`Â©`).

### Engineering takeaways
- Good crypto stack choices (argon2 for passwords, aes-gcm for data, zeroize for secrets in memory).
- Windows-specific security surface is large (raw windows-sys calls); keep these behind a small module boundary.
- Service plus monitor split is the right pattern for a locker that must survive user attempts to kill it, but it needs a careful threat model.

---

## 3. NetworkFix (Rust CLI and GUI)

Path: D:\projects\code\NetworkFix
Purpose: hotspot and cellular network repair. MIT, version 0.1.0. Release profile: lto = true, strip = true.

| Crate | Role (inferred from name and manifest; no description field) | Deps |
|---|---|---|
| networkfix-core | Shared library | serde, serde_json; dev: tempfile |
| networkfix | CLI binary `networkfix` | networkfix-core, clap 4 (derive), serde_json; winresource on Windows |
| networkfix-gui | GUI binary `networkfix-gui` with feature-gated UIs | networkfix-core; optional dioxus 0.6 (desktop), tokio (sync), windows-sys 0.61 |

networkfix-gui features:
- `default = ["webview-ui"]`: Dioxus desktop, needs WebView2 (Windows) or WebKitGTK (Linux).
- `native-ui`: pure Win32 via windows-sys, no browser runtime. Manifest says it targets i686-win7-windows-msvc.

Packaging via cargo-packager: ships both binaries; Linux .deb depends on libwebkit2gtk-4.1-0.

Notes: NetworkFix uses Dioxus 0.6 while innology-code uses 0.7. Versions are not shared.

### Engineering takeaways
- Same core library, two front ends (CLI and GUI). Good pattern: business logic in a core crate, UI as a thin adapter.
- Feature flags select the rendering backend at compile time, which also gives a legacy-OS path (Win7).

---

## 4. Testing setups

### OPMS (React + Vite + Firebase)
- Vitest (vitest.config.ts): environment `node`, TZ forced to Asia/Dhaka, include `src/**/*.test.ts`. About 10 tests, all pure logic: format, greeting, idb, mx, password, storage-util, table, totp, disposable-email, and a Firebase SWR data test.
- No component or DOM tests (node environment).
- Playwright (playwright.config.ts): testDir `tests/e2e`, Chromium only, 60s timeout, 2 retries in CI, baseURL localhost:5173, webServer runs `npm run dev` with reuseExistingServer outside CI. One spec: journey.spec.ts.

### innofinance (React + Vite + Firebase + Netlify)
- Vitest only: environment jsdom, globals on, React SWC plugin, `@` alias, 15s timeout. Include `src/**/*.{test,spec}.{ts,tsx}`. About 80 test files, all in `src/test/`.
- No Playwright. No browser end-to-end tests.
- Coverage areas:
  - Finance logic: budget rollover, currency and rebase, debt planner, recurring schedules, spending review, bill detection, family ledger, CSV and spreadsheet import.
  - Reminders and notifications: reminder engine, push queue, quick actions, overdue, native and Windows bridges.
  - Browser extension: background, popup operations, reminder scheduler and delivery, Web Speech.
  - Native Android manifest, capabilities, diagnostics.
  - AI: engine, auth, CORS, reminder assistant, recommendations, document suggest, prescription extract, health scan.
  - Serverless (Netlify and Firebase functions): admin core, function discovery, send message, site analytics, backup service, error logger.
  - Auth and security: auth redirect, storage permission, password history.
  - Routes and pages: home, contact, FAQ, privacy, visitor tracker.
  - Voice: voice service, input parser, speech recognition.
  - Other: weather, prayer times, location, theme preference, notes, date utils, and a test that checks the Vite dev setup itself.

### Open_Erp (React + Vite + Supabase + TanStack Router)
- Vitest: environment jsdom, React plugin, `@` alias. `pool: 'threads'` is set deliberately; the config comment says the default forks pool fails on this Windows host. Coverage config: v8, text and html reports.
- About 30 test files grouped by ERP module: routing and nav reachability, auth (idle logout, bootstrap, MFA), HR (employee master, directory, onboarding, transfer, leave, attendance), FICO (archive, setup screen), SD archive, MM purchase requisitions, MFG production screens, favourites, product table.
- Playwright: testDir `e2e`, Chromium, fullyParallel. Uses port 4173 on purpose (preview build) instead of 5173, because reuseExistingServer once let an e2e run attach to the dev server. Runs `bun run build && bun run preview --strictPort`. CI: 2 retries, 1 worker, HTML report.
- Six specs: smoke, sign-in, anonymous-routing, dash, favourites, account-settings. `e2e/probe-env.ts` is a helper.

### Cross-project testing observations
1. Test strategy is inconsistent: OPMS is logic-only, innofinance is unit and route-level with no e2e, Open_Erp has both unit and e2e.
2. Port choice for e2e matters: Open_Erp's comment explains a real false-positive failure mode.
3. Windows-specific workarounds appear in configs (threads pool). Keep this in mind for any test tooling a custom framework ships.
4. Architecture tests (arcovia) are used in OPMS and innofinance per the earlier dependency study; not re-verified here.

---

## 5. DriveBackup (Python desktop app)

Path: D:\projects\code\DriveBackup
Purpose: Google Drive backup, verify, and wipe, driven by rclone. Includes Immich and photo support and an AI analysis and report module. UI is NiceGUI shown in a pywebview window.

### Dependencies
Only requirements.txt, with lower and upper bounds only:
- requests >=2.31,<3
- nicegui >=3.0,<4
- pywebview >=5.0,<6
- seleniumbase >=4.0 (no upper bound; import location not verified)

No pyproject, setup.py, Pipfile, or lock file. Builds are not reproducible across machines.

### Structure (32 Python files)
- Entry and tooling: main.py, build.py (packaging), create_oauth.py.
- app/utils: version, updater, logging, config.
- app/gui: app, pages, widgets, workers, context.
- app/engine: backup, verify, wipe, rclone_manager, photos, immich.
- app/ai: analyzer, llm, report.
- Root-level scripts: smoke_test.py, gui_test.py, ai_test.py, ai_live_test.py, wipe_test.py. Names do not match pytest's default test_*.py pattern, so a plain pytest run may skip them. Contents not read.
- tests/test_ux.py: the only file in tests/. Headless NiceGUI smoke test: every tab must build and render. It redirects APPDATA to a temp dir before importing app modules, and copies rclone.exe from the real install (read only).

### Engineering takeaways
- Wrapping an external CLI (rclone) behind a manager module is a clean boundary; keep the same pattern for any tool a framework shells out to.
- Headless UI tests for a NiceGUI app are a useful model: the test renders real components without opening a window.
- Pin dependencies (lock file) before packaging for other machines.

---

## 6. Cross-cutting findings

1. innology-code docs lag the roadmap; treat ROADMAP.md as the source of truth.
2. Most innology-code crates are empty scaffolds.
3. InnologyLock's CSP allows inline scripts.
4. DriveBackup and innology-code both lack lock files or pinned versions in some places.
5. Dioxus versions differ across projects (0.6 in NetworkFix, 0.7 in innology-code).
6. Two front-end testing strategies coexist with no shared harness; a custom framework could ship one default test setup (Vitest plus Playwright on a preview build, not the dev server).

## 7. Not verified
- Existence of src/test/setup.ts referenced by the Vitest configs.
- Contents of vite.config.ts files and the DriveBackup root scripts.
- Whether innologylock needs config entries for the process and dialog plugins.
