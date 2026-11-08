# INNOFRAMEWORK FRAMEWORK — Structural Blueprint

**Codename:** Innoframework  
**Owner:** InnologyBD  
**Domain:** innologybd.com  
**Documentation host:** Netlify  
**Status:** Design Phase (v1.0 spec)

---

## 1. Executive Summary

Innoframework is a unified development framework for building secure, high-performance software across web, desktop, mobile, and cross-platform targets from a **single codebase**. It is not a wrapper around npm packages. It is a compiled framework written in Rust with a self-contained CLI that requires no Node.js installation by the end user.

The framework addresses these failures of current stacks (identified from the 20+ project studies in D:\projects\code):

| Problem in current stacks | Innoframework solution |
|---|---|
| Requires Node.js installation | Self-contained Rust binary CLI, zero Node dependency |
| Built on hackable runtimes (PHP/Node.js) | Rust core with memory safety, no eval/require/shell by default |
| Re-implements auth/email/payments/AI per project | Built-in modules: auth, email, payments, AI, scheduled jobs |
| Web-only, or PWA that feels like a website | Native UI renderer for desktop/mobile; web via WASM |
| Re-implements UI components per project | Built-in component library with platform-native themes |
| No AI layer | Native agent runtime with local model support (llama.cpp) |
| No design system | Built-in design system: principles, layouts, templates, themes |

---

## 2. Core Architecture & Security

### 2.1 Target Runtime Model

Innoframework compiles to three target runtimes from one source:

| Target | Runtime | Technology | Binary size |
|---|---|---|---|
| **Web** | WASM | Rust → wasm32-unknown-unknown + Dioxus 0.7 | ~5MB |
| **Desktop** | Native | Rust + Tauri-compatible webview OR Dioxus native renderer | ~15MB |
| **Mobile** | Native | Rust + Dioxus mobile (iOS/Android) | ~10MB |
| **Cross-platform** | Unified | Same codebase, feature-flagged per platform | Variable |

**Decision:** Dioxus 0.7 as the primary UI framework (Rust, signal-based reactivity, RSX macro, cross-platform web/desktop/mobile). This is validated by the user's existing Rust projects (innology-code, NetworkFix) and is the strongest Rust cross-platform option per the second-pass study.

### 2.2 Security Model

Security is designed first, features second — matching the innology-code ADR pattern (socket-level peer credentials, tunnel scope isolation, key isolation).

**Security layer (built-in, not optional):**

```
┌─────────────────────────────────────────────────────────┐
│  Application Layer (user code)                          │
├─────────────────────────────────────────────────────────┤
│  Auth Guard Layer — every RPC/route requires session    │
│  ├── 401 on unauthenticated                              │
│  ├── 403 on insufficient role                            │
│  └── Fail closed (default deny)                          │
├─────────────────────────────────────────────────────────┤
│  RPC / Server-Function Boundary                         │
│  ├── Typed validation (Zod equivalent in Rust)           │
│  ├── Server-only code never ships to client              │
│  ├── Non-leaky error messages                            │
│  └── Audit log per call                                  │
├─────────────────────────────────────────────────────────┤
│  CSP Preset Layer                                       │
│  ├── default-src 'self'                                  │
│  ├── script-src 'self' (no unsafe-inline/eval)           │
│  ├── object-src 'none', frame-ancestors 'none'           │
│  ├── form-action 'self', base-uri 'none'                 │
│  └── report-uri + report-to                              │
├─────────────────────────────────────────────────────────┤
│  Build-Time Secret Boundary                             │
│  ├── Build fails if SERVER_*, API_KEY*, SECRET_*         │
│    appear in client output                               │
│  ├── Server env vars never bundled                       │
│  └── Public config only (Firebase project ID, etc.)      │
├─────────────────────────────────────────────────────────┤
│  Runtime Layer (Rust)                                   │
│  ├── Memory safe (no eval, no require, no shell by       │
│    default)                                              │
│  ├── Argon2id password hashing                           │
│  ├── AES-GCM data encryption                             │
│  ├── Zeroize secrets in memory                           │
│  └── Local-first AI (llama.cpp GGUF, on-device)          │
└─────────────────────────────────────────────────────────┘
```

