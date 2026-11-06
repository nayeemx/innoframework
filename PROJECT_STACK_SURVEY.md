# Project Stack Survey (D:\projects)

## Hosting / backend usage patterns (from config files found)

### Netlify (heavy use)
innofinance, Astha_feed_industries, baishakhix, edutech (redux-frontend), innology-treding, Open_Erp, OPMS, portfolio, treding-frontend.
- Netlify auto-deploy from main is the primary flow for innofinance.
- Netlify Functions used (innofinance functions/worker, firebase service account).

### Firebase
innofinance, Astha_feed_industries, baishakhix, e-gp tracker (tender-bangla), OPMS.
- .firebaserc + firebase.json present; Firestore, Auth, hosting, remote config.

### Supabase
Not found in config scan at depth 4. May be used in code only; verify per project.

### Cloudflare
innofinance uses Cloudflare Workers (functions/worker/wrangler.toml).
- Need to scan for wrangler in other projects.

### Vercel
edutech/redux-backend (vercel.json).
- Minor use.

### Docker / VPS (self-hosted)
baishakhix (Dockerfile + docker-compose), vps/web (Dockerfile, docker-compose, next.config.ts), innology-vps-panel.
- VPS panel: Next.js frontend + Docker backend.

### Tauri (Rust desktop)
imrul (src-tauri/Cargo.toml, tauri.conf.json), innologylock (src-tauri).
- Cross-platform desktop apps in Rust.

### Cargo / Rust crates
innology-code (multi-crate workspace), innologylock, NetworkFix, imrul.
- Rust used for backend, desktop, and CLI tooling.

### Next.js
figma-to-code/shopery, fleet-vue-v2.0, ostad/blog-project, vps/web, vps/innology-vps-panel, projects/test.
- Mostly learning/demo and VPS panels.

### Vite (most common build tool)
Used across almost all frontend projects.

### Flutter / Dart
baishakhix has docker-compose (possibly Flutter backend). Check pubspec.yaml separately.

## Notes
- Most projects are Vite + React + Netlify + Firebase stacks.
- Rust/Tauri used for desktop (imrul, innologylock) and backend services.
- Go not found in scan (Buffalo not currently used in any project).
- Dioxus not currently used (candidate for new cross-platform apps).
