/* ------------------------------------------------------------------ */
/* Agent runtime                                                      */
/* Orchestrates model calls and tool execution with security          */
/* boundaries at every step.                                          */
/* ------------------------------------------------------------------ */

import { ToolRegistry, Tool } from "./tool";
import { ModelProvider, Message } from "./model";
import {
  renderSystemPrompt,
  sanitizeAgentOutput,
  detectPromptInjection,
  validateAgentOutput,
} from "./security";

export interface AgentOptions {
  model: ModelProvider;
  tools?: readonly Tool<any, any>[];
  systemPromptTemplate?: string;
  systemPromptVars?: Record<string, string>;
  maxSteps?: number;
}

export interface AgentResult {
  readonly ok: boolean;
  readonly content?: string;
  readonly toolCalls?: Array<{ toolName: string; input: unknown; output: unknown }>;
  readonly error?: string;
}

export class Agent {
  private model: ModelProvider;
  private registry: ToolRegistry;
  private systemPrompt: string;
  private maxSteps: number;

  constructor(opts: AgentOptions) {
    this.model = opts.model;
    this.registry = new ToolRegistry();
    for (const tool of opts.tools ?? []) {
      this.registry.register(tool);
    }
    this.systemPrompt = opts.systemPromptTemplate
      ? renderSystemPrompt(opts.systemPromptTemplate, opts.systemPromptVars ?? {})
      : "You are a helpful assistant.";
    this.maxSteps = opts.maxSteps ?? 5;
  }

  async run(userInput: string): Promise<AgentResult> {
    /* 1. Prompt injection check on user input */
    const injectionFlags = detectPromptInjection(userInput);
    if (injectionFlags.length > 0) {
      return {
        ok: false,
        error: `Prompt injection detected: ${injectionFlags.join(", ")}`,
      };
    }

    const messages: Message[] = [
      { role: "system", content: this.systemPrompt },
      { role: "user", content: userInput },
    ];

    const toolCalls: Array<{ toolName: string; input: unknown; output: unknown }> = [];

    for (let step = 0; step < this.maxSteps; step++) {
      const response = await this.model.generate(messages, {
        tools: this.registry.list(),
      });

      /* 2. Validate model output length */
      const outputCheck = validateAgentOutput(response);
      if (!outputCheck.ok) {
        return { ok: false, error: outputCheck.error };
      }

      /* 3. Try to parse as a tool call */
      const toolCall = this.tryParseToolCall(response);
      if (toolCall) {
        /* 4. Execute tool through registry (allowlist + validation) */
        const result = await this.registry.call(toolCall);
        toolCalls.push({
          toolName: toolCall.toolName,
          input: toolCall.input,
          output: result.ok ? result.output : result.error,
        });

        messages.push({ role: "assistant", content: response });
        messages.push({
          role: "tool",
          content: result.ok
            ? JSON.stringify(result.output)
            : `Error: ${result.error}`,
        });
        continue;
      }

      /* 5. Final answer — sanitize before returning */
      return {
        ok: true,
        content: sanitizeAgentOutput(response),
        toolCalls,
      };
    }

    return { ok: false, error: "Max steps reached" };
  }

  private tryParseToolCall(
    response: string,
  ): { toolName: string; input: unknown } | null {
    try {
      const parsed = JSON.parse(response);
      if (
        parsed &&
        typeof parsed === "object" &&
        typeof parsed.tool === "string" &&
        parsed.input !== undefined
      ) {
        return { toolName: parsed.tool, input: parsed.input };
      }
    } catch {
      /* Not JSON — treat as final answer */
    }
    return null;
  }
}

/* ------------------------------------------------------------------ */
/* Default agent factory                                              */
/* ------------------------------------------------------------------ */

export function createAgent(opts: AgentOptions): Agent {
  return new Agent(opts);
}
