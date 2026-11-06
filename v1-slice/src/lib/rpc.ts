/* ------------------------------------------------------------------ */
/* RPC client stub                                                    */
/* This is the client-side representation of a server function call.  */
/* In production the build step replaces .call() with a fetch to the  */
/* server function endpoint. Here it is a real fetch for the slice.   */
/* ------------------------------------------------------------------ */

const RPC_BASE = import.meta.env.DEV ? "http://localhost:3001" : "";

export interface ServerFnCall<Input, Output> {
  call: (input?: Input) => Promise<Output>;
}

export function createServerFn<Input = Record<string, never>, Output = unknown>(
  name: string,
  _defaultInput: Input,
): ServerFnCall<Input, Output> {
  return {
    call: async (input?: Input): Promise<Output> => {
      const token = typeof document !== "undefined" ? document.cookie.match(/v1_session=([^;]+)/)?.[1] : undefined;
      const res = await fetch(`${RPC_BASE}/rpc/${name}`, {
        method: "POST",
        headers: { "Content-Type": "application/json", ...(token ? { Cookie: `v1_session=${token}` } : {}) },
        body: JSON.stringify(input ?? {}),
        credentials: "include",
      });
      if (!res.ok) {
        const body = await res.text().catch(() => "");
        throw new Error(`${res.status} ${res.statusText}${body ? ": " + body : ""}`);
      }
      return (await res.json()) as Output;
    },
  };
}

declare global {
  // Injected by the build-time RPC transform in production builds.
  // In dev we use the real fetch implementation above.
  var __rpcCall: (name: string, input: unknown) => Promise<unknown> | undefined;
}
