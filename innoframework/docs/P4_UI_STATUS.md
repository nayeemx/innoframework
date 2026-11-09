# P4 UI kit v1 (in progress)

Status: **in progress**. Core scaffold + TR-4.1 catalog semantics, TR-4.2 token structure, TR-4.3 SSR layout/breakpoint contract, TR-4.4 motion/reduced-motion tokens, and TR-4.5 contrast/touch-target rule checks are implemented with automated tests (`innoframework-ui`, Dioxus 0.7 + `dioxus-ssr`). **Not gate-complete:** browser axe-core/AccessKit CI and true visual regression snapshots (3 breakpoints, overflow) still require web/native tooling pipelines.

## Delivered
- `innoframework-ui` crate: token system, Web + Windows themes (CSS vars), 3 layouts (`AppShell`, `PageLayout`, `FormLayout`), component set including TR-4.1 catalog primitives: `Button, Input, TextArea, Select, Checkbox, RadioGroup, Switch, Dialog, Popover, Tooltip, Tabs, Table, Form, Toast, Card` plus `Badge, Heading, Text, Link, Divider, Stack, Container, Alert`.
- Motion tokens + `@media (prefers-reduced-motion: reduce)` override (TR-4.4).
- TR-4.5 rule checks: `MIN_CONTRAST_NORMAL=4.5`, `MIN_TOUCH_TARGET_PX=44`, Web/Windows text contrast checks (`src/a11y.rs`).
- `inno init` web template wiring (CSP-safe): generates `assets/inno-ui.css` (theme vars + responsive/baseline styles, breakpoints 360/768/1280) and loads it as an external stylesheet via Dioxus `asset!`/`document::Stylesheet` (no inline style attribute requirement). `inno build web` CSP header unchanged and compatible with external self-hosted styles.
- Axe-core + Playwright snapshot **spike** (not run by Rust CI): `tools/ui-a11y-snapshots/` (3 breakpoint projects, axe WCAG 2.x A/AA critical/serious gate, screenshot baselines). Requires Node/pinned toolchain and a served fixture.

## Evidence (automated)
- `cargo test -p innoframework-ui`: **25 tests passing** (6 unit in `a11y/tokens`, 8 `components`, 8 `a11y_layout`, 3 `tokens`).
- `cargo clippy -p innoframework-ui --all-targets -D warnings`: clean.
- CI runs `cargo test --workspace` / clippy/fmt/deny/audit via `innoframework-ci`.

## TR mapping
- **TR-4.1**: semantics/ARIA/landmarks/labels for shipped catalog components covered by SSR tests. **Open:** axe-core CI (web, Node toolchain pinned) and AccessKit checks (native) — evaluate as P0/P4 spike; do not claim green.
- **TR-4.2**: token key/structure present for Web+Windows; platform values differ, contract shared. **Open:** full per-platform pack matrix; enforce “components unchanged” beyond structure.
- **TR-4.3**: 3 layout templates render in SSR at breakpoint markers 360/768/1280. **Open:** visual regression snapshots proving no overflow.
- **TR-4.4**: declarative motion tokens + reduced-motion CSS override. **Open:** component behavior/animation usage verification.
- **TR-4.5**: documented/checkable contrast + touch-target rules with basic token contrast checks. **Open:** lint/CI enforcement for rendered UI, focus/contrast/touch-target across components.

## Open / next
- Run/maintain axe-core + Playwright against real built fixtures in a pinned env (Node not in Rust CI); expand per shipped component/page. AccessKit/native a11y remains separate.
- Replace SSR breakpoint-marker contract with real visual regression evidence (no-overflow) once fixtures/baselines are stable.
- Complete component behavior hardening and responsive CSS coverage beyond baseline; confirm v1 shipped subset (~15 correct).
- Perf/budget conflict (ARCHITECTURE_PLAN §6 vs INNOFRAMEWORK_ARCHITECTURE §2.3) remains owner decision.