**Threat model (testable):**
1. RPC call without valid session → 401
2. Server-only module imported by client code → build fails
3. User-controlled output rendered into HTML → CSP blocks inline scripts; output escaped
4. Build-time secret string present in client bundle → build fails
5. Prompt injection attempt → agent returns error, does not proceed
6. Agent calls unregistered tool → rejected (allowlist-only registry)

### 2.3 Performance Characteristics

| Metric | Target | How achieved |
|---|---|---|
| Cold start (desktop/mobile) | < 500ms | Rust native binary, no runtime warmup |
| Cold start (web/WASM) | < 2s | Small WASM bundle, lazy loading |
| Frame time (p95) | < 16.6ms (60fps) | Signal-based fine-grained reactivity, no Virtual DOM diffing |
| Bundle size (web) | < 5MB gzipped | Rust WASM, tree-shaken, no Node modules |
| Memory (idle desktop) | < 50MB | Rust, no Node/electron overhead |
| Binary size | < 20MB | Rust static linking, strip + LTO |

---

## 3. Installation & User Experience

### 3.1 Zero-Node Installation

**The user never installs Node.js.** The framework ships as:

1. **CLI binary** (`inno`) — single Rust binary, ~10MB
2. **One-command install:**
   ```bash
   # macOS/Linux/Windows
   curl -L https://innologybd.com/inno/install | sh
   # or
   winget install Innoframework.InnologyCLI
   # or
   brew install innologybd/inno
   ```

3. **Setup wizard** — `inno init` walks the user through:
   ```
   Welcome to Innoframework Framework
   ================================
   
   ? What are you building?
     > Web application (React-style UI)
       Desktop application (native window)
       Mobile application (iOS/Android)
       Cross-platform (all targets)
       AI agent (automation-focused)
       API backend only
   
   ? Which database?
     > Local SQLite
       PostgreSQL
       Firebase
       Supabase
       None (static)
   
   ? Which AI provider?
     > Local llama.cpp (on-device, private)
       Google Gemini
       OpenAI via OpenRouter
       None
   
   ? Project name: my-app
   ? Project directory: ./my-app
   ```

4. **Generated project** — fully working, no manual configuration needed.

### 3.2 CLI Commands

| Command | Purpose |
|---|---|
| `inno init` | Interactive setup wizard |
| `inno add <module>` | Add a module (auth, ai, payments, email, etc.) |
| `inno generate <type> <name>` | Generate component, route, server function, AI agent |
| `inno dev` | Start dev server with hot reload |
| `inno build <target>` | Build for web/desktop/mobile/all |
| `inno test` | Run unit + integration + security tests |
| `inno deploy <target>` | Deploy to Netlify/VPS/Cloudflare |
| `inno doctor` | Diagnose environment issues |

### 3.3 Generated Project Structure

```
my-app/
├── inno.toml          # Project configuration
├── src/
│   ├── app.rs             # Application entry
│   ├── components/        # UI components
│   │   ├── button.rs
│   │   ├── form.rs
│   │   └── layout.rs
│   ├── routes/            # File-system or code-defined routes
│   │   ├── index.rs
│   │   └── dashboard.rs
│   ├── server/            # Server functions (never ships to client)
│   │   ├── auth.rs
│   │   └── api.rs
│   ├── agents/            # AI agent definitions
│   │   └── code-agent.rs
│   ├── styles/            # Theme definitions
│   │   ├── light.toml
│   │   └── dark.toml
│   └── assets/            # Images, icons, fonts
├── tests/                 # Unit + integration + security tests
├── assets/                # Static assets
└── target/                # Build output (ignored by git)
```

---

## 4. Multi-Platform Capability

### 4.1 Single Codebase Architecture

One Rust codebase compiles to all targets using feature flags:

```rust
// src/app.rs — same source for all platforms
use inno::prelude::*;

#[component]
fn App() -> Element {
    rsx! {
        Router {
            Route { to: "/", component: Home }
            Route { to: "/dashboard", component: Dashboard }
        }
    }
}
```

**Compilation matrix:**

| Feature flag | Web (WASM) | Desktop | Mobile (iOS) | Mobile (Android) |
|---|---|---|---|---|
| `web` | ✓ | — | — | — |
| `desktop` | — | ✓ | — | — |
| `mobile` | — | — | ✓ | ✓ |
| `ai` | ✓ | ✓ | ✓ | ✓ |
| `auth` | ✓ | ✓ | ✓ | ✓ |
| `payments` | ✓ | ✓ | ✓ | ✓ |
| `email` | ✓ | ✓ | ✓ | ✓ |

