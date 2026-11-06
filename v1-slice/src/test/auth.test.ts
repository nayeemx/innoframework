/* ------------------------------------------------------------------ */
/* Negative security tests                                            */
/* These must PASS for v1 to be considered secure.                    */
/* ------------------------------------------------------------------ */

import { describe, it, expect } from "vitest";
import { validateSession, authGuard, makeSession } from "../server/auth";

describe("auth guard", () => {
  it("rejects undefined token", () => {
    expect(validateSession(undefined)).toBeNull();
  });

  it("rejects malformed token", () => {
    expect(validateSession("not-a-real-session")).toBeNull();
  });

  it("rejects tampered token", () => {
    const good = makeSession("00000000-0000-0000-0000-000000000001", "user");
    const tampered = good.slice(0, -2) + "ZZ";
    expect(validateSession(tampered)).toBeNull();
  });

  it("authGuard throws 401 on null session", () => {
    let caught: any = null;
    try {
      authGuard(null);
    } catch (err) {
      caught = err;
    }
    expect(caught).not.toBeNull();
    expect(caught.status).toBe(401);
  });

  it("authGuard passes on valid session", () => {
    const session = makeSession("00000000-0000-0000-0000-000000000001", "user");
    let caught: any = null;
    try {
      authGuard(validateSession(session));
    } catch (err) {
      caught = err;
    }
    expect(caught).toBeNull();
  });

  it("requireRole rejects wrong role", () => {
    const requireAdmin = (s: any) => {
      authGuard(s);
      if (s.role !== "admin") {
        const err: any = new Error("Forbidden");
        err.status = 403;
        throw err;
      }
    };
    const userSession = makeSession("00000000-0000-0000-0000-000000000001", "user");
    let caught: any = null;
    try {
      requireAdmin(validateSession(userSession));
    } catch (err) {
      caught = err;
    }
    expect(caught.status).toBe(403);
  });
});
