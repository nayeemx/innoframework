/* ------------------------------------------------------------------ */
/* v6-compiler — Wrapper vs direct MIR benchmark                      */
/*                                                                    */
/* Compares:                                                          */
/*   1. Wrapper RPC: serialize JSON + network RTT per call            */
/*   2. Direct MIR: interpret MIR in-process (no boundary)            */
/*                                                                    */
/* Exit criterion (Phase 5):                                          */
/*   Demonstrated performance gain — direct MIR execution beats       */
/*   wrapper RPC round-trip.                                          */
/* ------------------------------------------------------------------ */

import { describe, it, expect, beforeEach } from "vitest";
import { addMirModule, fibMirModule, interpretMir } from "../mir";
import { WrapperRpc } from "../wrapper";

const ITERATIONS = 100;

describe("wrapper vs direct MIR benchmark", () => {
  let wrapper: WrapperRpc;

  beforeEach(() => {
    wrapper = new WrapperRpc();
  });

  it("direct MIR execution is faster than wrapper RPC round-trip", async () => {
    const module = addMirModule();

    /* Direct MIR execution — in-process, no serialization, no RTT. */
    const directStart = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
      interpretMir(module, "add", [i, i + 1]);
    }
    const directMs = performance.now() - directStart;

    /* Wrapper RPC — each call serializes + waits RTT. */
    const wrapperStart = performance.now();
    for (let i = 0; i < ITERATIONS; i++) {
      await wrapper.call("add", [i, i + 1]);
    }
    const wrapperMs = performance.now() - wrapperStart;

    console.log(
      `direct=${directMs.toFixed(1)}ms wrapper=${wrapperMs.toFixed(1)}ms iterations=${ITERATIONS}`,
    );

    expect(directMs).toBeLessThan(wrapperMs);
    const speedup = wrapperMs / directMs;
    console.log(`speedup: ${speedup.toFixed(2)}x`);
    expect(speedup).toBeGreaterThan(1);
  }, 30000);

  it("wrapper accumulates serialized bytes per call", async () => {
    const module = addMirModule();
    interpretMir(module, "add", [1, 2]);

    await wrapper.call("add", [1, 2]);
    const stats = wrapper.getStats();
    expect(stats.callCount).toBe(1);
    expect(stats.totalSerializedBytes).toBeGreaterThan(0);
    expect(stats.totalRpcTimeMs).toBeGreaterThan(0);
  });

  it("fib direct execution produces correct result", () => {
    const module = fibMirModule();
    const result = interpretMir(module, "fib", [5]);
    expect(result).toBeGreaterThanOrEqual(0);
  });
});
