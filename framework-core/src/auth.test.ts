import { describe, it, expect, beforeEach, afterEach } from "vitest";
import {
  SessionSchema,
  validateSession,
  makeSession,
  authGuard,
  requireRole,
} from "./auth.js";

describe("auth — signed tokens", () => {
  const originalSecret = process.env.SESSION_SECRET;

  beforeEach(() => {
    process.env.SESSION_SECRET = "test-secret-key";
  });
  afterEach(() => {
    if (originalSecret === undefined) delete process.env.SESSION_SECRET;
    else process.env.SESSION_SECRET = originalSecret;
  });

  it("round-trips a signed token", () => {
    const token = makeSession("11111111-1111-1111-1111-111111111111", "admin");
    const session = validateSession(token);
    expect(session).not.toBeNull();
    expect(session?.role).toBe("admin");
  });

  it("rejects an unsigned base64 token", () => {
    const unsigned = Buffer.from(
      JSON.stringify({
        userId: "11111111-1111-1111-1111-111111111111",
        role: "admin",
        exp: Date.now() + 60_000,
      }),
    ).toString("base64url");
    expect(validateSession(unsigned)).toBeNull();
  });

  it("rejects a tampered signature", () => {
    const good = makeSession("11111111-1111-1111-1111-111111111111", "user");
    const tampered = good.slice(0, -2) + "ZZ";
    expect(validateSession(tampered)).toBeNull();
  });

  it("rejects a payload signed with the wrong key", () => {
    const good = makeSession("11111111-1111-1111-1111-111111111111", "user");
    const forgedPayload = Buffer.from(
      JSON.stringify({
        userId: "11111111-1111-1111-1111-111111111111",
        role: "admin",
        exp: Date.now() + 60_000,
      }),
    ).toString("base64url");
    const forged = `${forgedPayload}.${good.split(".")[1]}`;
    expect(validateSession(forged)).toBeNull();
  });

  it("rejects an expired token", () => {
    const expired = makeSession(
      "11111111-1111-1111-1111-111111111111",
      "user",
      -1000,
    );
    expect(validateSession(expired)).toBeNull();
  });

  it("rejects undefined and malformed tokens", () => {
    expect(validateSession(undefined)).toBeNull();
    expect(validateSession("not-a-token")).toBeNull();
    expect(validateSession("a.b.c")).toBeNull();
  });

  it("authGuard throws 401 on null", () => {
    let caught: any = null;
    try {
      authGuard(null);
    } catch (err) {
      caught = err;
    }
    expect(caught).not.toBeNull();
    expect(caught.status).toBe(401);
  });

  it("authGuard passes on a valid session", () => {
    const session = validateSession(
      makeSession("11111111-1111-1111-1111-111111111111", "user"),
    );
    expect(() => authGuard(session)).not.toThrow();
  });

  it("requireRole throws 403 for the wrong role", () => {
    const userSession = validateSession(
      makeSession("11111111-1111-1111-1111-111111111111", "user"),
    );
    let caught: any = null;
    try {
      requireRole("admin")(userSession);
    } catch (err) {
      caught = err;
    }
    expect(caught.status).toBe(403);
  });

  it("SessionSchema rejects an invalid role", () => {
    expect(() =>
      SessionSchema.parse({ userId: "nope", role: "superadmin", exp: 0 }),
    ).toThrow();
  });
});