### 4.2 Rendering Strategy

| Platform | Renderer | Technology | Performance |
|---|---|---|---|
| **Web** | WASM + DOM | Dioxus web renderer | Native DOM performance |
| **Desktop** | Native UI | Dioxus desktop renderer (WGPU) or system webview | Native frame rate |
| **Mobile** | Native UI | Dioxus mobile (iOS/Android native views) | Native frame rate |

**Decision:** Prefer native renderer (WGPU-based, Dioxus Blitz) over webview for desktop/mobile. This provides true native performance, not PWA-in-a-webview. Webview available as fallback for legacy systems (NetworkFix pattern: native-ui feature flag for Win7).

### 4.3 Platform-Specific Features

| Feature | Web | Desktop | Mobile |
|---|---|---|---|
| File system access | Limited (browser sandbox) | Full native | Full native |
| Notifications | Web Push API | Native notifications | Push notifications |
| Camera/Media | MediaDevices API | Native camera | Native camera |
| Offline | Service Workers | Local SQLite | Local SQLite |
| Background tasks | Limited | Full native | Background tasks |
| Biometric | WebAuthn | Windows Hello / TouchID | FaceID / Fingerprint |

---

## 5. Component Library & Design System

### 5.1 Built-in Component Library

The framework ships with a complete component library — users don't install Radix, shadcn, or Ant Design separately.

**Core components (all platforms):**
- Button, Input, Text, Label, Checkbox, Radio, Switch, Select
- Dialog, Modal, Popover, Tooltip, Dropdown
- Navigation: NavBar, Sidebar, Tabs, Breadcrumb, Pagination
- Data: Table, List, Card, Badge, Avatar, Progress, Spinner
- Forms: Form, Field, Validation, FileUpload, DatePicker, TimePicker
- Layout: Grid, Flex, Stack, Container, Section, Divider
- Feedback: Toast, Alert, Notification, Confirmation
- Rich text: Tiptap-compatible editor
- Charts: recharts-compatible (SVG-based)
- Maps: leaflet-compatible
- Terminal: xterm-compatible
- 3D: Three.js-compatible (optional)

**Platform-specific components:**
- Desktop: System tray, window controls, file picker, native menus
- Mobile: Bottom navigation, tab bar, modal sheets, touch gestures
- Web: Service worker, PWA install prompt, share API

### 5.2 Animation Engine

Built-in animation system (not Framer Motion dependency):

```rust
// Animation defined once, works on all platforms
animate(Button, {
    hover: { scale: 1.05, duration: 0.2 },
    press: { scale: 0.95, duration: 0.1 },
    layout: { type: "spring", stiffness: 300 },
})
```

Features:
- Layout animations (shared-element transitions)
- Gestures (drag, hover, tap)
- Scroll-linked animations
- Exit animations
- Spring and easing functions

### 5.3 Design System

**Design principles (documented, not just code):**

```
1. Accessibility first — every component is WAI-ARIA compliant
2. Platform-native aesthetics — macOS looks like macOS, Windows like Windows
3. Dark mode by default — every component has light/dark variants
4. Responsive by default — mobile-first breakpoints
5. Typed boundaries — zod-equivalent schemas at every interface
6. Secure by default — CSP, auth guard, secret boundary built-in
```

**Themes (platform-native):**

| Platform | Light theme | Dark theme |
|---|---|---|
| macOS | SF Pro, system colors | SF Pro, dark system colors |
| Windows | Segoe UI, Win11 colors | Segoe UI, dark system colors |
| Linux | System fonts, GTK colors | System fonts, dark GTK |
| iOS | San Francisco, iOS colors | San Francisco, dark iOS colors |
| Android | Material You, system colors | Material You, dark colors |
| Web | Geist, Tailwind-like | Geist, dark variant |

**Layout templates (built-in, copy-paste ready):**
- Dashboard (sidebar + topbar + main)
- Landing page (hero + features + pricing + footer)
- Login/signup (centered card)
- Settings (sidebar + form)
- Data table (filter + table + pagination)
- Kanban board (drag-and-drop columns)
- Calendar (month/week/day views)
- Form wizard (multi-step with validation)

