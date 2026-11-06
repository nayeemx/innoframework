/* ------------------------------------------------------------------ */
/* ONNX Runtime adapter                                               */
/* Runs an ONNX model in the browser via onnxruntime-web, exposing    */
/* the same ModelProvider interface as llama.cpp and mock providers.  */
/*                                                                    */
/* Prerequisite (browser):                                            */
/*   npm add onnxruntime-web                                          */
/*                                                                    */
/* The adapter is a scaffold: it constructs an InferenceSession         */
/* lazily and exposes generate(). Tests can mock onnxruntime-web.       */
/* ------------------------------------------------------------------ */

import { ModelProvider, Message } from "./model.ts";

export interface OnnxProviderOptions {
  modelPath?: string;
  temperature?: number;
  maxTokens?: number;
  executionProvider?: "webgl" | "wasm" | "cpu";
}

export class OnnxProvider implements ModelProvider {
  readonly name = "onnx";
  private modelPath: string;
  private temperature: number;
  private maxTokens: number;
  private executionProvider: "webgl" | "wasm" | "cpu";
  private ort: any = null;
  private session: any = null;

  constructor(opts: OnnxProviderOptions = {}) {
    this.modelPath = opts.modelPath ?? "";
    this.temperature = opts.temperature ?? 0.7;
    this.maxTokens = opts.maxTokens ?? 512;
    this.executionProvider = opts.executionProvider ?? "cpu";
  }

  private async loadOrt(): Promise<void> {
    if (this.ort) return;
    this.ort = await import("onnxruntime-web");
  }

  private async ensureSession(): Promise<void> {
    if (this.session) return;
    await this.loadOrt();
    this.session = await this.ort.InferenceSession.create(this.modelPath, {
      executionProviders: [this.executionProvider],
    });
  }

  async generate(
    messages: Message[],
    options?: {
      tools?: readonly string[];
      temperature?: number;
    },
  ): Promise<string> {
    await this.ensureSession();

    const temperature = options?.temperature ?? this.temperature;

    const input = new this.ort.Tensor("float32", new Float32Array([1]), [1]);
    const feeds = { input };
    const results = await this.session.run(feeds);
    const output = results.output as any;

    if (typeof output === "string") return output;
    const lastUser = messages
      .slice()
      .reverse()
      .find((m) => m.role === "user")?.content ?? "";
    return `ONNX response to "${lastUser}" (temperature=${temperature}, maxTokens=${this.maxTokens})`;
  }

  async isAvailable(): Promise<boolean> {
    try {
      await this.loadOrt();
      return true;
    } catch {
      return false;
    }
  }
}
