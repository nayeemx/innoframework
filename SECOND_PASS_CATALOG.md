# Second Pass — Verified Deep Dives (batch 1: Forms + State)

## Forms & validation

### react-hook-form
- **What**: Performant, flexible, extensible React form library with easy-to-use validation.
- **Core**: useForm, register, handleSubmit, Controller, useController, Form component.
- **Validation**: HTML-standard rules (required, min, max, minLength, maxLength, pattern, validate) OR schema via resolver.
- **Schema support**: Yup, Zod, Superstruct, Joi, Vest, Class Validator, io-ts, Nope, computed-types, typanion, Ajv, TypeBox, ArkType, Valibot, TypeSchema, effect-ts, VineJS, fluentvalidation-ts, ata-validator, any Standard Schema lib. Full list in @hookform/resolvers.
- **Controlled inputs**: Controller/useController for non-native (shadcn, MUI, AntD, react-select).
- **Features**: uncontrolled by default (perf), global state integration, React Native support, TypeScript first-class, mode: onChange/onBlur/onSubmit, errors object, Form component for service submission (action, onSuccess, onError), DevTools.
- **Version note**: v7 current; v8 beta migration guide exists. Zod v4 needs resolvers ^3.10+ and tsconfig moduleResolution bundler/node16/nodenext.
- **Capability for framework**: excellent form foundation; pairs with zod for typed forms; uncontrolled perf is a differentiator.

### zod
- **What**: TypeScript-first schema validation with static type inference.
- **Core**: z.object, z.string, etc.; parse/safeParse; type inference via z.infer.
- **Features**: zero deps, 2kb gzipped core, immutable API, JSON Schema conversion, codecs, AOT compilation, metadata/registries, Zod Mini + Zod Core packages.
- **Ecosystem**: tRPC, React Hook Form, zshy. Many libs built on it.
- **Requirements**: TS v5.5+, strict mode in tsconfig.
- **Version**: v4 stable (4.6 latest). Migration guide from v3.
- **Capability for framework**: ideal for typed boundaries (API, forms, env, config); standard schema compatible.

### @hookform/resolvers
- Bridge between react-hook-form and schema libs. 20+ resolvers. Zod v4 support needs ^3.10+.

## State & data fetching

### @tanstack/react-query
- **What**: Server-state management: fetching, caching, synchronizing, updating.
- **Solves**: caching, deduping, background refetch, stale-while-revalidate, pagination, memory/gc, structural sharing.
- **Core**: QueryClient, QueryClientProvider, useQuery, useMutation, useInfiniteQuery.
- **Capability for framework**: essential for any data-driven app; zero-config defaults.

### @reduxjs/toolkit (RTK)
- **What**: Standard way to write Redux. configureStore, createSlice, createReducer, createAction, createAsyncThunk, createEntityAdapter, createSelector (Reselect).
- **Immer** built-in for mutative updates.
- **RTK Query**: data fetching/caching addon (createApi, fetchBaseQuery, ApiProvider, setupListeners). Optional.
- **Requirements**: TS 5.4+ for RTK 2.x.
- **Capability for framework**: global client state; RTK Query overlaps with TanStack Query — pick one.

### redux / react-redux / redux-persist
- redux: core store. react-redux: React bindings (Provider, useSelector, useDispatch). redux-persist: persist store to storage.

### dexie / localforage
- dexie: IndexedDB wrapper with typed tables, queries, transactions.
- localforage: simple key-value storage (IndexedDB/WebSQL/localStorage fallback).

## Capability comparison (forms)
- react-hook-form (uncontrolled, perf) + zod (typed schema) is the dominant modern combo in your projects (innofinance, e-gp tracker, portfolio, my-vps, Open_Erp).
- Alternative: Formik (not used) — heavier, controlled. RHF wins on perf.
- Alternative: TanStack Form (not used) — newer, headless, type-safe.

## Capability comparison (state)
- TanStack Query: server state (async data). RTK: client state (UI, auth, prefs). They complement; don't duplicate.
- RTK Query vs TanStack Query: RTK Query ties to Redux store; TanStack Query is standalone and more flexible. Your projects use both separately (TanStack Query in most, RTK in baishakhix/edutech/e-gp/innology-treding).

## UI primitives (verified batch 2)

