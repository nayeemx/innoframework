# Framework Engineering Fundamentals (blueprint reference)

Purpose: the core engineering concepts needed to design a custom framework, drawn from the stacks used in D:\projects (Vite, React, Next.js, TanStack Start/Router, Svelte, Astro, Qwik, Dioxus, Tauri, Buffalo) and from current web sources (checked Nov 2026). This is a reference, not a design. No framework choice has been made yet.

---

## 1. The five decisions every framework makes

Any framework is a set of answers to these questions. Write your own answers before writing code.

| # | Decision | Options seen in the ecosystem |
|---|---|---|
| 1 | Where does UI state live and how do updates reach the DOM? | Virtual DOM diffing (React, Preact); fine-grained signals (Solid, Svelte 5 runes, Vue Vapor, Dioxus); resumable signals (Qwik) |
| 2 | Where does code run? | Client only (SPA); server render + client hydrate (Next, TanStack Start); server-only components with client islands (Astro, RSC); resumable (Qwik) |
| 3 | How are routes defined? | File-system (Next, TanStack Start file routes, Astro, Fresh); code-defined route tree (TanStack Router, Dioxus); both |
| 4 | How is the build organized? | Bundler plugin pipeline (Vite on Rolldown, webpack); compiler that emits JS (Svelte, Solid, Vue SFC); native binary (Dioxus, Tauri backend) |
| 5 | How do client and server talk? | HTTP route handlers (Buffalo, Express); typed RPC "server functions" (TanStack Start, Dioxus, Next server actions); realtime channel (socket.io, LiveKit) |

---

## 2. Reactivity models (core of runtime design)

### 2.1 Virtual DOM (React model)
- Component re-runs on state change, builds a new tree, diffs against the old one, patches DOM.
- Pros: simple mental model, huge ecosystem, works with any reconciler (React Native, Three.js renderers via react-three-fiber).
- Cost: work scales with component tree size, not with what changed. Mitigated with memo, keys, and `useMemo`.
- Key source to read: `vuejs/core` `packages/runtime-core/src/renderer.ts` (renderer, patch, keyed-diff with a longest-increasing-subsequence step).

### 2.2 Signals (push-pull dependency graph)
- A signal holds a value and a set of subscribers. Reading inside an "effect" subscribes it. Writing marks dependents dirty and re-runs only them.
- Core primitives: `signal`, `computed` (lazy, memoized), `effect` (side effect). Batching groups writes into one flush.
- Minimal implementation outline (about 150 lines in practice): a global "currently running" context; `get()` registers the context as subscriber; `set()` notifies; `computed` caches with a dirty flag.
- Used by: Solid, Svelte 5 (runes `$state`, `$derived`, `$effect`), Vue 3 (`ref`, `computed`), Preact Signals, Dioxus (`use_signal`), Angular signals, Qwik.
- Reference reading: Nolan Lawson, "Let's learn how modern JavaScript frameworks work by building one" (nolanlawson.com, 2023-12-02). Covers signals, template cloning, and Proxy-based state as the three shared ideas of post-React frameworks.
- Two implementation choices to decide: Proxy-based (automatic, Vue/Svelte style) versus explicit getter/setter (Solid/Preact style). Proxy costs identity and debugging clarity; explicit costs ergonomics.

### 2.3 Fine-grained DOM updates via cloned templates
- Instead of rebuilding a vnode tree, the compiler/runtime clones a `<template>` once and keeps direct references to the dynamic nodes. A signal change writes only to that node.
- Svelte, Solid, Vue Vapor, Dioxus (RSX compiled to templates) follow this. The compiler is what makes it cheap.

### 2.4 Decision for your framework (checklist)
- Do you need a compiler (`.svelte`-style single-file syntax) or a runtime-only API (plain JS/TS functions, Solid-like `jsx`)?
- Proxy or explicit signals?
- Is the unit of update a component (React) or an expression (Solid)?

---

## 3. Rendering modes

| Mode | What the server sends | Client work | Example |
|---|---|---|---|
| CSR (client-side render) | empty shell + JS | build entire DOM | Vite React SPA (most projects in D:\projects\code) |
| SSR + hydration | full HTML + JS | attach handlers and rebuild component tree | Next.js pages router, TanStack Start full SSR |
| Streaming SSR | HTML chunks as ready (Suspense boundaries) | hydrate as chunks arrive | TanStack Start, Next App Router, Dioxus fullstack |
| SSG | pre-rendered HTML at build | optional hydrate | Next static export, Dioxus SSG, Astro default |
| Islands | static HTML + per-island JS | hydrate only marked components | Astro (`client:visible`, `client:idle`), Fresh |
| Resumability | HTML with serialized state and handler references | almost none; lazy-load handler on event | Qwik (`$` suffix marks lazy boundaries) |
| RSC (server components) | serialized component tree | client components hydrate | Next App Router, `@vitejs/plugin-rsc` |

