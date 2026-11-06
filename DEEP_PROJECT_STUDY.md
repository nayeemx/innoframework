# Deep Project Study — D:\projects

## Overview
~30+ projects. Predominantly Vite + React + Tailwind frontends, Firebase-backed, Netlify-hosted.

## Backend / BaaS usage
- **Firebase (heavy)**: innofinance, Astha_feed_industries, baishakhix, edutech, e-gp tracker, innology-treding, OPMS. Firestore + Auth + Cloud Functions + hosting rules.
- **Supabase**: Open_Erp only (`@supabase/supabase-js`, supabase CLI). RLS + migrations.
- **Appwrite**: fleet-vue-v2.0 (`appwrite`, `node-appwrite`).
- **Cloudflare Workers**: innofinance (`functions/worker/wrangler.toml`).
- **Vercel**: edutech/redux-backend only.
- **Self-hosted (Docker/VPS)**: baishakhix, my-vps, vps/web, innology-vps-panel. Express + Postgres + Socket.io + JWT.

## Frontend frameworks
- **Vite + React** (dominant): most projects.
- **Next.js**: shopery, fleet-vue-v2.0, ostad/blog, projects/test, vps/web, innology-vps-panel.
- **Tauri (Rust desktop)**: imrul, innologylock.
- **Svelte**: imrul (v1, svelte+vite).

## Rust
- innology-code (multi-crate workspace: ai, app, build, containers, core, engine, env, files, git, proc, secrets, share, tunnel)
- innologylock (Tauri + Rust service + shared crate)
- NetworkFix (networkfix, networkfix-core, networkfix-gui)
- imrul (Tauri)

## Common UI / libs
- Radix UI (shadcn) + Tailwind: innofinance, OPMS, portfolio, my-vps, e-gp tracker, fleet-vue.
- antd: edutech.
- framer-motion: edutech, innofinance, my-vps, vps/web.
- recharts: most.
- leaflet / react-leaflet: innofinance, Astha.
- tiptap: baishakhix.
- univerjs (spreadsheets): Astha.

## State / data
- Redux Toolkit + react-redux: baishakhix, edutech, e-gp tracker, innology-treding, Astha.
- TanStack Query: baishakhix, e-gp tracker, innofinance, Open_Erp, portfolio, my-vps.
- TanStack Router: Open_Erp (only one using it).
- Dexie (IndexedDB): innofinance, my-vps.
- localforage: Astha.

## Forms / validation
- react-hook-form + zod: innofinance, e-gp tracker, portfolio, my-vps, Open_Erp.

## i18n
- i18next + react-i18next: innofinance, OPMS.

## AI
- @google/generative-ai: my-vps/backend.
- openai + langchain: my-vps/backend, vps/web.
- @xenova/transformers, tensorflow: vps/web.

## Testing
- vitest + testing-library: innofinance, OPMS, my-vps, Open_Erp.
- playwright: OPMS, extension/innocapture, Open_Erp.
- arcovia (architecture tests): innofinance, OPMS.
- jest + supertest: my-vps/backend.

## Hosting pattern
- Netlify auto-deploy from main is the dominant flow.
- VPS/Docker for baishakhix, my-vps, vps/*.
- Firebase hosting for some.

## Notes for framework design
- User heavily relies on: Netlify, Firebase (auth/firestore/functions), Cloudflare Workers, Vite, React, Tailwind, Radix/shadcn.
- Cross-platform desktop via Tauri (Rust) is an established pattern.
- Supabase is used in exactly one project (Open_Erp) — mostly Firebase elsewhere.
- AI providers: Google Gemini works; OpenRouter unreliable (per AGENTS.md).
