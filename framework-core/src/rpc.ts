/*
 * @framework/core — typed server functions (client stub).
 *
 * A server function is declared once with its input/output shapes; the client
 * call looks like a local function but POSTs to /rpc/<name>. The cookie is
 * forwarded automatically so the v1 auth guard applies.
 *
 *   import { createServerFn } from "@framework/core/rpc";
 *   export const getMessage = createServerFn<{}, { ok: boolean; text: string }>("getMessage", {});
 *   const { text } = await getMessage.call();
 */

const RPC_BASE = (import.meta as any).env?.RPC_BASE ?? ((import.meta as any).env?.DEV ? "http://localhost:3001" : "");

export interface ServerFnCall<Input, Output> {
  call: (input?: Input) => Promise<Output>;
}

/** Shape of a server function declaration. */
export interface ServerFnDescriptor<Input, Output> {
  name: string;
  defaultInput: Input;
  _output?: Output;
}

/**
 * Declare a typed server function.
 * @param name the /rpc/<name> endpoint
 * @param defaultInput used for typing; never sent unless the caller omits input
 */
export function createServerFn<Input = Record<string, never>, Output = unknown>(
  name: string,
  defaultInput: Input,
): ServerFnCall<Input, Output> {
  return {
    call: async (input?: Input): Promise<Output> => {
      const token =
        typeof document !== "undefined"
          ? document.cookie.match(/v1_session=([^;]+)/)?.[1]
          : undefined;
      const res = await fetch(`${RPC_BASE}/rpc/${name}`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
          ...(token ? { Cookie: `v1_session=${token}` } : {}),
        },
        body: JSON.stringify(input ?? defaultInput),
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

/**
 * Express middleware: validate the session and attach it to `req.session`.
 * Pass the auth helpers from `@framework/core/auth` so the server bundle
 * never pulls in client-only code.
 *
 *   import { rpcGuard } from "@framework/core/rpc";
 *   import { validateSession, authGuard } from "@framework/core/auth";
 *   app.post("/rpc/getMessage", rpcGuard(validateSession, authGuard), ...);
 */
export function rpcGuard(
  validate: (token: string | undefined) => any,
  guard: (session: any) => void,
) {
  return (req: any, res: any, next: () => void): void => {
    const token = req.cookies?.v1_session;
    const session = validate(token);
    try {
      guard(session);
      req.session = session;
      next();
    } catch (err: any) {
      res.status(err.status ?? 401).json({ error: err.message ?? "Unauthenticated" });
    }
  };
}