Source material: patterns.dev islands page; Qwik resumable architecture (islands-architecture.com, 2024-02-15); Astro + Cloudflare deep dive (fattoruso.dev, 2026-04-17); Egnworks islands guide (2026-08-30).

Key facts:
- Islands and progressive hydration are different. Progressive hydration still hydrates one tree in some order. Islands never assemble one client tree.
- Hydration cost is the main reason for resumability. Qwik serializes signal graph and closures into HTML attributes (`q:state`) and downloads handler chunks on interaction.
- Hydration mismatches are a whole class of bugs: any `window`/`matchMedia`/`Date.now()` read during SSR must be guarded (see Svelte `MediaQuery` docs warning).

---

## 4. Routing

### 4.1 File-system routing
- A build step (or dev server) maps `pages/blog/[slug].tsx` to a route pattern, generates a route table, and code-splits per route.
- Needs: convention for layouts (`layout.tsx`), loading/error boundaries, catch-all segments, route groups.

### 4.2 Code-defined route tree
- Routes are objects with `path`, `component`, `loader`, `beforeLoad`, search-param schema, and children. TanStack Router is the reference: route tree is typed end to end, search params are validated, loaders run before render.
- Dioxus has typed routing with an enum (`#[derive(Routable)]` style): route is a Rust enum, URL parsing is generated.

### 4.3 Concepts you must implement either way
- Matching (specificity, params, wildcards)
- Nested layouts with outlet
- Navigation (history API, scroll restoration, prefetch)
- Data loading tied to route (loader before component)
- Pending states and error boundaries per route segment
- Code splitting per route (dynamic import at route boundary)

---

## 5. Server functions and RPC

The modern pattern: write a function, call it from the client like a local function, it runs on the server.

- TanStack Start: `createServerFn({ method }).validator(schema).handler(fn)`. Compiler replaces the body on the client with a fetch call. Validation runs on the server.
- Dioxus: `#[server]` attribute function; generated client stub.
- Next.js: Server Actions (`"use server"`), form-action integration.
- Buffalo: no RPC; explicit actions per route with request binding (`c.Bind(&model)`).

What the compiler/transform must do:
1. Detect the server boundary (directive or attribute).
2. On server build: keep the body, expose an HTTP endpoint with a generated id.
3. On client build: replace body with a typed fetch stub.
4. Serialize arguments and results (JSON, or a richer format for Date/Map/BigInt).
5. Attach middleware (auth, context, logging) to the endpoint.

Security notes from this study:
- Validate input on the server even when the client already validates (react-hook-form + zod on client does not replace server checks).
- Server functions are public HTTP endpoints. Auth middleware is required per function, not per page.

---

## 6. Build pipeline and plugin system

### 6.1 Vite / Rolldown (current state, checked Nov 2026)
- Vite is a dev server over native ES modules plus a production build. Rolldown is the Rust bundler that Vite now uses for builds (Vite docs: "A build command that bundles your code with Rolldown").
- Oxc toolchain handles parsing/transform natively; Babel is only used by framework plugins when needed; SWC is an optional swap for plugin-heavy dev setups.
- Plugin API is a superset of Rollup's: hooks such as `resolveId`, `load`, `transform`, `generateBundle`, plus Vite hooks `configureServer`, `handleHotUpdate`, `transformIndexHtml`.
- `enforce: 'pre' | 'post'` controls order relative to Vite core plugins. `apply: 'build' | 'serve'` restricts when a plugin runs.
- Virtual modules: resolve an id with a `\0` prefix (`virtual:posts`) so other plugins skip it. Used for build-time data injection.
- Presets: a plugin can return an array of plugins (flattened). Frameworks use this (e.g. `framework()` returns `[refresh, devtools]`).
- Rolldown plugin compatibility: many Rollup plugins work unchanged; some build-only hooks are not supported (`shouldTransformCachedModule`). Use `build.rolldownOptions.plugins` for build-only ones.
- Source: vite.dev/guide/api-plugin, rolldown.rs/apis/plugin-api, vite.dev/guide/philosophy.

Key design lesson from Vite (applies to any custom framework):
- Keep the core lean and framework-agnostic. Put framework behavior in plugins that compose. Vite's own philosophy page states that frameworks should be built on Vite's primitives (dev server, SSR primitives, JS API).

### 6.2 Module graph
- Dev servers maintain a graph: each module knows its importers and imports. A changed file invalidates its node and dependents; HMR boundaries stop propagation.
- `handleHotUpdate` receives `modules: ModuleNode[]` (a single file can map to several served modules, e.g. a Vue SFC).
- Exposed via `server.moduleGraph` in Vite. Needed for: HMR, CSS dependencies, SSR externals.

