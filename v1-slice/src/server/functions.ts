/* ------------------------------------------------------------------ */
/* Server function contract                                           */
/* These names must match the Express endpoints in server/index.js.   */
/* The client stubs live in src/lib/rpc.ts and src/routes/index.tsx.  */
/* ------------------------------------------------------------------ */

export const SERVER_FUNCTIONS = {
  getMessage: { input: {}, output: { ok: boolean; text: string } },
  getProfile: { input: {}, output: { ok: boolean; name: string; role: string } },
} as const;

export type ServerFnName = keyof typeof SERVER_FUNCTIONS;

/* ------------------------------------------------------------------ */
/* Zod schemas for validating server function input                     */
/* Every state-changing server function must validate input.          */
/* ------------------------------------------------------------------ */

import { z } from "zod";

export const GetMessageInput = z.object({});
export const GetProfileInput = z.object({});