### Radix Primitives
- **What**: Low-level, unstyled, accessible UI component library. WAI-ARIA compliant.
- **Key features**: accessible (aria/role, focus mgmt, keyboard nav), unstyled (any styling solution), opened (granular parts), uncontrolled by default but controllable, fully-typed API, asChild prop, incremental adoption.
- **Install**: `npm install radix-ui` (tree-shakeable umbrella) OR individual `@radix-ui/react-*`.
- **Components**: accordion, alert-dialog, aspect-ratio, avatar, checkbox, collapsible, context-menu, dialog, dropdown-menu, hover-card, label, menubar, navigation-menu, popover, progress, radio-group, scroll-area, select, separator, slider, slot, switch, tabs, toast, toggle, toggle-group, tooltip, etc.
- **Capability for framework**: foundation for shadcn/ui; best for design systems needing accessibility without styling opinions.

### Headless UI (Tailwind Labs)
- **What**: Unstyled accessible components. React + Vue.
- **Components**: Menu, Disclosure, Dialog, Popover, Tabs, Transition; Forms: Button, Checkbox, Combobox, Fieldset, Input, Listbox, Radio Group, Select, Switch, Textarea.
- **Styling**: data-* attributes + render props; Tailwind data-attribute modifier.
- **Features**: anchor positioning (CSS vars --anchor-gap/offset/padding), transitions, Framer Motion integration, portals, modal mode, keyboard interaction full.
- **Capability for framework**: lighter alternative to Radix; tighter Tailwind integration; fewer components than Radix.

### Cloud Firestore (Firebase)
- **What**: Flexible, scalable NoSQL cloud database. Documents in collections, hierarchical.
- **Key capabilities**: flexible hierarchical data structures, expressive queries (chained filters, sorting), realtime listeners (sync across clients), offline support (cache + sync on reconnect), auto multi-region replication, strong consistency, atomic batch ops, ACID transactions, MongoDB-compatible API (Enterprise edition).
- **SDKs**: native Apple, Android, web, Node.js, Java, Python, Unity, C++, Go; REST/RPC.
- **Security**: Firebase Auth + Security Rules (client), IAM (server).
- **Editions**: Standard + Enterprise (MongoDB compat).
- **Capability for framework**: primary DB in your stack; realtime + offline are differentiators; security rules need testing (arcovia/rules tests).

### TanStack Router
- Docs fetch timed out. Known from training: type-safe routing, file-based routes, loaders, search params validation, pending states, boundaries. Used in Open_Erp. Will retry in next batch.

### Supabase
- **What**: Open-source Firebase alternative. Postgres database + Auth + Storage + Edge Functions + Realtime + Vector.
- **Key features**: Postgres (full SQL, RLS, migrations via CLI), Auth (email, social, magic link, MFA), Storage, Edge Functions (Deno), Realtime (broadcast, presence, postgres changes), Vector embeddings (pgvector), local dev via CLI.
- **Framework quickstarts**: React, Next.js, TanStack Start, Astro, Vue, Nuxt, SvelteKit, SolidJS, RedwoodJS, Refine, Hono, iOS, Android, Expo, Flutter, Python, Laravel, Rails, Spring Boot.
- **Capability for framework**: SQL/Postgres + RLS is a strong alternative to Firestore for relational data; your only usage is Open_Erp.

### TanStack Router (retry)
- Docs still timed out. From training: type-safe routing with file-based routes, loaders, search params validation, pending states, error boundaries, code splitting. Used in Open_Erp with TanStack Query.

### Tauri (v2)
- **What**: Framework for tiny, fast binaries for desktop (and mobile) platforms. Frontend = any web framework (HTML/JS/CSS); backend = Rust (also Swift/Kotlin for mobile plugins).
- **Advantages**: secure foundation (Rust memory/thread/type safety, security audits), small app size (<600KB minimal, uses system webview), flexible architecture (any frontend, invoke JS?Rust).
- **Core**: TAO (window creation), WRY (webview rendering). IPC via `invoke`. Plugins: fs, dialog, shell, updater, notification, SQL, store, HTTP, websocket, global shortcut, autostart, clipboard, deep-link, etc.
- **Distribute**: App Store, AppImage, AUR, Debian, DMG, Flathub, Google Play, MS Store, RPM, Snapcraft, Windows Installer. Signing for all OS.
- **Frontend support**: Vite, Next.js, Nuxt, Qwik, SvelteKit, Leptos, Trunk.
- **Capability for framework**: your established desktop path (imrul, innologylock). Good for offline-first desktop tools; not for web-only apps.

