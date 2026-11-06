# How to Run

All commands and results below were executed and observed on this machine.

## Prerequisites

- Node v26.5.0 and npm 11.17.0 (verified via `node --version` / `npm --version`).
- `npm install` requires `--ignore-scripts` here: the esbuild postinstall binary validation fails with `EFTYPE` on Node 26.

## v1 - portfolio (TypeScript fullstack)

```powershell
cd D:\projects\code\portfolio
npm run dev              # client 8080 + server 3001 via concurrently/tsx
npx tsx server/index.js  # server only
npx vitest run           # 11 passing
npm run build            # vite build + "check:secrets passed"
```

`npm run dev` runs `concurrently "npm:dev:client" "npm:dev:server"`: Vite on 8080, `tsx watch server/index.js` on 3001.

Observed: `npx vitest run` -> 3 files, 11 tests passed. `npm run build` -> Vite build succeeded, then `check:secrets passed - no server-only secrets in client code.`

### Auth — signed session tokens

`portfolio/src/server/auth.ts` issues **signed** tokens:

```
<base64url(payload)>.<base64url(hmac-sha256(payload, secret))>
```

`payload = { userId, role, exp }`. Verification uses `timingSafeEqual` and rejects
expired sessions. `SESSION_SECRET` env var; dev fallback
`v1-dev-only-insecure-session-key-change-me` (documented, not committed to prod).

`portfolio/src/test/auth.test.ts` — 10 tests (signature valid, tampered signature
rejected, expired rejected, wrong secret rejected, role enforcement 403).

### Verified endpoints

| Request | Observed result |
| --- | --- |
| `GET /health` | 200 `{"ok":true}` |
| `POST /health` | 200 `{"ok":true}` (added) |
| `POST /rpc/getMessage` no cookie | 401 `{"error":"Unauthenticated"}` |
| `POST /login` | 200 + `Set-Cookie: v1_session=...; HttpOnly; SameSite=Lax` |
| `POST /rpc/getMessage` + cookie | 200 `{"ok":true,"text":"Hello from portfolio v1 server function."}` |
| `POST /rpc/agent` weather | 200, `toolCalls: [{ toolName: "getWeather", input: {"city":"Lahore"}, output: {"temperature":28,"condition":"sunny"} }]` |
| `POST /rpc/agent` prompt-injection | 422 `{"error":"Prompt injection detected: ignore-previous, leak-system-prompt"}` |

The `/demo` frontend route renders `ServerFunctionDemo` (wired in `src/App.tsx`).

## v4 - Tauri 2 cross-platform desktop shell

```powershell
cd D:\projects\code\framework-research\v4-shell
npm run build            # tsc + vite build, clean
cd src-tauri && cargo build   # emits target/debug/v4-shell.exe
cd src-tauri && cargo test    # 3 auth tests pass
```

`npm run build` -> 34 modules transformed, gzip 46.46 kB. `cargo build` -> `Finished dev profile`.
`cargo test` -> 3 passed (guarded commands fail closed without session;
authorize passes after unlock; authorize fails again after lock).

### Commands

| Command | Guard | Purpose |
| --- | --- | --- |
| `unlock` | — | establishes a local session (returns role) |
| `lock` | — | clears the session |
| `health` | — | reports whether a session exists |
| `get_message` | `authorize` | fail-closed without session |
| `get_profile` | `authorize` | fail-closed without session |

`authorize()` mirrors v1's `authGuard`: any protected command returns
`"Unauthenticated"` until `unlock` has been called. The Rust unit tests prove
the fail-closed default and the lock→unauthorized cycle.

### Live run

`target/debug/v4-shell.exe` starts and stays resident (verified — no crash on
startup). The window renders the Unlock/Lock UI; `get_message` fails closed by
default and returns `Hello from v4 Tauri shell.` after `unlock`.

### Security

`tauri.conf.json` enforces a strict CSP: `default-src 'self'; script-src 'self';
style-src 'self'; frame-ancestors 'none'; object-src 'none'; form-action 'self';`
— no `unsafe-inline`, no external origins.

## v2 - ai-slice (AI agent module)

```powershell
cd D:\projects\code\framework-research\ai-slice
npx vitest run              # 15 passing
npx tsc --noEmit            # clean (no output)
node scripts/check-secrets.js  # passed
```

Observed: `npx vitest run` -> 1 file, 15 tests passed. `npx tsc --noEmit` -> no output, exit 0. `check-secrets.js` -> `check:secrets passed - no model API keys in client code.`

### Local model

```powershell
npm run start-llama   # D:\models\llama.cpp\b9142\llama-server.exe on 127.0.0.1:11434
npm run stop-llama
```

