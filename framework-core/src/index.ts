/*
 * @innologybd/core — public entry point.
 *
 * Re-exports the pieces every v1 app needs. Install with:
 *   npm i @innologybd/core
 * then `import { createServerFn, authGuard, rpcGuard } from "@innologybd/core"`.
 */

export {
  createServerFn,
  rpcGuard,
  type ServerFnCall,
  type ServerFnDescriptor,
} from "./rpc.js";

export {
  SessionSchema,
  type Session,
  validateSession,
  makeSession,
  authGuard,
  requireRole,
  DEV_SECRET,
} from "./auth.js";

export { DEFAULT_CSP, securityHeaders, type CspOptions } from "./csp.js";