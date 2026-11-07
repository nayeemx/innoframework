# innologybd-core — TypeScript Fullstack Framework Research

A multi-target TypeScript fullstack framework research project (v1–v6) with typed
server functions, signed-session auth guards, CSP presets, build-time secret checks,
a WebView mobile shell, and a compiler frontend + MIR. The package is
published to npm as `innologybd-core@1.0.1`.

```
npm install innologybd-core
npx create-innology-app my-app
cd my-app
# NOTE: Use --ignore-scripts on Node v26 (esbuild postinstall fails otherwise)
npm install --ignore-scripts
npm run dev
```

## Repository layout

| Directory | Target | Description |
|-----------|--------|-------------|
| `framework-core/` | Package | `@innologybd`/`innologybd-core` npm package: typed server functions, auth guards, CSP, secret check, scaffolder |
| `v1-slice/` | Web | Express + Vite server with typed RPC and auth guard |
| `v3-rust/` | Native | Rust WASM + Go native endpoints |
| `v3-go/` | Native | Go native module |
| `v4-shell/` | Desktop | Tauri shell with auth-guarded commands |
| `v5-mobile/` | Mobile | WebView shell + frame-time / cold-start benchmarks |
| `v6-compiler/` | Compiler | Frontend parser + MIR + interpreter (4584x vs wrapper RPC) |
| `ai-slice/` | AI | Typed tool contracts, agent runtime, llama.cpp + ONNX + WebGPU adapters |

## Phase status

| Phase | Scope | Status |
|-------|-------|--------|
| 0 Foundation | Spec, benchmark, package over HTTP | ✅ Complete |
| 1 Web target | CLI, Vite, typed RPC, auth, Tailwind/shadcn | ✅ Complete (401/403/422 runtime tests) |
| 2 Desktop | Tauri shell, capability manifest | ✅ Complete (native vs JS measured; native removed per "or is removed" clause) |
| 3 Graphics and AI | WebGPU/Three.js, llama.cpp, ONNX adapters | ✅ Complete (adapter conformance tests, secret check blocks model keys) |
| 4 Mobile | WebView shell, frame-time/cold-start budgets | ✅ Complete (4 tests) |
| 5 Compiler | Frontend + MIR, wrapper vs direct benchmark | ✅ Complete (4584x speedup, 3 tests) |

## Using the published package

```bash
# Install
npm install innologybd-core

# Scaffold a new v1 app
npx create-innology-app my-app
cd my-app
npm install
npm run dev          # client + server via concurrently

# Run the build-time secret check (fails if server secrets leak to client)
npm run check:secrets
```

The scaffolder generates an Express server with guarded `/rpc/*` routes, a typed
client stub, Tailwind CSS, vitest config, and an `auth.test.ts` with runtime
negative tests (401/403/422).

## Running targets locally

```bash
# framework-core (package)
cd framework-core
npm install
npm run test           # 29 tests
npm run verify:pack    # proves tarball is installable

# v1-slice (web)
cd v1-slice
npm install
npm run test           # 7 tests

# v3-rust (WASM + native)
cd v3-rust
npm install
npm run test           # 7 tests

# ai-slice (AI agent + adapters)
cd ai-slice
npm install
npm run test           # 29 tests

# v5-mobile (benchmarks)
cd v5-mobile
npm install
npm run test           # 4 tests (frame-time + cold-start)

# v6-compiler (MIR benchmark)
cd v6-compiler
npm install
npm run test           # 3 tests (direct MIR 4584x vs wrapper RPC)
```

## Auth model

Sessions are HMAC-signed tokens:
```
<base64url(payload)>.<base64url(hmac-sha256(payload, secret))>
```

Verification uses `timingSafeEqual` (constant-time) and rejects expired sessions.
`SESSION_SECRET` is required in production; dev fallback is documented as insecure.

Auth guard is fail-closed: no session → 401, wrong role → 403, forged/expired/malformed tokens rejected.

## Publishing

```bash
cd framework-core
npm run build
npm run verify:pack
npm publish --access public   # requires npm login + 2FA bypass token
```

## License

MIT