`start-llama` launches the server with the llama3.2-3b blob `D:\models\blobs\sha256-dde5aa3fc5ffc17176b5e8bdc82f587b24b2678c6c66101bf7da77af9f7ccdff` and `--ctx-size 4096`. `stop-llama` runs `taskkill /F /IM llama-server.exe`.

Verified real round-trip: 5336 ms elapsed, reply exactly `Hello, blue.`; `/v1/models` reported `n_params 3212749888`, `n_ctx 4096`.

## Known limitations

- `/rpc/agent` in the Express server uses `MockModelProvider`. `LlamaCppProvider` is implemented and live-tested but is not wired into that route.
- Slices are local projects, not published packages.
- No streaming support.
- Prompt-injection detection is regex-based and not exhaustive.
- The session token is unsigned base64 JSON (demo only, not production auth).

## Troubleshooting

- Do not run `node server/index.js` for the portfolio server. Node type stripping erases type-only exports, so it fails with `SyntaxError: The requested module './model.ts' does not provide an export named 'Message'`. Always use `tsx` or the npm scripts.

## v3 - native integrations (Rust/WASM + Go) behind typed clients

### Toolchain found

| Tool | Version | Notes |
| --- | --- | --- |
| rustc | 1.97.1 | with `wasm32-unknown-unknown` target installed |
| cargo | 1.97.1 | |
| wasm-pack | not found | used `cargo build --target wasm32-unknown-unknown --release` |
| go | 1.26.5 | |
| node | v26.5.0 | |
| npm | 11.17.0 | `npm install --ignore-scripts` required (esbuild postinstall EFTYPE on Node 26) |

### v3-rust (Rust/WASM)

```powershell
cd D:\projects\code\framework-research\v3-rust
npm install --ignore-scripts
npm run build:wasm        # cargo build + wasm-bindgen --target web --out-dir pkg
npx vitest run           # 7 passing
node benchmark.ts        # prints native vs JS numbers
```

`wasm-bindgen-cli v0.2.130` is installed (cargo). The crate exposes
`fibonacci(n: i64) -> i64` via `#[wasm_bindgen]`; the generated `pkg/` glue
(`v3_rust.js`, `v3_rust.d.ts`, `v3_rust_bg.wasm`) is loaded by `client.ts`
through `initSync`, so the typed client calls `fibonacci(BigInt(n))` directly
instead of poking raw WebAssembly exports.

Observed benchmark (`fibonacci(n=1,000,000) mod 1_000_000`, 50 iterations):

| Metric | Value |
| --- | --- |
| expected result | 546875 |
| native mean | 5.177 ms |
| JS mean | 11.853 ms |
| speedup | 2.29x |
| module load time (first call incl. compile) | 19.675 ms |

### v3-go (Go HTTP service, optional)

```powershell
cd D:\projects\code\framework-research\v3-go
npm install --ignore-scripts
go build -o bin/v3-go.exe .
.\bin\v3-go.exe        # listens on :8081
npx tsc --noEmit       # typecheck clean
```

Exposes `POST /health` and `POST /fibonacci` (typed JSON) plus a generated TypeScript client stub (`client.ts`) with zod validation.

### Wired into portfolio server

`POST /rpc/native/<name>` added to `D:\projects\code\portfolio/server/index.js`, guarded by the existing `requireAuth` auth guard. Only `fibonacci` is supported.

| Request | Observed result |
| --- | --- |
| `POST /rpc/native/fibonacci` no cookie | 401 `{"error":"Unauthenticated"}` |
| `POST /rpc/native/fibonacci` + `v1_session` cookie | 200 `{"ok":true,"name":"fibonacci","result":55,"n":10}` |

### Regression checks (all observed passing)

| Project | Command | Result |
| --- | --- | --- |
| portfolio | `npx vitest run` | 11 passed |
| portfolio | `npm run build` | Vite build OK + `check:secrets passed` |
| ai-slice | `npx vitest run` | 15 passed |
| ai-slice | `npx tsc --noEmit` | clean |
| v3-rust | `npx vitest run` | 7 passed |
| v3-rust | `node benchmark.ts` | prints real numbers (above) |
| v3-go | `npx tsc --noEmit` | clean |

### Known gaps

- v3 spec file `FRAMEWORK_V3_SPEC.md` did not exist in the repo; this section implements the required work described in the task.
- The native speedup is workload-dependent: for small `n` the JS↔WASM call boundary dominates and native is slower; the measured 2.29x requires ~1M iterations per call so compute dominates the boundary.
- Session tokens are signed with HMAC-SHA256 but still use the dev fallback secret (`v1-dev-only-insecure-session-key-change-me`) when `SESSION_SECRET` is unset. Set the env var before production.
- Go service was built and typechecked; live HTTP round-trip verified locally but not added to the portfolio regression test suite.
