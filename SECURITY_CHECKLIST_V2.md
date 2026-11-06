# Security Checklist — Framework v2 (AI Agent Module)

Source: OWASP Cheat Sheet Series (LLM Prompt Injection, RAG Security, Secrets Management), plus v1 security principles adapted for AI agents.

---

## 1. Tool access control (AI-1 through AI-3)

| # | Check | Requirement |
|---|---|---|
| AI-1 | Tool registry is allowlist-only | Agent cannot call unregistered tools. `ToolRegistry.call()` returns `ok: false` for unknown tool names. |
| AI-2 | Tool input validated with zod | Every tool call validated against its input schema before the handler runs. Invalid input → `ok: false` with `Invalid input` error. |
| AI-3 | Tool output validated | Handler return validated against output schema. Invalid output → `ok: false` with `Invalid output` error. |

---

## 2. Prompt injection defense (AI-4)

| # | Check | Requirement |
|---|---|---|
| AI-4 | System prompt uses variable substitution | `renderSystemPrompt()` replaces `{{key}}` placeholders with sanitized values. User input is never concatenated into system instructions. |
| AI-5 | Unreplaced placeholders stripped | Any `{{...}}` not in the vars map is removed from the rendered prompt to prevent template leakage. |
| AI-6 | User input screened for injection | `detectPromptInjection()` flags patterns like "ignore previous instructions", "disregard", "system prompt", "you are now", "admin mode", "execute", "eval(". Flagged input → agent returns error, does not proceed. |
| AI-7 | No eval/require/shell tools by default | Arbitrary code execution tools are not registered unless explicitly added after security review. |

---

## 3. Output sanitization (AI-5 through AI-7)

| # | Check | Requirement |
|---|---|---|
| AI-8 | Agent output sanitized before UI render | `sanitizeAgentOutput()` strips `<script>` tags, `javascript:` URLs, and inline event handlers (`on*=`) from agent text output. |
| AI-9 | Output length limit | `validateAgentOutput()` rejects responses exceeding the configured max length (default 4000 chars). |
| AI-10 | No raw model output to client | Agent output always passes through sanitization before being sent to the client. |

---

## 4. Secret management (AI-6)

| # | Check | Requirement |
|---|---|---|
| AI-11 | Model API keys never in client bundle | Build-time secret check (`scripts/check-secrets.js`) blocks `GEMINI_API_KEY`, `OPENAI_API_KEY`, `ANTHROPIC_API_KEY`, `SECRET_*`, `PRIVATE_*` in client source. |
| AI-12 | Local model preferred for secrets | Using `LlamaCppProvider` with a local GGUF model keeps all inference on-device; no API keys transmitted over the network. |

---

## 5. RPC / server-function boundary (when Agent is exposed via v1 server function)

| # | Check | Requirement |
|---|---|---|
| AI-13 | Auth guard on agent endpoint | Every agent server function requires an authenticated session (401 on unauthenticated). |
| AI-14 | Typed validation on agent input | Agent request input validated with zod before reaching the Agent. |
| AI-15 | Agent output non-leaky | Generic error messages to client; stack traces and model internals logged server-side only. |

---

## Definition of done for v2 security
- Every AI-1 through AI-15 item has a corresponding test in `src/test/ai.test.ts` or the integration test suite.
- Build-time secret check enforced on the portfolio build pipeline.
- Agent integrated behind the v1 auth guard on `/rpc/*`.
