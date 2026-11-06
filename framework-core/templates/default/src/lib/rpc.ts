/*
 * {{NAME}} — RPC client.
 *
 * Re-exports the typed server-function stub from @innologybd/core so the
 * app never carries its own copy of the fetch wrapper.
 */

export { createServerFn, type ServerFnCall } from "@innologybd/core/rpc";