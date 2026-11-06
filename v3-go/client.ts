import { z } from "zod";
import {
  FibonacciInputSchema,
  FibonacciOutputSchema,
  type FibonacciInput,
  type FibonacciOutput,
  BASE_URL,
} from "./contract.ts";

export async function fibonacciHttp(input: FibonacciInput): Promise<FibonacciOutput> {
  const parsed = FibonacciInputSchema.parse(input);
  const res = await fetch(`${BASE_URL}/fibonacci`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ n: parsed.n }),
  });
  if (!res.ok) {
    throw new Error(`Go service error: ${res.status} ${res.statusText}`);
  }
  const body = await res.json();
  return FibonacciOutputSchema.parse(body);
}

export async function health(): Promise<{ ok: boolean }> {
  const res = await fetch(`${BASE_URL}/health`);
  if (!res.ok) {
    throw new Error(`Go service error: ${res.status} ${res.statusText}`);
  }
  return res.json();
}

export { fibonacciHttp as fibonacci };