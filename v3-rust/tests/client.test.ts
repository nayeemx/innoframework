import { describe, it, expect } from "vitest";
import { nativeClient, createNativeClient } from "../client.ts";
import { FibonacciInputSchema } from "../contract.ts";

describe("typed native client boundary", () => {
  it("validates input: rejects negative n", async () => {
    await expect(nativeClient.fibonacciNative({ n: -1 })).rejects.toThrow();
  });

  it("validates input: rejects non-integer n", async () => {
    await expect(nativeClient.fibonacciNative({ n: 2.5 } as any)).rejects.toThrow();
  });

  it("validates input: rejects n above max", async () => {
    await expect(nativeClient.fibonacciNative({ n: 99999999 })).rejects.toThrow();
  });

  it("validates input: rejects missing n", async () => {
    await expect(nativeClient.fibonacciNative({} as any)).rejects.toThrow();
  });

  it("returns validated output for a known input", async () => {
    const result = await nativeClient.fibonacciNative({ n: 10 });
    expect(FibonacciInputSchema.parse({ n: 10 })).toEqual({ n: 10 });
    expect(result.n).toBe(10);
    expect(result.result).toBe(55);
    expect(Number.isInteger(result.result)).toBe(true);
  });

  it("returns validated output for n=0 and n=1", async () => {
    expect((await nativeClient.fibonacciNative({ n: 0 })).result).toBe(0);
    expect((await nativeClient.fibonacciNative({ n: 1 })).result).toBe(1);
  });

  it("fails closed with a clear error when the WASM module is unavailable", async () => {
    const client = createNativeClient({ wasmPath: "./pkg/does-not-exist.wasm" });
    await expect(client.fibonacciNative({ n: 5 })).rejects.toThrow(/WASM module unavailable/);
  });
});