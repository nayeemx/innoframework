# Threat Model (P0)

Status: draft for owner review. Source: `INNOFRAMEWORK_ARCHITECTURE.md` §2.2
("Threat model (testable)") and ADR-04 (default-deny security).

Each threat has a test ID. A threat is `passing` only when a negative test with
that ID exists in the repo and passes. IDs match `docs/TRACEABILITY.md` where a
requirement already exists there.

## 1. Scope

In scope for v1:

- RPC server functions (typed calls from client to server)
- Session tokens (HMAC-signed, constant-time verification) in `innoframework-core`
- Build-time checks: server-only module leakage, secret strings in client bundles
- Web output: Content Security Policy and output escaping
- Agent/AI features (prompt injection, tool allowlist)

Out of scope for P0: store publishing (ADR-13), production key management
(ADR-09), network hardening of the VPS (ADR-10 transition).

## 2. Assets

| Asset | Why it matters |
|---|---|
| Session token signing key | Forging it grants any role |
| Server-only code and secrets | Leakage exposes credentials and internal logic |
| Authenticated user data | Cross-user access |
| Agent tool registry | Unregistered tools could run arbitrary actions |

## 3. Trust boundaries

1. Browser or client app to server (RPC over HTTP). All input untrusted.
2. Build pipeline to client bundle. Build output is public.
3. Agent output to rendered HTML. Model output is untrusted.
4. Agent to tools. Tool calls must come from an allowlist.

## 4. Threats and tests

| ID | Threat | Mitigation | Test | Status |
|---|---|---|---|---|
| TM-1 | RPC call without a valid session | Auth guard on every RPC (ADR-04) | `rpc_without_session_returns_401` | not started |
| TM-2 | Forged or tampered session token | HMAC-SHA256, constant-time compare, expiry check | `tampered_payload_is_rejected`, `wrong_key_is_rejected`, `expired_token_is_rejected` (exist in `innoframework-core`) | test written (core unit tests) |
| TM-3 | Role escalation (user calls admin route) | Fail-closed `require_role` | `user_cannot_access_admin_route` (exists in `innoframework-core`) | test written (core unit tests) |
| TM-4 | Server-only module imported by client code | Build-time import graph check | `server_only_import_fails_build` | not started |
| TM-5 | Build-time secret present in client bundle | Build-time secret scan | `secret_in_client_bundle_fails_build` | not started |
| TM-6 | User or model output rendered as HTML | CSP blocks inline scripts; output escaped | `inline_script_blocked_by_csp`, `output_is_escaped` | not started |
| TM-7 | Prompt injection makes the agent proceed | Agent returns error and does not act | `prompt_injection_returns_error` | not started |
| TM-8 | Agent calls an unregistered tool | Allowlist-only tool registry | `unregistered_tool_is_rejected` | not started |
| TM-9 | Weak signing key accepted at startup | Minimum length check (`MIN_SECRET_LEN` = 32) | `short_secret_is_refused` (exists in `innoframework-core`) | test written (core unit tests) |
| TM-10 | Key material leaked through logs or Debug | Redacted `Debug` for `SessionKey` | `debug_does_not_leak_key` (exists in `innoframework-core`) | test written (core unit tests) |

Rows TM-4 to TM-8 depend on phases P2 to P3 and have no code yet. They remain
`not started`.

## 5. Known gaps

- Token expiry is enforced by the caller passing `now`. Clock source is not yet
  defined.
- No token revocation. A stolen token is valid until expiry.
- Rate limiting and brute-force protection are not in scope for P0.
- `cookies.txt` exists at the repo root (untracked and ignored). It must never be
  committed.

## 6. Review

Re-review at the end of P2 (RPC and auth) and P3 (web build).
