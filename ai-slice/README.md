# AI Agent Slice (v2)

Minimal demonstration of framework v2: AI agent module with typed tool
contracts, agent runtime, and security boundaries.

## Stack
- TypeScript (strict)
- zod (tool input/output validation)
- vitest (tests)

## Scripts
- `npm run dev` — runs the demo agent with mock model
- `npm run build` — type check + secret check
- `npm test` — unit + security tests
- `npm run check:secrets` — blocks model API keys from client code

## Design
- `createTool()` — declares a tool with zod input/output schemas
- `ToolRegistry` — allowlist-only tool execution
- `Agent` / `createAgent()` — orchestrates model calls + tool execution
- `MockModelProvider` — deterministic demo provider (no API key needed)
- Security: prompt injection detection, output sanitization, output length limits

## Security
- Tools are allowlist-only — agents cannot call unregistered tools
- Tool input/output validated with zod
- System prompts use variable substitution, not string concatenation
- Agent output sanitized (script tags stripped)
- Build check blocks model API keys from client code

## Next step
Replace MockModelProvider with a real provider (Gemini/OpenAI/Ollama) behind
the ModelProvider interface. API keys must never appear in client bundles.
