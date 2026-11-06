/* ------------------------------------------------------------------ */
/* Llama.cpp local model provider                                     */
/* Connects to a locally running llama-server.exe instance serving a  */
/* GGUF model via an OpenAI-compatible /v1/chat/completions endpoint. */
/*                                                                    */
/* Prerequisite:                                                      */
/*   D:\models\llama.cpp\b9142\llama-server.exe --model <gguf>       */
/*       --port 11434 --host 127.0.0.1                                */
/*                                                                    */
/* The server exposes POST http://127.0.0.1:11434/v1/chat/completions */
/* ------------------------------------------------------------------ */

import { ModelProvider, Message } from "./model.ts";

export interface LlamaCppProviderOptions {
  baseUrl?: string;
  model?: string;
  temperature?: number;
  numPredict?: number;
  timeoutMs?: number;
}

/* ------------------------------------------------------------------ */
/* LlamaCppProvider                                                   */
/* ------------------------------------------------------------------ */

export class LlamaCppProvider implements ModelProvider {
  readonly name = "llama.cpp";
  private baseUrl: string;
  private model: string;
  private temperature: number;
  private numPredict: number;
  private timeoutMs: number;

  constructor(opts: LlamaCppProviderOptions = {}) {
    this.baseUrl = opts.baseUrl ?? "http://127.0.0.1:11434";
    this.model = opts.model ?? "llama3.2-3b";
    this.temperature = opts.temperature ?? 0.7;
    this.numPredict = opts.numPredict ?? 512;
    this.timeoutMs = opts.timeoutMs ?? 30000;
  }

  async generate(
    messages: Message[],
    options?: {
      tools?: readonly string[];
      temperature?: number;
    },
  ): Promise<string> {
    const temperature = options?.temperature ?? this.temperature;

    /* Convert our Message format to OpenAI chat format */
    const openAiMessages = messages.map((m) => ({
      role: m.role as "system" | "user" | "assistant" | "tool",
      content: m.content,
    }));

    const body: any = {
      model: this.model,
      messages: openAiMessages,
      temperature,
      max_tokens: this.numPredict,
      stream: false,
    };

    /* llama-server supports tool_choice when tools are listed */
    if (options?.tools?.length) {
      body.tools = options.tools.map((name) => ({
        type: "function",
        function: { name, description: "" },
      }));
      body.tool_choice = "required";
    }

    const controller = new AbortController();
    const timeoutId = setTimeout(() => controller.abort(), this.timeoutMs);

    try {
      const res = await fetch(`${this.baseUrl}/v1/chat/completions`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(body),
        signal: controller.signal,
      });

      if (!res.ok) {
        const text = await res.text().catch(() => "");
        throw new Error(
          `llama-server returned ${res.status}: ${text}`,
        );
      }

      const data = (await res.json()) as {
        choices?: Array<{ message?: { content?: string } }>;
      };
      return data.choices?.[0]?.message?.content ?? "";
    } catch (err: any) {
      if (err.name === "AbortError") {
        throw new Error(
          `llama-server request timed out after ${this.timeoutMs}ms`,
        );
      }
      throw err;
    } finally {
      clearTimeout(timeoutId);
    }
  }
}

/* ------------------------------------------------------------------ */
/* Helper: check if llama-server is reachable                         */
/* ------------------------------------------------------------------ */

export async function isLlamaServerReachable(
  baseUrl: string = "http://127.0.0.1:11434",
): Promise<boolean> {
  try {
    const res = await fetch(`${baseUrl}/v1/models`, {
      method: "GET",
      signal: AbortSignal.timeout(3000),
    });
    return res.ok;
  } catch {
    return false;
  }
}
