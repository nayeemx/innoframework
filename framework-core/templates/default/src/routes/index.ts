import { createServerFn } from "@/lib/rpc";

export const getMessage = createServerFn<{}, { ok: boolean; text: string }>(
  "getMessage", {},
);

export const getProfile = createServerFn<
  {},
  { ok: boolean; name: string; role: string }
>("getProfile", {});