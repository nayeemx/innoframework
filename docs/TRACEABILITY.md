# Security Traceability Matrix

Every security requirement must map to a negative test before its phase gate
can pass. Status values: `not started`, `test written`, `passing`.
No row may be marked `passing` without a test ID that exists in the repo.

Source checklists: `SECURITY_CHECKLIST_V1.md`, `SECURITY_CHECKLIST_V2.md`.

| Req ID | Requirement (short) | Phase | Test ID | Status |
|---|---|---|---|---|
| CSP-1 | CSP header on all responses | P3 | — | not started |
| CSP-2 | No unsafe-inline in script-src | P3 | — | not started |
| CSP-3 | No unsafe-eval | P3 | — | not started |
| CSP-4 | object-src 'none' | P3 | — | not started |
| CSP-5 | base-uri 'none' | P3 | — | not started |
| CSP-6 | frame-ancestors 'none' | P3 | — | not started |
| CSP-7 | form-action 'self' | P3 | — | not started |
| CSP-8 | Strict policy (nonce or hash) | P3 | — | not started |
| CSP-9 | report-to / report-uri configured | P3 | — | not started |
| CSP-10 | CSP not the sole defense | P3 | — | not started |
| XSS-1 | Context-specific output encoding | P4 | — | not started |
| XSS-2 | No untrusted data in dangerous contexts | P4 | — | not started |
| XSS-3 | Safe DOM sinks only | P4 | — | not started |
| XSS-4 | DOMPurify-equivalent for user HTML | P4 | — | not started |
| XSS-5 | dangerouslySetInnerHTML blocked | P4 | — | not started |
| XSS-6 | URL scheme allowlist | P4 | — | not started |
| XSS-7 | textContent default | P4 | — | not started |
| XSS-8 | Safe element creation | P4 | — | not started |
| XSS-9 | Trusted Types where supported | P3 | — | not started |
| CSRF-1 | State changes use POST/PUT/PATCH/DELETE only | P2 | — | not started |
| CSRF-2 | SameSite cookie attribute | P2 | — | not started |
| CSRF-3 | Sec-Fetch-Site check on non-safe methods | P2 | — | not started |
| CSRF-4 | Custom header on API calls | P2 | — | not started |
| CSRF-5 | No wildcard CORS with credentials | P2 | — | not started |
| CSRF-6 | Fail closed when Sec-Fetch-* absent | P2 | — | not started |
| CSRF-7 | Login endpoints covered | P2 | — | not started |
| CSRF-8 | Unpredictable per-session tokens | P2 | — | not started |
| SEC-BUILD-1 | Server env not in client bundle | P1 | — | not started |
| SEC-BUILD-2 | Secret leak is a build error, not a warning | P1 | — | not started |
| SEC-BUILD-3 | Public config allowed, secrets not | P1 | — | not started |
| SEC-RPC-1 | Unauthenticated call returns 401 (fail closed) | P2 | — | not started |
| SEC-RPC-2 | Typed validation before handler | P2 | — | not started |
| SEC-RPC-3 | Server-only code never in client build | P2 | — | not started |
| SEC-RPC-4 | Error messages do not leak internals | P2 | — | not started |
| AI-1 | Tool registry is allowlist-only | P7 | — | not started |
| AI-2 | Tool input validated | P7 | — | not started |
| AI-3 | Tool output validated | P7 | — | not started |
| AI-4 | System prompt uses substitution only | P7 | — | not started |
| AI-5 | Unreplaced placeholders stripped | P7 | — | not started |
| AI-6 | Injection screening | P7 | — | not started |
| AI-7 | No eval/require/shell tools by default | P7 | — | not started |
| AI-8 | Output sanitized before render | P7 | — | not started |
| AI-9 | Output length limit | P7 | — | not started |
| AI-10 | No raw model output to client | P7 | — | not started |
| AI-11 | Model keys not in client bundle | P1 | — | not started |
| AI-12 | Local model preferred for secrets | P7 | — | not started |
| AI-13 | Auth guard on agent endpoint | P7 | — | not started |
| AI-14 | Typed validation on agent input | P7 | — | not started |
| AI-15 | Agent errors non-leaky | P7 | — | not started |

## Additional requirements from the architecture plan (no checklist source)

| Req ID | Requirement | Phase | Test ID | Status |
|---|---|---|---|---|
| TR-1.3 | Unguarded RPC route fails to compile | P2 | — | not started |
| TR-3.2 | Platform call without capability fails at runtime; build fails if missing | P5 | — | not started |
| TR-5.5 | File tool rejects `..`, absolute paths, and symlink escapes | P7 | — | not started |
| EDGE-RPC-413 | Oversized body returns 413 | P2 | — | not started |
| EDGE-SESSION-EXP | Valid signature, expired token returns 401 | P2 | — | not started |
| EDGE-SESSION-KEY | Same payload, different key rejected | P2 | — | not started |
| EDGE-ROUTE-DUP | Duplicate route name is a compile error | P2 | — | not started |
| EDGE-AGENT-XSS | Tool output containing script is sanitized | P7 | — | not started |
| EDGE-LOCALE | Bangla text round-trips through RPC and CSV | P2 | — | not started |

## Counts
- Checklist items: 49 (V1: CSP 10, XSS 9, CSRF 8, SEC-BUILD 3, SEC-RPC 4 = 34; V2: AI 15).
- Architecture-plan items: 9 (TR 3 plus EDGE 6 rows, counted as listed above).
- Total rows: 58.
- Passing: 0.
