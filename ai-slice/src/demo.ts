/* ------------------------------------------------------------------ */
/* Demo                                                               */
/* Runs the agent with a mock model and a weather tool.               */
/* ------------------------------------------------------------------ */

import { createAgent } from "./agent";
import { createTool, ToolRegistry } from "./tool";
import { MockModelProvider } from "./model";
import { z } from "zod";

const weatherTool = createTool({
  name: "getWeather",
  description: "Get current weather for a city",
  input: z.object({ city: z.string().min(1) }),
  output: z.object({ temperature: z.number(), condition: z.string() }),
  handler: async () => {
    return { temperature: 28, condition: "sunny" };
  },
});

const registry = new ToolRegistry();
registry.register(weatherTool);

const agent = createAgent({
  model: new MockModelProvider(),
  tools: [weatherTool],
  systemPromptTemplate: "You are a helpful assistant for {{userName}}.",
  systemPromptVars: { userName: "Developer" },
});

async function main() {
  const result = await agent.run("What's the weather in Dhaka?");
  console.log(JSON.stringify(result, null, 2));
}

main().catch(console.error);
