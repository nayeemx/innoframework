/* ------------------------------------------------------------------ */
/* Tool contract                                                      */
/* A tool is a named, typed function with zod input/output schemas.   */
/* The framework owns the registry; agents can only call registered   */
/* tools.                                                             */
/* ------------------------------------------------------------------ */

import { type ZodSchema } from "zod";

export interface Tool<Input, Output> {
  readonly name: string;
  readonly description: string;
  readonly inputSchema: ZodSchema<Input>;
  readonly outputSchema: ZodSchema<Output>;
  readonly handler: (input: Input) => Promise<Output> | Output;
}

export interface ToolCall<Input> {
  readonly toolName: string;
  readonly input: Input;
}

export interface ToolResult<Output> {
  readonly toolName: string;
  readonly ok: boolean;
  readonly output?: Output;
  readonly error?: string;
}

/* ------------------------------------------------------------------ */
/* createTool                                                         */
/* Declares a tool with input/output validation.                      */
/* ------------------------------------------------------------------ */

export function createTool<Input, Output>(
  def: {
    name: string;
    description: string;
    input: ZodSchema<Input>;
    output: ZodSchema<Output>;
    handler: (input: Input) => Promise<Output> | Output;
  },
): Tool<Input, Output> {
  return {
    name: def.name,
    description: def.description,
    inputSchema: def.input,
    outputSchema: def.output,
    handler: def.handler,
  };
}

/* ------------------------------------------------------------------ */
/* Tool registry                                                      */
/* Allowlist-only: agents can only call tools that are registered.    */
/* ------------------------------------------------------------------ */

export class ToolRegistry {
  private tools = new Map<string, Tool<any, any>>();

  register(tool: Tool<any, any>): void {
    if (this.tools.has(tool.name)) {
      throw new Error(`Tool already registered: ${tool.name}`);
    }
    this.tools.set(tool.name, tool);
  }

  get(name: string): Tool<any, any> | undefined {
    return this.tools.get(name);
  }

  has(name: string): boolean {
    return this.tools.has(name);
  }

  list(): string[] {
    return Array.from(this.tools.keys());
  }

  async call<Input, Output>(
    toolCall: ToolCall<Input>,
  ): Promise<ToolResult<Output>> {
    const tool = this.tools.get(toolCall.toolName);
    if (!tool) {
      return {
        toolName: toolCall.toolName,
        ok: false,
        error: `Tool not registered: ${toolCall.toolName}`,
      };
    }

    /* Validate input */
    let input: Input;
    try {
      input = tool.inputSchema.parse(toolCall.input) as Input;
    } catch (err: any) {
      return {
        toolName: toolCall.toolName,
        ok: false,
        error: `Invalid input: ${err.message}`,
      };
    }

    /* Run handler */
    let output: Output;
    try {
      output = (await tool.handler(input)) as Output;
    } catch (err: any) {
      return {
        toolName: toolCall.toolName,
        ok: false,
        error: `Handler error: ${err.message}`,
      };
    }

    /* Validate output */
    let validated: Output;
    try {
      validated = tool.outputSchema.parse(output) as Output;
    } catch (err: any) {
      return {
        toolName: toolCall.toolName,
        ok: false,
        error: `Invalid output: ${err.message}`,
      };
    }

    return { toolName: toolCall.toolName, ok: true, output: validated };
  }
}

/* ------------------------------------------------------------------ */
/* Default empty registry                                             */
/* ------------------------------------------------------------------ */

export const defaultRegistry = new ToolRegistry();