---

## 6. Integrated AI & Agentic Capabilities

### 6.1 Agent Runtime (built-in)

The framework includes a native AI agent layer — not LangChain dependency, but a Rust-native agent runtime:

```rust
// User defines an agent once, works on all platforms
let agent = Agent::new()
    .model(LocalModel::gguf("models/llama-3.1-8b.Q4_K_M.gguf"))
    .system_prompt("You are a code assistant for {{user_name}}.")
    .tool(code_edit_tool())
    .tool(image_generate_tool())
    .tool(spreadsheet_edit_tool())
    .tool(video_edit_tool());
```

### 6.2 Agent Capabilities

| Capability | Description | Example use |
|---|---|---|
| **Code generation** | Write/refactor complex code from natural language | "Refactor this function to use async/await" |
| **Code review** | Analyze code for bugs, security, performance | "Find SQL injection vulnerabilities" |
| **Image generation** | Create images from descriptions | "Generate a logo for my app" |
| **Image editing** | Modify existing images | "Remove background, make 200x200" |
| **Video editing** | Cut, merge, add captions | "Create a 15-second promo from these clips" |
| **Spreadsheet automation** | Read/write/edit Excel/CSV | "Sum column B and create a chart" |
| **Document automation** | Generate Word/PDF documents | "Create an invoice from this data" |
| **Email automation** | Draft, send, manage emails | "Send weekly report to team" |

### 6.3 Security for AI (from v2 spec)

- Tool registry is allowlist-only (agent cannot call unregistered tools)
- Tool input validated with zod-equivalent schemas
- Tool output validated before returning
- System prompt uses variable substitution (no string concatenation)
- User input screened for prompt injection ("ignore previous instructions", etc.)
- No eval/require/shell tools by default
- Agent output sanitized before UI render
- Output length limit (default 4000 chars)
- Model API keys never in client bundle
- Local model preferred (llama.cpp GGUF, on-device, no API keys transmitted)
- Auth guard on agent endpoint (401 on unauthenticated)

### 6.4 Model Providers

| Provider | Mode | API keys needed? |
|---|---|---|
| **Local llama.cpp** | On-device GGUF inference | No (fully private) |
| **Google Gemini** | Cloud API | Yes (server-side only) |
| **OpenAI via OpenRouter** | Cloud API | Yes (server-side only) |

**Default:** Local llama.cpp for privacy. Cloud models used only when explicitly configured.

---

## 7. Documentation & Showcase Strategy

### 7.1 Documentation Site (Netlify)

**Structure:**

```
docs/
├── index.md              # Landing page — what Innoframework is
├── get-started/          # Installation and setup
│   ├── install.md        # CLI installation (no Node.js)
│   ├── quickstart.md     # 5-minute app build
│   └── wizard.md         # Interactive setup wizard guide
├── core/                 # Core framework concepts
│   ├── architecture.md   # How Innoframework works under the hood
│   ├── security.md       # Security model, CSP, auth, secrets
│   ├── reactivity.md     # Signal-based reactivity explained
│   └── build.md          # Build pipeline, targets
├── ui/                   # Component library
│   ├── components/       # Each component: props, examples, platform notes
│   ├── animations.md     # Animation engine reference
│   ├── themes.md         # Platform-native themes
│   └── layouts.md        # Layout templates with copy-paste code
├── ai/                   # AI agent capabilities
│   ├── agents.md         # Creating and configuring agents
│   ├── tools.md          # Built-in tools (code, image, spreadsheet, etc.)
│   ├── security.md       # AI security boundaries
│   └── models.md         # Local vs cloud model providers
├── deploy/               # Deployment options
│   ├── web.md            # Deploy to Netlify/Vercel/Cloudflare
│   ├── desktop.md        # Build .exe/.app/.deb
│   └── mobile.md         # Build .ipa/.apk
├── api/                  # Auto-generated API reference
│   ├── inno_cli.md   # CLI command reference
│   ├── framework_api.md  # Rust API reference
│   └── agent_api.md      # Agent runtime API
├── examples/             # Real-world application showcases
│   ├── finance-app.md    # Personal finance app (like innofinance)
│   ├── erp-module.md     # ERP module (like Open_Erp)
│   ├── ai-agent.md       # Code assistant agent
│   └── desktop-tool.md   # Native desktop application
├── contributing.md       # How to contribute
├── changelog.md          # Version history
└── faq.md                # Frequently asked questions
```

