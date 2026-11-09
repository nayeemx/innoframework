# UI a11y + visual snapshot spike (P4)

Purpose: close the **TR-4.1 (axe-core, web)** and **TR-4.3 (visual regression, no-overflow proof at 3 breakpoints)** evidence gaps that Rust/SSR tests cannot prove. This is a **Node toolchain spike** and is **not run by Rust CI** (`innoframework-ci`) by default.

## Scope
- axe-core (WCAG 2.x A/AA tags) smoke; fails on critical/serious violations.
- Playwright screenshots for 3 breakpoints: **360x740, 768x1024, 1280x800** (projects).
- Run against a built/served fixture (`INNO_UI_BASE_URL`, default `http://127.0.0.1:4173`).

## Run against an `inno init` web fixture (recommended)
```bash
# 1) build a fixture app (from innoframework workspace)
cargo run -p innoframework-cli -- init --name ui-fixture -y --path <temp-dir>   # or: inno init --name ui-fixture -y
cd <temp-dir>/ui-fixture
inno build web   # or: dx build --platform web --release
inno dev --port 4173   # serves with CSP; keep running

# 2) run the spike (separate shell)
cd innoframework/tools/ui-a11y-snapshots
corepack enable && pnpm install
pnpm exec playwright install chromium
INNO_UI_BASE_URL=http://127.0.0.1:4173 pnpm test
```
Update baselines intentionally: `pnpm test:update` (review diffs). Pin Node/browser/OS in CI before treating results as gate evidence.

## Gaps still open
- **AccessKit/native** a11y checks (desktop/mobile) - separate decision/tooling.
- Axe passing on a smoke page != all components/pages proven; expand fixtures per shipped UI.
- Visual diffs require a stable fixture and font/render environment.
- Do not mark TR-4.1/4.3 "passing" in traceability until these run green in a pinned environment against release fixtures.
