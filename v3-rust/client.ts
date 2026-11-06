import { z } from "zod";
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import {
  FibonacciInputSchema,
  FibonacciOutputSchema,
  type FibonacciInput,
  type FibonacciOutput,
  type NativeClientOptions,
  type NativeModule,
} from "./contract.ts";

const __dirname = dirname(fileURLToPath(import.meta.url));
const DEFAULT_WASM_PATH = join(__dirname, "pkg", "v3_rust_bg.wasm");

/*
 * Typed client over the wasm-bindgen-generated `pkg/` glue.
 *
 * The Rust crate exposes `fibonacci(n: i64) -> i64`; wasm-bindgen widens
 * both to `bigint`, so the client coerces back to `number` after validating
 * the input range. Loading uses `initSync` for a synchronous, deterministic
 * path (no fetch, works from disk in Node and from a bundled file in the
 * browser).
 */

export interface WasmBindgenModule {
  initSync(module: { module: BufferSource } | BufferSource): {
    memory: WebAssembly.Memory;
    fibonacci: (n: bigint) => bigint;
  };
}

function makeModule(opts: NativeClientOptions): Promise<NativeModule> {
  const wasmPath = opts.wasmPath ?? DEFAULT_WASM_PATH;
  let cache: Promise<NativeModule> | null = null;
  return (cache ??= (async () => {
    // Dynamic import keeps the wasm-bindgen glue out of the server bundle
    // until a client actually needs native acceleration.
    const mod: WasmBindgenModule = await import(
      "./pkg/v3_rust.js",
    ) as WasmBindgenModule;

    let bytes: BufferSource;
    try {
      bytes = readFileSync(wasmPath);
    } catch (err) {
      throw new Error(`WASM module unavailable: ${(err as Error).message}`);
    }

    const instance = mod.initSync({ module: bytes });
    const fibonacci = instance.fibonacci;
    if (typeof fibonacci !== "function") {
      throw new Error("WASM module missing required 'fibonacci' export");
    }
    return {
      fibonacci: (n: number) => Number(fibonacci(BigInt(n))),
    };
  })());
}

async function fibonacciNative(
  opts: NativeClientOptions,
  input: FibonacciInput,
): Promise<FibonacciOutput> {
  const parsedInput = FibonacciInputSchema.parse(input);
  const module = await makeModule(opts);
  const raw = module.fibonacci(parsedInput.n);
  const output: FibonacciOutput = { result: raw, n: parsedInput.n };
  return FibonacciOutputSchema.parse(output);
}

export function createNativeClient(opts: NativeClientOptions = {}) {
  return {
    fibonacciNative: (input: FibonacciInput): Promise<FibonacciOutput> =>
      fibonacciNative(opts, input),
  };
}

export const nativeClient = createNativeClient();
export { fibonacciNative };
export type { FibonacciInput, FibonacciOutput };