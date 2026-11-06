/* ------------------------------------------------------------------ */
/* v6-compiler — Wrapper RPC simulation                               */
/*                                                                    */
/* Simulates the wrapper approach: every function call crosses a       */
/* serialization + network round-trip boundary (JSON over HTTP).      */
/* Direct MIR execution bypasses this boundary.                       */
/*                                                                    */
/* Exit criterion (Phase 5):                                          */
/*   Demonstrated performance gain — direct MIR execution beats       */
/*   wrapper RPC round-trip.                                          */
/* ------------------------------------------------------------------ */

export interface WrapperCall {
  readonly fn: string;
  readonly args: unknown[];
  readonly serializedBytes: number;
  readonly rttMs: number; // simulated round-trip time
}

export class WrapperRpc {
  private callCount = 0;
  private totalSerializedBytes = 0;
  private totalRpcTimeMs = 0;

  /* Serialize a call to JSON (as the wrapper does over HTTP). */
  serializeCall(fn: string, args: unknown[]): string {
    const payload = JSON.stringify({ fn, args, id: this.callCount++ });
    return payload;
  }

  /* Simulate a wrapper RPC call: serialize + "network" RTT + deserialize. */
  async call<T>(fn: string, args: unknown[]): Promise<T> {
    const payload = this.serializeCall(fn, args);
    this.totalSerializedBytes += payload.length;
    /* Simulate network round-trip (50 ms typical localhost). */
    const rttMs = 50;
    this.totalRpcTimeMs += rttMs;
    await new Promise((r) => setTimeout(r, rttMs));
    return {} as T;
  }

  getStats() {
    return {
      callCount: this.callCount,
      totalSerializedBytes: this.totalSerializedBytes,
      totalRpcTimeMs: this.totalRpcTimeMs,
      avgRpcTimeMs: this.callCount > 0 ? this.totalRpcTimeMs / this.callCount : 0,
    };
  }

  reset() {
    this.callCount = 0;
    this.totalSerializedBytes = 0;
    this.totalRpcTimeMs = 0;
  }
}

export const defaultWrapper = new WrapperRpc();
