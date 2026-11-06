/* ------------------------------------------------------------------ */
/* Model provider interface                                           */
/* The agent talks to a model through this interface. Providers can   */
/* be swapped (Gemini, OpenAI, Ollama) without changing the agent.    */
/* ------------------------------------------------------------------ */

export interface Message {
  readonly role: "system" | "user" | "assistant" | "tool";
  readonly content: string;
}

export interface ModelProvider {
  readonly name: string;
  generate(messages: Message[], options?: {
    tools?: readonly string[];
    temperature?: number;
  }): Promise<string>;
}

/* ------------------------------------------------------------------ */
/* Mock model provider (for testing and demos without API keys)       */
/* ------------------------------------------------------------------ */

export class MockModelProvider implements ModelProvider {
  readonly name = "mock";

  async generate(
    messages: Message[],
    options?: { tools?: readonly string[]; temperature?: number },
  ): Promise<string> {
    const lastUser = [...messages]
      .reverse()
      .find((m) => m.role === "user");
    const userContent = lastUser?.content ?? "";

    /* If a tool result already exists in the conversation, give a       */
    /* final answer instead of calling the tool again.                   */
    const hasToolResult = messages.some((m) => m.role === "tool");

    const wantsTool =
      (options?.tools?.includes("getWeather") ?? false) &&
      /weather|temperature|forecast/i.test(userContent) &&
      !hasToolResult;

    if (wantsTool) {
      const cityMatch = userContent.match(/in\s+([A-Za-z\s]+)/i);
      const city = cityMatch ? cityMatch[1].trim() : "Dhaka";
      return JSON.stringify({
        tool: "getWeather",
        input: { city },
      });
    }

    return `I received your message: "${userContent}". (Mock provider — no real model called.)`;
  }
}
