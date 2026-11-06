/*
 * @framework/core — auth.
 *
 * Signed session tokens.
 *
 *   token = <base64url(payload)>.<base64url(hmac-sha256(payload, secret))>
 *   payload = { userId, role, exp }
 *
 * Verification uses `timingSafeEqual` (constant-time) and rejects expired
 * sessions. Set `SESSION_SECRET` in production; the dev fallback is
 * documented and intentionally insecure.
 */

import { z } from "zod";
import { createHmac, timingSafeEqual } from "node:crypto";

export const SessionSchema = z.object({
  userId: z.string().uuid(),
  role: z.enum(["user", "admin"]),
  exp: z.number(),
});

export type Session = z.infer<typeof SessionSchema>;

/** Dev-only fallback. Set SESSION_SECRET before any production use. */
export const DEV_SECRET = "v1-dev-only-insecure-session-key-change-me";

function getSecret(): string {
  return process.env.SESSION_SECRET ?? DEV_SECRET;
}

function sign(payloadB64: string): string {
  return createHmac("sha256", getSecret()).update(payloadB64).digest("base64url");
}

function safeEqual(a: string, b: string): boolean {
  const bufA = Buffer.from(a);
  const bufB = Buffer.from(b);
  if (bufA.length !== bufB.length) return false;
  return timingSafeEqual(bufA, bufB);
}

/**
 * Issue a signed session token.
 * @param ttlMs defaults to 24 hours.
 */
export function makeSession(
  userId: string,
  role: Session["role"] = "user",
  ttlMs: number = 1000 * 60 * 60 * 24,
): string {
  const payload = { userId, role, exp: Date.now() + ttlMs };
  const payloadB64 = Buffer.from(JSON.stringify(payload)).toString("base64url");
  return `${payloadB64}.${sign(payloadB64)}`;
}

/** Validate + verify a token. Returns the session or null. */
export function validateSession(token: string | undefined): Session | null {
  if (!token) return null;
  const parts = token.split(".");
  if (parts.length !== 2) return null;
  const [payloadB64, signature] = parts;
  if (!payloadB64 || !signature) return null;

  if (!safeEqual(signature, sign(payloadB64))) return null;

  try {
    const payload = JSON.parse(Buffer.from(payloadB64, "base64url").toString("utf-8"));
    const session = SessionSchema.parse(payload);
    if (session.exp <= Date.now()) return null;
    return session;
  } catch {
    return null;
  }
}

/** Fail closed: throws 401 when there is no session. */
export function authGuard(session: Session | null): void {
  if (!session) {
    const err: any = new Error("Unauthenticated");
    err.status = 401;
    throw err;
  }
}

/** Fail closed + role check: throws 401 then 403. */
export function requireRole(role: Session["role"]) {
  return (session: Session | null): void => {
    authGuard(session);
    if (!session || session.role !== role) {
      const err: any = new Error("Forbidden");
      err.status = 403;
      throw err;
    }
  };
}