**Showcase applications (built with Innoframework, hosted on Netlify):**

| Showcase | Description | Platform | Live demo |
|---|---|---|---|
| Innoframework Finance | Personal finance app (income/expense, AI reminders) | Web + Mobile | netlify.app |
| Innoframework ERP | ERP module (HR, FICO, SD, MM, MFG) | Web | netlify.app |
| Innoframework Code Agent | AI code assistant (refactor, generate, review) | Desktop | download |
| Innoframework Desktop Locker | File/folder locker (like InnologyLock) | Desktop | download |
| Innoframework CRM | Customer relationship management | Web + Mobile | netlify.app |

### 7.2 Documentation Hosting

- **Platform:** Netlify (matches user's existing hosting pattern)
- **Auto-deploy:** From `main` branch on push
- **Domain:** docs.innologybd.com (or innologybd.com/docs)
- **SSL:** Automatic via Netlify
- **CSP:** Strict CSP on docs site (matches security checklist)

---

## 8. What Innoframework Is NOT

| Not this | Because |
|---|---|
| Not an npm package | Users shouldn't install Node.js |
| Not a React wrapper | React/Node.js are hackable runtimes |
| Not a PWA builder | PWA is web-in-a-browser, not native |
| Not a Next.js clone | Next.js requires Node.js, server-side complexity |
| Not a Tauri clone | Tauri still requires Node.js for frontend dev |
| Not a LangChain wrapper | Built-in Rust agent runtime, not dependency |
| Not a design system theme | Full component library + animation engine + layout templates |

---

## 9. Build Pipeline

```
User writes Rust code (src/app.rs)
         │
         ▼
┌─────────────────────────────────────────┐
│  Innoframework Build Pipeline                │
│                                         │
│  1. Parse & type-check (Rust compiler)  │
│  2. Security scan (secret boundary)     │
│  3. Compile per target:                 │
│     ├── Web → WASM + JS glue            │
│     ├── Desktop → Native binary         │
│     └── Mobile → Native binary          │
│  4. Bundle assets (images, fonts, etc.) │
│  5. Generate installer per platform     │
│                                         │
└─────────────────────────────────────────┘
         │
         ▼
   Installer ready for distribution
```

**Build commands:**
```bash
inno build web      # WASM output for Netlify/Vercel
inno build desktop  # .exe/.app/.deb
inno build mobile   # .ipa/.apk
inno build all      # All targets
```

---

## 10. Open Questions (to resolve before coding)

1. **Language:** Rust confirmed (memory safety, matches user's Rust projects, Dioxus cross-platform). No Node.js, no Python, no Go in the core.
2. **UI framework:** Dioxus 0.7 (signal-based, RSX, cross-platform). Alternative: native Rust GUI (egui, iced) — but Dioxus has web support.
3. **Database:** SQLite local-first (desktop/mobile), Firestore/Postgres optional (web/cloud).
4. **Auth:** Built-in session auth (HMAC-signed tokens, like v1 design) + optional Firebase/Supabase integration.
5. **Package registry:** Innoframework modules distributed via `innologybd.com/modules` (not npm).
6. **v1 scope:** Single codebase → web + desktop. Mobile in v1.1. AI agents in v1.2.

---

## 11. Immediate Next Steps (Implementation Order)

| Step | Task | Deliverable |
|---|---|---|
| 1 | Define Rust crate structure (core, ui, agent, auth, build) | Cargo workspace |
| 2 | Build CLI binary (`inno init`, `inno build`) | Working CLI |
| 3 | Implement signal-based reactivity | Reactive UI runtime |
| 4 | Build core component library (Button, Input, Dialog, etc.) | Usable UI kit |
| 5 | Implement auth guard + CSP + secret boundary | Secure by default |
| 6 | Build desktop renderer (WGPU or webview) | Native desktop app |
| 7 | Build web renderer (WASM) | Deployable web app |
| 8 | Implement AI agent runtime (local llama.cpp) | Working agent |
| 9 | Build documentation site | Netlify deployment |
| 10 | Build showcase applications | Proof of capability |

---

**End of blueprint.** This is the architectural foundation. Code comes after design is approved.
