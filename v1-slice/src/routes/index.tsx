import { createServerFn } from "@/lib/rpc";

/* ------------------------------------------------------------------ */
/* Typed client stubs for server functions                            */
/* Each call hits POST /rpc/<name> on the v1 server.                  */
/* ------------------------------------------------------------------ */

export const getMessage = createServerFn<{}, { ok: boolean; text: string }>(
  "getMessage",
  {},
);

export const getProfile = createServerFn<{}, { ok: boolean; name: string; role: string }>(
  "getProfile",
  {},
);
