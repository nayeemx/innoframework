import { invoke } from "@tauri-apps/api/core";

export interface ServerFnCall<Input, Output> {
  call: (input?: Input) => Promise<Output>;
}

export function createServerFn<Input = Record<string, never>, Output = unknown>(
  name: string,
  _defaultInput: Input,
): ServerFnCall<Input, Output> {
  return {
    call: async (input?: Input): Promise<Output> => {
      return (await invoke<Output>(name, { input: input ?? {} })) as Output;
    },
  };
}

export const getMessage = createServerFn<Record<string, never>, { ok: boolean; text: string }>("get_message", {});
export const getProfile = createServerFn<Record<string, never>, { ok: boolean; name: string; role: string }>("get_profile", {});
