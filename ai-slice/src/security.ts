/* ------------------------------------------------------------------ */
/* Prompt injection defenses                                            */
/* System prompts use variable substitution; user input is never      */
/* concatenated into system instructions.                             */
/* ------------------------------------------------------------------ */

/* Template placeholders are {{name}} style. Only whitelisted keys    */
export function renderSystemPrompt(
  template: string,
  vars: Record<string, string>,
): string {
  const allowedKeys = Object.keys(vars);
  let result = template;

  for (const key of allowedKeys) {
    const value = vars[key].replace(/[{}]/g, ""); /* strip braces from value */
    result = result.replace(new RegExp(`{{\\s*${key}\\s*}}`, "g"), value);
  }

  /* Strip any remaining unreplaced placeholders to avoid leaking      */
  /* template structure into the prompt.                               */
  result = result.replace(/\{\{[^}]*\}\}/g, "");

  return result;
}

/* Sanitize agent text output before rendering in UI.                 */
export function sanitizeAgentOutput(text: string): string {
  return text
    .replace(/<script\b[^<]*(?:(?!<\/script>)<[^<]*)*<\/script>/gi, "")
    .replace(/javascript:/gi, "")
    .replace(/on\w+\s*=/gi, "");
}

/* Detect obvious prompt injection attempts in user input.            */
export function detectPromptInjection(input: string): string[] {
  const flags: string[] = [];
  const lower = input.toLowerCase();

  const patterns = [
    ["ignore previous instructions", "ignore-previous"],
    ["disregard", "disregard-instructions"],
    ["system prompt", "leak-system-prompt"],
    ["you are now", "role-redefinition"],
    ["admin mode", "admin-mode"],
    ["execute", "execute-command"],
    ["eval(", "eval-call"],
  ];

  for (const [pattern, flag] of patterns) {
    if (lower.includes(pattern)) flags.push(flag);
  }

  return flags;
}

/* ------------------------------------------------------------------ */
/* Output validation                                                  */
/* ------------------------------------------------------------------ */

export function validateAgentOutput(
  text: string,
  maxLength: number = 4000,
): { ok: boolean; error?: string } {
  if (text.length > maxLength) {
    return { ok: false, error: "Output too long" };
  }
  return { ok: true };
}
