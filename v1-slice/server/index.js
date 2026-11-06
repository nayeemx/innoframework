/* ------------------------------------------------------------------ */
/* v1 Express server                                                  */
/* Provides:                                                          */
/*   - CSP headers on every response                                  */
/*   - Auth guard on /rpc/* (server functions)                        */
/*   - Server function endpoints                                       */
/* ------------------------------------------------------------------ */

import express from "express";
import cookieParser from "cookie-parser";
import jwt from "jsonwebtoken";
import { validateSession, authGuard, makeSession, Session } from "../src/server/auth";
import { ServerFns } from "../src/server/functions";

const app = express();
const PORT = 3001;

app.use(cookieParser());
app.use(express.json());

/* ------------------------------------------------------------------ */
/* 1. CSP header preset (security checklist CSP-1 to CSP-10)          */
/* ------------------------------------------------------------------ */
const CSP =
  "default-src 'self'; " +
  "script-src 'self'; " +
  "style-src 'self' 'unsafe-inline'; " +
  "img-src 'self' data:; " +
  "connect-src 'self'; " +
  "frame-ancestors 'none'; " +
  "base-uri 'self'; " +
  "object-src 'none'; " +
  "form-action 'self';";

app.use((_req, res, next) => {
  res.setHeader("Content-Security-Policy", CSP);
  res.setHeader("X-Frame-Options", "DENY");
  res.setHeader("X-Content-Type-Options", "nosniff");
  res.setHeader("Referrer-Policy", "strict-origin-when-cross-origin");
  res.setHeader("Strict-Transport-Security", "max-age=31536000; includeSubDomains; preload");
  next();
});

/* ------------------------------------------------------------------ */
/* 2. Auth guard middleware                                           */
/*   - /rpc/* requires a valid v1_session cookie                      */
/*   - /login sets a session cookie (dev/demo only)                   */
/* ------------------------------------------------------------------ */

function requireAuth(req: express.Request, _res: express.Response, next: express.NextFunction): void {
  const token = req.cookies?.v1_session;
  const session = validateSession(token);
  try {
    authGuard(session);
    (req as any).session = session;
    next();
  } catch (err: any) {
    res.status(err.status ?? 401).json({ error: err.message ?? "Unauthenticated" });
  }
}

/* Dev-only login endpoint to obtain a session cookie for testing */
app.post("/login", (_req, res) => {
  const session = makeSession("00000000-0000-0000-0000-000000000001", "user");
  res.cookie("v1_session", session, {
    httpOnly: true,
    secure: false, // set true in production (HTTPS only)
    sameSite: "lax",
    maxAge: 1000 * 60 * 60 * 24,
  });
  res.json({ ok: true });
});

app.post("/logout", (_req, res) => {
  res.clearCookie("v1_session");
  res.json({ ok: true });
});

/* ------------------------------------------------------------------ */
/* 3. Server function endpoints                                       */
/*   - /rpc/:name  POST  -> invokes the matching server function      */
/*   - Each function is guarded by requireAuth                        */
/* ------------------------------------------------------------------ */

app.post("/rpc/getMessage", requireAuth, (_req, res) => {
  res.json({ ok: true, text: "Hello from the v1 server function." });
});

app.post("/rpc/getProfile", requireAuth, (_req, res) => {
  res.json({ ok: true, name: "v1 Developer", role: "framework engineer" });
});

/* Unknown /rpc/* -> 404 */
app.use("/rpc/*", (_req, res) => {
  res.status(404).json({ error: "Server function not found" });
});

/* Health check (no auth required) */
app.get("/health", (_req, res) => {
  res.json({ ok: true });
});

app.listen(PORT, () => {
  console.log(`v1 slice server listening on http://localhost:${PORT}`);
});