### 6.3 Virtual module convention
- `\0` prefix on resolved id. Prevents node resolution and sourcemap confusion.
- Use it when a plugin generates a module that has no file on disk (route tree, env, content collections).

### 6.4 Environment API
- Vite's Environment API separates client, SSR, and RSC environments with their own module graphs and configs. `@vitejs/plugin-rsc` builds on it. If the custom framework has more than one runtime (browser, server, edge), design environments explicitly from the start.

### 6.5 Compile-time vs runtime boundary
- Svelte and Solid push work to compile time (smaller runtime, more build complexity).
- React and Preact keep work at runtime (simpler build, larger runtime).
- Choose based on bundle budget versus build complexity you can maintain.

---

## 7. Native and desktop targets (for cross-platform goals)

- Tauri v2: system webview frontend (any Vite app) plus Rust backend; IPC via `invoke`; plugin system (fs, dialog, shell, updater). Used in D:\projects (imrul, innologylock).
- Dioxus: Rust-native renderer. Web (WASM), desktop, mobile from one codebase. Blitz (WGPU-based HTML/CSS renderer) is the newer native path. Hot reload via Subsecond.
- Tradeoffs to decide: webview (smaller binary, consistent with web, depends on OS webview versions) versus native renderer (control, larger effort, less web compatibility).

---

## 8. Backend and data layer patterns

- Full-stack Node: Express, Fastify, or Hono for HTTP; socket.io for realtime (used in my-vps).
- Edge: Cloudflare Workers (V8 isolates, bindings to D1/KV/R2/Durable Objects). Used in innofinance/functions/worker.
- Go: Buffalo (MVC, Pop ORM, Plush templates). Maintenance mode per study.
- BaaS: Firebase (Firestore + Auth + Functions), Supabase (Postgres + RLS), Appwrite. Each has different auth and data-access models; a framework should abstract "who can read this row" as a first-class concept, not an afterthought.
- ORM options seen: Pop (Go), drizzle-orm (in UNIQUE_DEPENDENCIES), Prisma (sponsor list).

---

## 9. Core engineering concepts checklist (what you must understand, not just use)

1. Parsing and ASTs: tokenizer, parser, visitor pattern (zimmerframe is used by Svelte's compiler for visitors). Needed for any compiler or transform.
2. Code generation and source maps.
3. Dependency graphs and topological update order (signals, module graph, build graph).
4. Diffing algorithms: keyed list reconciliation (longest increasing subsequence used in Vue's renderer).
5. Event delegation and the event loop: microtasks vs tasks in effect flushing.
6. HTTP semantics: streaming responses, cache headers, content negotiation.
7. Serialization: how state crosses server/client boundaries (JSON limits, Date/Map/Set, circular refs, Qwik's serialization of closures).
8. Module formats: ESM, dynamic import, code-splitting, tree-shaking (requires side-effect-free exports).
9. Hydration: DOM node reuse, mismatch detection, markers.
10. Plugin ordering and hook lifecycle.
11. Error boundaries and error propagation across server and client.
12. Security: XSS in templates, CSRF on mutations, auth checks on every server function, secret handling in build output (never embed server env in client bundle).

---

## 10. Reading list (primary sources)

- Nolan Lawson, "Let's learn how modern JavaScript frameworks work by building one" (2023-12-02): signals, template cloning, Proxy.
- Vite plugin API and philosophy: vite.dev/guide/api-plugin, vite.dev/guide/philosophy.
- Rolldown plugin API: rolldown.rs/apis/plugin-api.
- TanStack Start overview and server functions: tanstack.com/start/latest/docs/framework/react/overview.
- Qwik resumability: islands-architecture.com (2024-02-15).
- Astro island architecture internals (fattoruso.dev, 2026-04-17).
- Svelte runtime internals: packages/svelte `internal/client`, `internal/server` split (DeepWiki summary of sveltejs/svelte).
- Vue renderer: vuejs/core `packages/runtime-core/src/renderer.ts`.
- Dioxus guide 0.7: dioxuslabs.com/learn/0.7/ (Subsecond hot reload, WASM splitting, manganis assets).

---

## 11. Open questions to answer before choosing an approach

These are the decisions the study cannot make for you:
1. Target runtime(s): browser only, browser + Node SSR, edge, desktop, mobile?
2. Language: TypeScript/JS (ecosystem fit with your projects) versus Rust (Dioxus-style, matches innology-code/NetworkFix) versus Go?
3. Authoring model: JSX-like, template file (`.svelte`-like), or plain function calls?
4. Reactivity: signals or virtual DOM?
5. Data: which backends must be first-class (Firebase, Supabase, Postgres, Appwrite)?
6. Who uses it: only you, or a team and external users (affects docs, stability, versioning burden)?
7. Scope of v1: smallest slice that runs one real project end to end.
