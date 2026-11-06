# Security Checklist — Framework v1

Source: OWASP Cheat Sheet Series (Content Security Policy, Cross-Site Request Forgery Prevention, Cross Site Scripting Prevention), fetched Nov 2026. Each item below maps to a testable requirement for v1.

---

## 1. Content Security Policy (CSP)

| # | Check | Requirement |
|---|---|---|
| CSP-1 | Default CSP header sent on all HTTP responses | `Content-Security-Policy` header, not just index page. No `X-Content-Security-Policy` / `X-WebKit-CSP`. |
| CSP-2 | No `unsafe-inline` in `script-src` | Blocks injected inline scripts and inline event handlers. |
| CSP-3 | No `unsafe-eval` | Blocks text-to-JavaScript functions like `eval()`. |
| CSP-4 | `object-src 'none'` | Blocks plugins/object/embed/applet injection. |
| CSP-5 | `base-uri 'none'` | Blocks `<base>` tag hijacking. |
| CSP-6 | `frame-ancestors 'none'` | Blocks clickjacking (all framing); `'self'` if same-origin framing needed. |
| CSP-7 | `form-action 'self'` | Restricts form submissions to same origin; blocks phishing form actions. |
| CSP-8 | Strict policy preferred | Use nonce-based or hash-based strict CSP where practical; allowlist-only policies are weaker. |
| CSP-9 | `report-to` + `report-uri` configured | Violations reported to an endpoint; backward-compatible dual declaration. |
| CSP-10 | No CSP reliance as sole defense | CSP is defense-in-depth; output encoding and input validation still required. |

**Default CSP preset for v1:**
```
Content-Security-Policy: default-src 'self'; script-src 'self'; connect-src 'self'; img-src 'self'; style-src 'self'; frame-ancestors 'none'; form-action 'self'; object-src 'none'; base-uri 'none';
```

---

## 2. Cross-Site Request Forgery (CSRF)

| # | Check | Requirement |
|---|---|---|
| CSRF-1 | Every state-changing request protected | POST / PUT / PATCH / DELETE require CSRF mitigation; GET never used for state changes. |
| CSRF-2 | SameSite cookie attribute on session cookies | `SameSite=Lax` minimum; `Strict` for highly sensitive operations. |
| CSRF-3 | Fetch Metadata `Sec-Fetch-Site` check | Primary signal: reject non-safe methods when `Sec-Fetch-Site: cross-site`. Fallback to Origin/Referer verification for legacy browsers. |
| CSRF-4 | Custom request header for AJAX/API endpoints | E.g. `X-CSRF-Token` on fetch/XHR; custom headers trigger CORS preflight, which browsers enforce. |
| CSRF-5 | Custom header + credentials restriction | If credentials allowed, `Access-Control-Allow-Origin` must be explicit origins, never `*`. |
| CSRF-6 | Fallback when `Sec-Fetch-*` absent | Fail-safe (block unknown) preferred; fail-open only with explicit documented exceptions. |
| CSRF-7 | Login forms protected | Login endpoints included in CSRF mitigation. |
| CSRF-8 | Tokens unique per session | Synchronizer token pattern if used: per-session or per-request, unpredictable, secret. |

---

## 3. Cross-Site Scripting (XSS)

| # | Check | Requirement |
|---|---|---|
| XSS-1 | Context-specific output encoding | HTML body → HTML entity encoding; HTML attribute → attribute encoding; URL → URL encoding; JS context → JS string encoder; CSS → strict structural validation. |
| XSS-2 | No dangerous contexts for variables | Script blocks, HTML comments, CSS blocks, event-handler attributes, `eval`, `setTimeout`, `setInterval` — untrusted data never placed there. |
| XSS-3 | Prefer safe DOM sinks | `textContent`, `insertAdjacentText`, `setAttribute`, `className`, `formfield.value`. Avoid `innerHTML`, `outerHTML`, `document.write`. |
| XSS-4 | HTML sanitization for user-authored HTML | Use DOMPurify when user-authored HTML must be accepted; never modify sanitized output after sanitizing. |
| XSS-5 | React `dangerouslySetInnerHTML` blocked | No raw HTML insertion without sanitization; React 19 blocks `javascript:` URLs in src/href but does not replace app validation. |
| XSS-6 | URL validation for URL-bearing attributes | Allowlist http/https schemes; reject `javascript:` protocol in `href`/`src`. |
| XSS-7 | `textContent` over `innerHTML` | Default to textContent for user data. |
| XSS-8 | Safe `createElement` | Tag name from constant or allowlist, never untrusted input. |
| XSS-9 | Trusted Types where supported | `require-trusted-types-for 'script'` via CSP; policies apply validation/sanitization. |

---

## 4. Build-time secret boundary

| # | Check | Requirement |
|---|---|---|
| SEC-BUILD-1 | Server env vars not in client bundle | Build fails if `process.env.SERVER_*` or equivalent appears in client output. |
| SEC-BUILD-2 | Secrets never logged to client | Build-time error, not runtime warning. |
| SEC-BUILD-3 | Public config allowed only | Firebase web config (public project ID) may be bundled; true secrets must fail the build. |

---

## 5. RPC / server-function boundary

| # | Check | Requirement |
|---|---|---|
| SEC-RPC-1 | Auth guard on every server function | Unauthenticated call → 401. Fail closed. |
| SEC-RPC-2 | Typed validation on all inputs | Zod schema or equivalent; invalid input rejected before handler runs. |
| SEC-RPC-3 | Server-only code never shipped | Compiler/build transform keeps server function body out of client stubs. |
| SEC-RPC-4 | Error messages non-leaky | Generic error messages to client; stack traces logged server-side only. |

---

## Definition of done for v1 security
- Every CSP- and XSS- item above has a corresponding negative test in the slice repo.
- Build-time secret check enforced on the target project (`portfolio`) build pipeline.
- CSRF check enforced via Sec-Fetch-Site + SameSite cookies on the RPC endpoint.
