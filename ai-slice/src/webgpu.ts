/* ------------------------------------------------------------------ */
/* WebGPU adapter                                                     */
/* Exposes the ModelProvider interface backed by the browser WebGPU   */
/* API (navigator.gpu). This is a scaffold: it checks GPU availability */
/* and exposes generate(). Tests can mock navigator.gpu.               */
/* ------------------------------------------------------------------ */

import { ModelProvider, Message } from "./model.ts";

export interface WebGpuProviderOptions {
  modelName?: string;
  temperature?: number;
  maxTokens?: number;
}

export class WebGpuProvider implements ModelProvider {
  readonly name = "webgpu";
  private modelName: string;
  private temperature: number;
  private maxTokens: number;
  private device: any = null;

  constructor(opts: WebGpuProviderOptions = {}) {
    this.modelName = opts.modelName ?? "";
    this.temperature = opts.temperature ?? 0.7;
    this.maxTokens = opts.maxTokens ?? 512;
  }

  private async ensureDevice(): Promise<any> {
    if (this.device) return this.device;
    const navigatorGpu = (navigator as any).gpu;
    if (!navigatorGpu || !navigatorGpu.requestAdapter) {
      throw new Error("WebGPU is not available in this environment");
    }
    const adapter = await navigatorGpu.requestAdapter();
    if (!adapter) {
      throw new Error("No WebGPU adapter found");
    }
    this.device = await adapter.requestDevice();
    return this.device;
  }

  async generate(
    messages: Message[],
    options?: {
      tools?: readonly string[];
      temperature?: number;
    },
  ): Promise<string> {
    await this.ensureDevice();

    const temperature = options?.temperature ?? this.temperature;
    const prompt = messages.map((m) => `${m.role}: ${m.content}`).join("\n");

    /* Scaffold: WebGPU compute would encode the prompt into a GPU      */
    /* buffer and run a compute shader. Here we return a placeholder.   */
    return `WebGPU model="${this.modelName}" response to "${prompt.slice(0, 40)}..." (temperature=${temperature}, maxTokens=${this.maxTokens})`;
  }

  async isAvailable(): Promise<boolean> {
    try {
      await this.ensureDevice();
      return true;
    } catch {
      return false;
    }
  }
}
