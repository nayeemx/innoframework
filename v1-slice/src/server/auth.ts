import { z } from "zod";

/* ------------------------------------------------------------------ */
/* Session validation                                                 */
/* In v1 this is a minimal auth guard. A real deployment swaps this   */
/* for a database-backed session store.                               */
/* ------------------------------------------------------------------ */

export const SessionSchema = z.object({
  userId: z.string().uuid(),
  role: z.enum(["user", "admin"]),
  exp: z.number(),
});

export type Session = z.infer<typeof SessionSchema>;

export function validateSession(token: string | undefined): Session | null {
  if (!token) return null;
  try {
    const payload = JSON.parse(Buffer.from(token, "base64url").toString("utf-8"));
    return SessionSchema.parse(payload);
  } catch {
    return null;
  }
}

export function makeSession(userId: string, role: Session["role"] = "user"): string {
  const payload = { userId, role, exp: Date.now() + 1000 * 60 * 60 * 24 };
  return Buffer.from(JSON.stringify(payload)).toString("base64url");
}

export function authGuard(session: Session | null): void {
  if (!session) {
    const err: any = new Error("Unauthenticated");
    err.status = 401;
    throw err;
  }
}

export function requireRole(role: Session["role"]) {
  return (session: Session | null): void => {
    authGuard(session);
    if (session.role !== role) {
      const err: any = new Error("Forbidden");
      err.status = 403;
      throw err;
    }
  };
}