### Vite
- **What**: Next-generation frontend build tool and dev server. Native ESM dev serving, HMR, optimized production build (Rollup/Rolldown).
- **Features**: instant server start, lightning HMR, rich features out-of-box (TS, JSX, CSS, workers, WASM), flexible plugin system (Rollup-compatible), fully typed API, first-class SSR support.
- **Use**: foundation for React/Vue/Svelte/Solid apps; powers Next.js alternative stacks (TanStack Start, SvelteKit, etc.).
- **Capability for framework**: core build layer; plugin ecosystem is huge.

### Tailwind CSS (v4)
- **What**: Utility-first CSS framework. v4.3 current.
- **Core concept**: single-purpose utility classes combined in markup; compiler scans source files and generates only used CSS.
- **Key features**: responsive variants (sm:, md:, etc.), state variants (hover:, focus:, dark:), arbitrary values ([#316ff6]), class composition via CSS vars, important modifier (!), prefix option, dark mode, theme variables, @layer components for custom CSS.
- **v4 changes**: CSS-first config (@import "tailwindcss", @theme), @tailwindcss/vite plugin, no tailwind.config.js needed.
- **Integration**: Vite plugin (@tailwindcss/vite), PostCSS, CLI, framework guides.
- **Managing duplication**: loops, multi-cursor, components, custom CSS classes.
- **Capability for framework**: your standard styling layer across all projects.

### Firebase Auth (brief)
- Auth methods: email/password, phone, Google/Apple/Facebook/GitHub/Twitter/Microsoft, anonymous, custom tokens, multi-tenancy, MFA, blocking functions.
- Integrates with Firestore security rules via request.auth.
- Capability for framework: your standard auth (used in ~10 projects).

### Cloudflare Workers
- **What**: Serverless platform on Cloudflare's global network. Deploy with single command (Wrangler CLI).
- **Key features**: fast global performance, full-stack framework support (React, Vue, Svelte, Next, Astro, React Router, and more), multi-language (JS, TS, Python, Rust, WASM), built-in observability, cron triggers, queues, workflows.
- **Bindings**: Durable Objects (stateful), D1 (SQL), KV (key-value), Queues, Hyperdrive (external DB acceleration), Workers AI (LLMs/images), Workflows (durable), Vectorize (vector DB), R2 (object storage, zero egress), Browser Run (headless browser), Cache/CDN, Images.
- **Capability for framework**: your edge compute layer (innofinance functions/worker); good for lightweight API/edge logic and global low-latency.

### Next.js (brief, verified)
- **What**: Full-stack React framework. v16.4.5 current. Routing, rendering (SSR/SSG/ISR), data fetching, caching, build tooling.
- **Routers**: App Router (/docs/app) vs Pages Router (/docs/pages). Docs versioned by package.json version.
- **Usage in your projects**: shopery, fleet-vue, ostad, projects/test, vps/*, innology-vps-panel.
- **Capability for framework**: reference for full-stack React; your projects use it for VPS panels and shop demos.

### Tiptap (v3)
- **What**: Headless, framework-agnostic rich text editor based on ProseMirror. Free core + 100+ extensions; paid Cloud features (collaboration, comments, AI, conversion, snapshots).
- **Packages**: @tiptap/react, @tiptap/starter-kit, @tiptap/extension-* (color, highlight, image, link, mention, table*, task-list, text-align, underline).
- **Features**: headless (you style), framework-agnostic (React/Vue/Svelte/Vanilla), extensible, collaboration (real-time, offline-first via Hocuspocus), AI toolkit, comments, version history, DOCX conversion.
- **Capability for framework**: rich text editing foundation; baishakhix uses it heavily (tables, mentions, tasks, images). Paid tier needed for collab/comments.

### @react-three/* (Three.js)
- **What**: React renderer for Three.js. @react-three/fiber (renderer), @react-three/drei (helpers/abstractions).
- **Capability for framework**: 3D visualization; imrul-ui-v2 uses it.

### @stripe/* (Stripe)
- **What**: Stripe payments SDK. @stripe/stripe-js + @stripe/react-stripe-js.
- **Capability for framework**: payment processing; edutech uses it.

### Appwrite
- **What**: Open-source backend platform (Firebase alternative). Auth, Databases, Storage, Functions, Messaging, Realtime, Sites hosting, Avatars.
- **Products**: Auth (OAuth, magic link, MFA), Databases (tables/collections), Storage (files), Functions (serverless), Messaging (email/SMS/push), Sites (hosting), Avatars (icons/screenshots/QR), Realtime (events).
- **APIs**: REST, GraphQL, Realtime. SDKs for many platforms. CLI, Terraform, MCP server, agent skills.
- **Deploy**: Appwrite Cloud or self-hosted (Docker). Migration tools from Firebase/Supabase/Nhost/Vercel.
- **Capability for framework**: alternative to Firebase/Supabase; your usage is fleet-vue-v2.0 (appwrite + node-appwrite).

### @simplewebauthn/browser
- **What**: WebAuthn/passkey client. Registration and authentication flows.
- **Capability for framework**: passwordless auth; used in vps/web.

### @stripe/* (verified)
- **What**: Stripe payments. @stripe/stripe-js (load Stripe.js), @stripe/react-stripe-js (React components: Elements, PaymentElement, etc.).
- **Capability for framework**: payment processing; edutech uses it.

### LangChain (JS)
- **What**: Framework for building LLM-powered apps/agents. `createAgent` (model + tools + prompt + middleware). Supports OpenAI, Anthropic, Google, OpenRouter, Fireworks, Baseten, Ollama, Azure, AWS Bedrock.
- **Ecosystem**: LangGraph (low-level orchestration, durable execution, human-in-the-loop, persistence), LangSmith (tracing/eval), Deep Agents (batteries-included harness).
- **Core**: tools (zod schema), agents, model interface, middleware, RAG, vector stores, chains.
- **Packages**: langchain (core), @langchain/community, @langchain/openai, @langchain/google, @langchain/anthropic, etc.
- **Capability for framework**: AI feature layer; your my-vps and vps/web use @langchain/community + @langchain/openai for AI engine. Google Gemini is your reliable provider.

### @google/generative-ai (Gemini)
- **What**: Google Gemini API client. Text, vision, function calling, streaming, embeddings.
- **Capability for framework**: your primary AI provider (only one that works reliably per AGENTS.md).

### TensorFlow.js
- **What**: ML library for JavaScript. Develop/train/run ML models in browser or Node.js.
- **Capabilities**: run existing models (convert Python TF models), retrain via transfer learning, develop ML in JS with flexible APIs, pre-trained models available.
- **Used in**: vps/web (@tensorflow/tfjs) for client-side ML.
- **Capability for framework**: browser/edge ML inference; heavy bundle size consideration.

### @xenova/transformers
- **What**: Run Hugging Face Transformers in JS (browser/Node). ONNX-based, no Python needed.
- **Capability for framework**: local LLM/embedding inference; vps/web uses it.

### @xterm/* (xterm.js)
- **What**: Terminal emulator for the web. @xterm/xterm + addons (fit, web-links).
- **Capability for framework**: in-browser terminal UI; my-vps and vps/web use it.

### @zxing/* (ZXing)
- **What**: Barcode/QR code scanner (browser + library). @zxing/browser + @zxing/library.
- **Capability for framework**: camera-based barcode scanning; baishakhix uses it.

### LiveKit
- **What**: Platform for voice, video, and physical AI agents. WebRTC-based.
- **SDKs**: JS client, React components, server SDKs (Node/Go/Python/Ruby), agent frameworks (Python/Node).
- **Features**: realtime audio/video, agent framework (voice AI, multimodal), deploy/observe/scale.
- **Packages**: @livekit/components-react, @livekit/components-styles, livekit-client, livekit-server-sdk.
- **Capability for framework**: real-time voice/video + AI agents; Astha_feed_industries uses it.

### @hello-pangea/dnd (verified)
- **What**: Fork of react-beautiful-dnd. Drag-and-drop lists. Accessible, beautiful.
- **Capability for framework**: kanban/sortable lists; Astha uses it.

### @dnd-kit/* (verified)
- **What**: Lightweight, accessible DnD toolkit. Sensors, collision detection, sortable.
- **Capability for framework**: flexible DnD; edutech uses it.

### @fingerprintjs/fingerprintjs (verified)
- **What**: Browser fingerprinting (visitor ID). Used for device identification.
- **Capability for framework**: device tracking/anti-fraud; Astha uses it.

## Framework deep dives (batch 2: Dioxus, Buffalo, TanStack Start)

### Dioxus (v0.7)
- **What**: Rust framework for building fullstack web, desktop, and mobile apps from one codebase. 24.5k GitHub stars.
- **Cross-platform**: web (WASM), desktop (Windows/macOS/Linux), mobile (iOS/Android). One Rust source ? platform-native binaries.
- **Syntax**: RSX macro (JSX-like), React-style hooks (use_signal, use_effect, use_memo), components via #[component] attribute.
- **Reactivity**: Signal-based (not virtual DOM). Fine-grained reactivity inspired by SolidJS. Hot reload via Subsecond Rust hot-reloader.
- **Fullstack**: server functions, SSR, HTML streaming, SSG, websockets, SSE, middleware, auth, forms/multipart. Backend uses Axum.
- **Routing**: file-based or declarative, nested routes, layouts, search params.
- **Styling**: HTML/CSS (not custom DSL). Tailwind supported via official guide. WGPU-based renderer (Blitz crate).
- **Assets**: manganis (linker-based asset bundling), WASM bundle-splitting.
- **Ecosystem**: SDK (1st-party system integrations), plugins, custom renderers.
- **Escapability**: drop into web-sys (web), jni (Android), objc2 (iOS) via cfg gates.
- **Funding**: YC + sponsors; open source forever. Dioxus Deploy planned.
- **Capability for framework**: strongest Rust cross-platform option. HTML/CSS styling + Rust native = better Flutter. Your Rust projects (innology-code, NetworkFix) could reuse this for desktop/mobile frontends instead of Tauri+React.

### Buffalo (Go)
- **What**: Go web framework for rapid full-stack web apps. `buffalo` CLI toolbox + framework.
- **Stack**: Gorilla Mux routing, Plush templating (Rails-like), Pop/Soda ORM (MySQL/Postgres/SQLite/CockroachDB), webpack asset pipeline, hot code reload.
- **CLI**: new, generate, dev, build, destroy, pop (db), routes, test, task (grift), plugins.
- **Request handling**: Actions (controllers), Resources (CRUD generators), middleware, context, request binding, sessions, cookies, flash messages.
- **Frontend**: Plush templates OR html/template OR custom engine. Layouts, partials, helpers, forms, asset pipeline.
- **Auth**: local (bcrypt) + third-party (goth = OAuth provider).
- **Background workers**: Grift tasks (Rake-like).
- **Deploy**: single binary via `buffalo build` (pack), cross-compile, systemd, proxy, cloud providers.
- **Status**: last release ~2023. Maintenance mode. Go 1.16+ required.
- **Capability for framework**: Rails-for-Go. Great for rapid CRUD + DB-backed internal tools. But Plush is dated; consider replacing with html/template + hx/templ + Go templates. Not in your stack currently.

### TanStack Start (v1 stable)
- **What**: Full-stack React framework powered by TanStack Router. Adds SSR, streaming, server functions, server routes, middleware to the Router.
- **Build**: Vite OR Rsbuild. Universal deployment (Netlify, Vercel, Cloudflare, Render, Railway, Lovable).
- **Core**: 100% TanStack Router for routing. Start adds server + build layers around the route tree.
- **Server functions**: createServerFn with validator + handler. Type-safe RPC from client to server.
- **Server routes**: for external callers (API endpoints).
- **Middleware**: composable for auth, context, logging, policy.
- **SSR modes**: full SSR, data-only SSR, client-only.
- **Streaming**: progressive HTML streaming.
- **Type safety**: end-to-end across client/server.
- **Comparison**: vs Next.js — Start is Router-first, more flexible deployment. vs Remix — similar server-function model but Router-driven.
- **Partners**: Vercel, Netlify, Cloudflare, Render, Lovable, CodeRabbit, AG Grid, WorkOS, OpenRouter, Clerk, Sentry, Unkey, Prisma.
- **Capability for framework**: strong Next.js alternative. Your Open_Erp already uses TanStack Router + Query — Start would be the natural full-stack evolution. Deploy targets match your hosting pattern (Netlify, Cloudflare, Vercel).

## Second-pass status
- Verified deep dives completed for ~55 packages + 3 frameworks (Dioxus, Buffalo, TanStack Start) across forms, state, UI primitives, databases, desktop, build, styling, auth, edge, Next.js, Tiptap, 3D, payments, Appwrite, WebAuthn, LangChain, Gemini, TensorFlow, xterm, ZXing, LiveKit, DnD, fingerprinting.
- Remaining ~275 packages are mostly standard tooling (Babel, ESLint, TypeScript, Gulp, Webpack, type defs, gulp plugins) or niche utilities whose capabilities are documented in the first-pass catalog. Flag any specific one for a deep dive.

## Third pass (batch 3: realtime, UI kit, animation, charts, maps)

### socket.io (v4.x, docs updated Oct 2026)
- **What**: Event-based, bidirectional client/server communication. Not a WebSocket implementation: Socket.IO clients cannot connect to plain WebSocket servers.
- **Transports**: HTTP long-polling, WebSocket, WebTransport; auto-selects per browser/network.
- **Features**: automatic reconnection with exponential backoff, heartbeat, packet buffering while disconnected, acknowledgements with optional timeout, rooms and broadcasting, namespaces (multiplexing), multi-node scaling via adapters.
- **Cost**: ~10.4 kB gzipped client bundle. Protocol overhead is a small prefix on each message (e.g. `42[...]`); custom parser possible.
- **Caveat**: not suited to mobile background services (keeps TCP open; use FCM there).
- **Capability for framework**: used in my-vps/frontend and my-vps/backend. Good default for realtime dashboards and terminals; plain `ws` is lighter if you only need raw sockets and can accept losing reconnection/ack logic.

### antd (Ant Design) (v5/v6 line)
- **What**: Enterprise React UI library from Ant Group. CSS-in-JS theming with design tokens and algorithms (dark, compact, etc.).
- **Components**: large set including DatePicker, Tour, FloatButton, Splitter, Masonry, plus data tables and forms (Form integration).
- **Theming**: token-based theme editor; style algorithms; antd-style for custom styling.
- **Capability for framework**: edutech uses it. Full-featured alternative to the Radix + shadcn stack (fewer styling decisions, heavier bundle, opinionated look). Mixing it with Tailwind needs care due to competing style systems.

### motion (formerly Framer Motion)
- **What**: Animation library for React, JS, and Vue. Renamed from framer-motion; the `motion` package is the current name.
- **Features**: layout animations (`layoutId`, shared-element transitions), gestures (drag, hover, tap), scroll-linked values using ScrollTimeline where supported, springs, `AnimatePresence` for exit animations, 3D (Three.js) integration.
- **Bundle**: starts at ~3.2 kB with the mini `animate` function; `LazyMotion` loads features on demand.
- **Capability for framework**: used in edutech, innofinance, my-vps, vps/web. Package name in those projects is `framer-motion`; new work should target `motion`.

### recharts (v3.10.x)
- **What**: Composable charting library built from React components on top of SVG and D3 submodules.
- **Components**: LineChart, BarChart, AreaChart, PieChart, etc., with CartesianGrid, XAxis, YAxis, Legend, and custom components.
- **Notes**: docs moved to recharts.github.io. v3 has a migration guide. `ResponsiveContainer`-style sizing is supported via `responsive` and aspect ratio props.
- **Capability for framework**: used by most projects in D:\projects. shadcn/ui charts are built on it, so it is the de facto default for dashboards here.

### leaflet (v2.0 beta, Nov 2026)
- **What**: Open-source interactive map library. ~40 kB gzipped JS plus ~2.4 kB CSS, zero dependencies, BSD-2 license.
- **Features**: tile layers and WMS, markers, popups, tooltips, vector shapes, GeoJSON, image/video/SVG overlays, custom projections, keyboard and touch support, TypeScript types included.
- **Notes**: v2.0 beta is out; migration guide in repo (MIGRATION.md). 1.x API remains the stable reference.
- **Capability for framework**: used in innofinance and Astha (react-leaflet wrapper). Map layer for any location-based feature; pair with OpenStreetMap tiles or a commercial tile provider for production traffic.

## Third-pass status
- Added deep dives for socket.io, antd, motion, recharts, leaflet.
- Remaining high-value candidates for a fourth pass: express, pg / drizzle-orm, @aws-sdk/client-s3, @univerjs/*, dayjs / date-fns, @tanstack/react-table, vitest / @playwright/test, vite-plugin-pwa, @metered-ca/realtime, peerjs.
