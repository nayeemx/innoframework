# TR-2.1 Fresh-VM Install Test (P3 gate)

Status: **not executed on a clean VM in this environment** (developer workstation). Automated checks below cover the local toolchain path; a true fresh-VM run remains an owner/CI action.

## Preconditions (fresh Windows 11 VM, no Rust/dev tools)
- Windows 11 x64 (Pro/Enterprise/Home) on a VM snapshot.
- Network access to `static.rust-lang.org`, `crates.io`, `github.com`.
- No preinstalled: Rust/rustup, cargo, wasm32 target, `dx`/dioxus-cli, Visual Studio Build Tools (note if required by linker).

## Steps
1. Install Rust stable via rustup (default profile). Verify: `rustc --version`, `cargo --version`.
2. Add WASM target: `rustup target add wasm32-unknown-unknown`.
3. Install Dioxus CLI: `cargo install dioxus-cli --version 0.7.10`. Verify: `dx --version`.
4. Obtain `inno` binary (from release artifact or `cargo install --path crates/cli` from repo checkout).
5. Scaffold: `inno init --name demo -y` in an empty temp dir. Expect exit 0 and files `demo/inno.toml`, `demo/Cargo.toml`, `demo/src/main.rs`.
6. Build: `cd demo; inno build web`. Expect exit 0 and `target/dx/demo/release/web/public/{index.html,csp-header.txt,assets/*.wasm,assets/*.js}`.
7. Serve: `inno dev --port 18765` (background). `GET http://127.0.0.1:18765/` must return `200` with headers `Content-Security-Policy` (default-src 'self'; script-src 'self'; ...) and `X-Content-Type-Options: nosniff`.
8. Stop dev server; record timings and any missing-tool errors (should be actionable AUD codes/fix text).

## Pass criteria
- Steps 1-7 complete without manual source edits.
- `inno build web` produces a WASM bundle and `csp-header.txt`.
- `inno dev` serves CSP on the wire (TR-4.1/CSP-1).
- Missing toolchain produces an actionable error (TR-2.4).

## Local evidence (2026-11-08, developer machine)
- `inno init --name demo -y` -> exit 0.
- `inno build web` (`dx build --platform web --release`) -> exit 0, `target/dx/demo/release/web/public/{index.html,csp-header.txt,assets/*.wasm,assets/*.js}`.
- `inno dev --port 18765` -> `GET /` and `GET /csp-header.txt` returned `200` with `Content-Security-Policy` and `X-Content-Type-Options: nosniff`.

## Owner-deferred (intentional)
- TR-2.1 has not been run on a clean Windows VM in this environment; the local toolchain path is verified above. Run on a clean VM/image before treating the gate as closed.
- Trademark clearance, license/publication decisions, and ADR-09 signing are intentionally deferred while this is in development with no product/public-release decision. No action required now.
