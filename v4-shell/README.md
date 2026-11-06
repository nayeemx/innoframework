# v4-shell

Tauri 2 desktop shell wrapping a React client. RPC via Tauri invoke commands (not HTTP). CSP strict, no unsafe-inline scripts.

## Architecture decision
Tauri 2 chosen over Dioxus: reuse existing React client + Rust backend, native webview, smaller binary, no rewrite.

## Build
npm install --ignore-scripts
npm run build
cd src-tauri && cargo build

## Auth guard
Reused from v1 conceptually (session cookie). In desktop shell, auth is implicit (local app); RPC commands are registered in Rust invoke handler.
