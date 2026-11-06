# Framework v2 — AI Agent Module (Spec)

## Purpose
An AI agent module that wraps an existing agent library (LangChain/LangGraph or equivalent) with **typed tool definitions generated from the same typed contracts** used by v1 server functions. The module does not replace the agent library; it provides a secure, typed boundary between the framework's server functions and the agent's tool-calling layer.

## Scope (in-scope)
- AI agent runtime that can invoke registered tools
- Tool registry: tools are declared with zod schemas (same style as v1 server function validators)
- Typed tool contracts: tool name + input schema + output shape, shared between client and server
- Agent executor with a configurable model provider (local llama.cpp via `LlamaCppProvider`, or mock for testing)
- System prompt management with injection-safe variable substitution
- Output validation: agent results validated before returning to caller
- Security: tool access control, prompt injection mitigations, no arbitrary code execution, output sanitization
- Tests: unit tests for tool validation, agent routing, and security boundaries

## Out of scope (explicitly deferred)
- Training/fine-tuning models
- Vector database / RAG (deferred to v2.1 or later)
- Multi-agent orchestration beyond a single agent with tools
- Model hosting or inference infrastructure beyond local llama.cpp
- Speech-to-text / text-to-speech (imrul already handles this separately)
- Streaming (deferred — non-streaming `generate()` only)

## Design principles
1. **Wrap, don't replace**: Use an agent runtime under the hood; the framework adds typing and security boundaries on top.
2. **Typed tool contracts**: Tools are declared once with zod schemas; the same schema validates both the agent's tool call and the framework's server function input.
3. **Fail closed**: If a tool is not registered, the agent cannot call it. If tool output fails validation, the call is rejected.
4. **No arbitrary code execution**: Agents cannot execute `eval`, `require`, `import`, or shell commands unless explicitly registered as a tool with a security review.
5. **Prompt injection defense**: System prompts use variable substitution (not string concatenation); user input is never injected into system instructions.
6. **Local-first models**: Prefer local GGUF models via `llama-server.exe` (D:\models\llama.cpp\b9142\llama-server.exe) so no API keys leave the machine.

## Model providers
- `MockModelProvider` — deterministic demo provider, no API key, for testing
- `LlamaCppProvider` — calls local `llama-server.exe` at `http://127.0.0.1:11434/v1/chat/completions` (OpenAI-compatible)

## Tool contract format
```ts
export const weatherTool = createTool({
  name: "getWeather",
  description: "Get current weather for a city",
  input: z.object({ city: z.string().min(1) }),
  output: z.object({ temperature: z.number(), condition: z.string() }),
  handler: async ({ city }) => { ... },
});
```

## Agent runtime
```ts
const agent = createAgent({
  model: new MockModelProvider(),
  tools: [weatherTool],
  systemPromptTemplate: "You are a helpful assistant for {{userName}}.",
  systemPromptVars: { userName: "Developer" },
});

const result = await agent.run("What's the weather in Dhaka?");
// result: { ok: true, content: "...", toolCalls: [...] }
```

## Security checklist (v2)
| # | Check | Requirement |
|---|---|---|
| AI-1 | Tool registry is allowlist-only | Agent cannot call unregistered tools |
| AI-2 | Tool input validated with zod | Every tool call validated before handler runs |
| AI-3 | Tool output validated | Handler return validated against output schema |
| AI-4 | System prompt uses substitution | User input never concatenated into system instructions |
| AI-5 | Unreplaced placeholders stripped | Template structure never leaks into prompt |
| AI-6 | User input screened for injection | `detectPromptInjection()` flags dangerous patterns |
| AI-7 | No eval/require/shell tools by default | Arbitrary code execution requires explicit review |
| AI-8 | Agent output sanitized before UI render | Script tags, javascript: URLs, event handlers stripped |
| AI-9 | Output length limit | Responses exceeding max length are rejected |
| AI-10 | Model API keys never in client bundle | Build-time secret check blocks client-side keys |
| AI-11 | Local model preferred for secrets | Local GGUF via llama-server keeps inference on-device |
| AI-12 | Auth guard on agent endpoint | Every agent server function requires authenticated session |
| AI-13 | Typed validation on agent input | Agent request input validated with zod |
| AI-14 | Agent output non-leaky | Generic errors to client; internals logged server-side |

## Threat model (top risks)
1. Prompt injection: user input tricks agent into calling tools with malicious args → mitigated by zod validation + injection detection
2. Tool overreach: agent calls unregistered tool → mitigated by allowlist-only registry
3. Output injection: agent returns malicious HTML/JS → mitigated by output sanitization
4. Secret leakage: model API key in client bundle → mitigated by build-time secret check
5. Local model DoS: long-running inference blocks the agent → mitigated by maxSteps and timeoutMs

## Target test project
- `D:\projects\code\framework-research\ai-slice` (standalone demo)
- `D:\projects\code\portfolio` (integrated: `/rpc/agent` server function endpoint)

## Definition of done per phase
| Phase | Check |
|---|---|
| Spec | Written, reviewed |
| Tool registry | createTool + zod validation works |
| Agent runtime | createAgent + run() returns typed result |
| Model providers | MockModelProvider + LlamaCppProvider implemented |
| Security | AI-1 through AI-14 tested |
| Build | Secret check blocks client-side API keys |
| Integration | Agent wired into portfolio `/rpc/agent` endpoint |
| Tests | Unit + integration tests pass |

## Open questions
1. Streaming: deferred — non-streaming `generate()` only for v2.
2. Tool calling format: llama-server native tool calling vs custom JSON parsing from text.
