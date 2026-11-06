# Project Study: Backend, infrastructure, and deployment

Source: direct reads of manifests and deploy configs under D:\projects, Nov 2026. Read-only; no project files modified. Anything not read is marked "not inspected". Secret values in .env files were not printed.

---

## 1. Self-hosted VPS platform: vps/web ("Innology AI VPS")

This is the multi-tenant hosting panel listed as Active Project #1 in PROJECT_INDEX.md. It is the largest backend in the workspace.

### Containers (docker-compose.yml)
- **backend**: built from ./backend. Ports 8000 (API), 21 (FTP mapped to 2121), 30010-30019 (passive FTP range). Mounts /var/run/docker.sock, so the backend controls the host's Docker daemon. This is the highest-risk privilege in the stack.
- **Other compose files**: docker-compose.dev.yml and docker-compose.vps-4g.yml (a lower-memory variant, by name only; not inspected).
- Backend source is bind-mounted file by file into /app (about 50 individual files). Adding a module requires editing compose. Not inspected whether a catch-all volume exists elsewhere.
- Extra Dockerfiles: docker/code-server/Dockerfile (VS Code in browser), docker/php-fpm/Dockerfile (PHP hosting), backend/ai-engine/Dockerfile (separate AI service).

### Reverse proxy (Caddyfile)
- Caddy with the admin API on 0.0.0.0:2019.
- Dynamic vhost import: `import /etc/caddy/Caddyfile.d/*.conf`. The backend writes per-project vhosts at runtime (tunnel_service, routes_appstore, email_setup_service). This makes Caddy configuration state-dependent.
- Routing rules:
  - vps.{DOMAIN} and apex {DOMAIN}: /api/*, /health, /system/*, /docs, /openapi.json, /redoc go to backend:8000 with flush_interval -1 (no buffering, important for SSE/streaming).
  - /vscode* goes to code-server:8080 with a permissive CSP (unsafe-inline, unsafe-eval). Needed by VS Code, but a weakened policy on that origin.
  - Everything else goes to frontend:3000 (Next.js panel).
  - Apex domain serves static user project files from /workspace/projects/user_1/{DOMAIN} if the file exists, otherwise falls back to the panel. Cache-Control no-store for user sites.
- Strict-Transport-Security and nosniff headers on panel routes.
- Note: the Caddyfile contains mojibake (`â€”` in comments, rendered as dashes) from an encoding round-trip. Cosmetic.

### Frontend (vps/web/package.json, Dockerfile)
- Next.js 15.5 (dependency range `^15.5.18`), React 19, Tailwind CSS 4 (`@tailwindcss/postcss`), TypeScript 5.3.
- Client libs: @langchain/community 0.3, @langchain/openai 0.3, langchain 0.3, @simplewebauthn/browser 10 (passkeys), @tensorflow/tfjs 4.22, @xenova/transformers 2.17 (in-browser ML), onnxruntime-node 1.20, @xterm/xterm 5.5 with fit addon (terminal), framer-motion 11, next-themes, lucide-react, tailwind-merge.
- `overrides` pins transitive deps (protobufjs, axios, uuid, ws, qs, langsmith) to patch versions. Good practice for supply-chain fixes.
- Dockerfile is multi-stage on node:20-alpine. Firebase and Turnstile public keys are written to .env.local at build time from build args (ARG), so they are baked into the client bundle. Public keys only, which is acceptable for Firebase web config; note it is not a secret store.
- Runtime uses Next standalone output (`.next/standalone`) run as non-root user nextjs (uid 1001). HEALTHCHECK on port 3000.
- `npm install ... 2>/dev/null || npm install --ignore-optional` swallows the first install error. Can hide dependency failures.

### Backend (vps/web/backend)
- requirements.txt covers: FastAPI + Uvicorn, SQLAlchemy + psycopg2, Alembic (migrations), Redis, python-jose and PyJWT (JWT), passlib + bcrypt, pyotp (TOTP MFA), cryptography, docker SDK, psutil, boto3 (S3), google-genai and google-generativeai (Gemini), langchain 0.1+ family (community, core, google-genai, ollama), numpy (RAG), ddgs (web search), pyftpdlib (FTP server), httpx, qrcode, Pillow, python-multipart, pydantic[email], pytest and pytest-asyncio.
- Ranges are minimums (`>=`) with no lock file. Builds are not reproducible.
- Route modules (from the compose mounts, about 30): agent, AI hybrid, AI models, appstore, container filesystem, code-server, dev server, DNS, files, market, MFA, notifications, RAG, security, self-healing, SSE, storage, terminal, tunnel, tunnel settings, updates, GDPR, workflow, email features.
- Service modules: AI builder and build doctor, AI router, AI providers (Google, OpenAI, Hugging Face, Together, Cohere, Replicate, local, web search), app store, backup, billing, CMS, codebase indexer, composer, domain, email processor and service and setup, file manager, FTP service and server, git analyzer, GitHub deploy and service, Gmail fetcher.
- Provider abstraction: ai_providers/base.py defines a common interface; each vendor is a module. This is the right pattern for multi-model routing. The router (ai_router.py) selects provider per request.
- Not inspected: the code inside any route or service file.

### AI engine (vps/web/backend/ai-engine)
- Separate FastAPI service with llama-cpp-python for local GGUF inference, psutil for resource checks, httpx. Runs as its own container.
- This matches the innology-code ADR-0013 direction (embedded llama.cpp).

### Relevance to a custom framework
- Clear service-per-concern split; a good reference for a backend module registry.
- Risks to avoid in any new framework: Docker socket mounted into an app container (root-equivalent on host), per-file bind mounts that must be edited by hand, and dynamic reverse-proxy config written by the app without validation.

---

## 2. Innology VPS panel: vps/innology-vps-panel

- Frontend: Next.js 16.2.4, React 19.2.4, Tailwind 4, ESLint 9, TypeScript 5, lucide-react.
- Backend: requirements are FastAPI, Uvicorn, SQLAlchemy, psycopg2-binary, python-jose, passlib[bcrypt], python-multipart, google-generativeai, requests, python-dotenv.
- docker-compose.yml: backend (FastAPI), postgres:15 (database vpsdb), redis:7. Compose contains hard-coded dev credentials (vpsuser/vpspass) and exposes Postgres on 5432 and Redis on 6379 to the host. Acceptable for local development only; must not ship as-is.
- infra/caddy/Caddyfile: localhost to frontend:3000, api.localhost to backend:8000. Local development routing only. Email placeholder.
- Relationship to vps/web: a smaller sibling with the same panel idea but Postgres and Redis already in compose. Likely an earlier or parallel iteration.

---

## 3. my-vps: code/my-vps (Plug-and-play VPS platform, repohive DB)

### Backend (Node/Express)
- Node app: Express 4, socket.io 4.8, node-pty (real PTY terminals), xterm addons, pg (Postgres), jsonwebtoken, bcryptjs, helmet, cors, cookie-parser, express-rate-limit, express-async-handler, multer, unzipper and adm-zip (archive handling), fs-extra, node-cron (scheduled jobs), nodemailer (email), qrcode and otplib (MFA), axios, uuid, openai and @google/generative-ai (AI providers), dotenv.
- Dev: nodemon, Jest 30 with supertest. Test glob `**/tests/**/*.test.js`, node environment, coverage on.
- Scripts: start (node index.js), dev (nodemon), test (jest --coverage).
- Note: Express 4 is in maintenance; Express 5 is current. Migration effort is moderate.
- Dev database: docker-compose.dev.yml runs postgres:15-alpine (db `repohive`, user postgres/postgres, dev-only) and MailHog (SMTP 1025, UI 8025) for local email testing.

### Frontend (Vite React)
- Vite React SWC template (name vite_react_shadcn_ts, lovable-style scaffolding).
- Radix primitives (full set), TanStack Query 5, axios, class-variance-authority, clsx, cmdk, date-fns 3, embla-carousel, framer-motion 12, input-otp, lucide-react, next-themes, react-hook-form with @hookform/resolvers, react-router-dom, react-toastify, recharts, socket.io-client, sonner, tailwind-merge, tailwindcss-animate, vaul (drawer), zod.
- Terminal in browser: xterm, xterm-addon-fit, xterm-addon-web-links.
- Test: vitest, @testing-library/react, jest-dom, jsdom.

### Relevance
- Full-stack reference for: Node PTY terminals over socket.io, MFA (otplib + QR), scheduled jobs (node-cron), and an AI provider pair (OpenAI + Gemini).

---

## 4. Innofinance: code/innofinance (finance PWA with Firebase and Netlify)

- Hosting: netlify.toml builds with `npm run build`, publishes `dist`. Netlify Functions in `netlify/functions` bundled with esbuild. SPA fallback `/* -> /index.html`. Security headers (X-Frame-Options DENY, nosniff, Referrer-Policy). Cache rules: assets immutable one year; sw.js, index.html, manifest no-cache.
- Firebase: firebase.json declares Firestore rules and indexes, and a Cloud Functions codebase `default` on nodejs18 (source `functions`). Functions package: firebase-admin 12, firebase-functions 5, Node engine 18.
- Cloudflare Worker: functions/worker/wrangler.toml, name `budget-buddy-digest`, entry send-reminders.js, cron `0 2 * * *` (02:00 UTC = 08:00 Asia/Dhaka). Secrets (service account JSON, project id) are set via `wrangler secret put`, not stored in the file. Good practice.
- Companion packages: scripts/fcm-sender (firebase-admin), brag-output-... (no deps), functions.
- Front-end manifest (from earlier study): Vite, React, Radix, TanStack Query, Dexie (IndexedDB), i18next, leaflet, recharts, xlsx, jspdf, pdfjs-dist, nodemailer (dev-side), otpauth, qrcode.react, vite-plugin-pwa, vitest, testing-library, arcovia.
- Architecture note: three backends at once (Firebase Firestore + Cloud Functions, Netlify Functions, and a Cloudflare Worker). Each has its own deploy pipeline and secrets model. This is the kind of fragmentation a custom framework should unify.

---

## 5. Other backend services

### OPMS and Open_Erp, portfolio (Netlify)
- netlify.toml present in each. Front-ends were covered in the frontend study. OPMS uses Firebase (firebase.json); Open_Erp uses Supabase (no firebase.json; Supabase CLI in devDependencies).

### baishakhix: code/baishakhix
- Dockerfile and docker-compose.yml: single `frontend` service on port 5173 with bind mount `.:/app`, anonymous volume for /app/node_modules (correct pattern to avoid host/container node_modules clashes), env_file .env.
- Firebase: firestore rules and indexes; Cloud Functions in `functions/` (firebase-admin, firebase-functions, bcryptjs for custom password hashing, firebase-functions-test for tests).
- Netlify config present. Vite + React + Redux Toolkit + TanStack Query + Tiptap rich text + ZXing barcode + jsPDF.
- Deployment is dual: Docker (local) and Netlify/Firebase (production). Not inspected which is primary.

### edutech: code/edutech/redux47/redux-backend
- Express 4 app with `"type": "module"`, cors, dotenv, express, firebase-admin 13, stripe 15.
- vercel.json routes every request to "/" with `@vercel/node` builder for ./app.js. The rewrite `/(.*) -> /` is unusual: it sends every path to the root, so any sub-path route in Express would never match on Vercel. Likely a bug unless the app is a single-route webhook. Not verified by running it.
- redux-frontend: Netlify config, Vite + React, Redux Toolkit, Stripe React, Firebase, Framer Motion, Swiper, Echarts.

### Rust backend crates
- See PROJECT_RUST_TESTING_STUDY.md for innology-code (axum planned, currently scaffold), innologylock service crate, NetworkFix.

### Python services in other projects
- **imrul** (code/imrul/requirements.txt): FastAPI 0.104+, Uvicorn, Pydantic 2.5+, httpx, psutil, python-dotenv, pyyaml, ollama client; voice stack sounddevice, soundfile, faster-whisper (speech-to-text), edge-tts (text-to-speech), numpy; memory via chromadb (vector DB). A local AI assistant with voice and RAG memory.
- **DriveBackup**: rclone-driven backup desktop app; see PROJECT_RUST_TESTING_STUDY.md section 5.

---

## 6. Patterns across backend projects (what a framework must unify)

| Concern | Patterns found |
|---|---|
| Runtime | Node/Express (my-vps, baishakhix functions, edutech, innofinance functions), FastAPI (vps/web, innology-vps-panel, imrul), Go (none active; Buffalo studied only), Rust (innology-code, NetworkFix, innologylock), Cloudflare Workers (innofinance) |
| Database | Postgres (vps/web, innology-vps-panel, my-vps), Firestore (innofinance, baishakhix, OPMS, Astha), Supabase (Open_Erp), Redis (vps/web, innology-vps-panel), chromadb (imrul) |
| Auth | JWT (jsonwebtoken, PyJWT, python-jose), bcrypt (bcryptjs, passlib, bcrypt), TOTP MFA (otplib, pyotp), WebAuthn passkeys (@simplewebauthn), Firebase Auth |
| Realtime | socket.io (my-vps, vps/web via xterm), SSE (vps/web routes_sse), LiveKit (Astha) |
| Scheduled jobs | node-cron (my-vps), Cloudflare cron trigger (innofinance worker), Firebase scheduled functions (not inspected) |
| Email | nodemailer (my-vps, innofinance), Brevo and Gmail fetcher (vps/web), EmailJS (client-side, portfolio/baishakhix) |
| Payments | Stripe (edutech redux-backend, redux-frontend) |
| AI providers | Gemini (google-genai, @google/generative-ai, google-generativeai) in most; OpenAI in my-vps and vps/web; LangChain orchestration in vps/web; local llama.cpp in vps/web ai-engine and innology-code; Ollama in vps/web and imrul |
| Deploy | Docker compose (vps/web, innology-vps-panel, my-vps dev, baishakhix), Netlify (most frontends), Vercel (edutech backend), Firebase Hosting and Functions (innofinance, OPMS, baishakhix, Astha), Cloudflare Workers (innofinance) |
| Reverse proxy | Caddy (vps/web, innology-vps-panel) |

Key observation: the same concerns (auth, email, payments, AI, scheduled jobs, deploy) are re-implemented per project with different libraries. A custom framework that owns these as modules would remove most of the duplication.

---

## 7. Security findings to avoid repeating

1. vps/web backend mounts /var/run/docker.sock: host root equivalent.
2. vps/web Caddy exposes admin API on 0.0.0.0:2019 (should be bound to localhost or removed in production).
3. vps/web Dockerfile writes secrets-adjacent public config into .env.local at build; acceptable for public keys only.
4. innology-vps-panel compose has hard-coded credentials and host-exposed Postgres and Redis.
5. my-vps dev compose uses postgres/postgres; dev only, but should not leak to production.
6. edutech vercel.json rewrites all paths to "/", which likely breaks non-root Express routes.
7. Python requirements with minimum-only pins (vps/web, innology-vps-panel, imrul, DriveBackup) are not reproducible.
8. Install commands that swallow errors (`2>/dev/null ||` in vps/web Dockerfile).

---

## 8. Not inspected (honest gaps)
- Source code inside route and service modules of vps/web and my-vps. Only dependency lists and configs were read.
- Netlify functions in innofinance, OPMS, Open_Erp, and portfolio.
- Firebase rules files (firestore.rules) in any project.
- The docker-compose.vps-4g.yml and docker-compose.dev.yml files in vps/web.
- Whether the Astha and OPMS Firebase functions exist.
