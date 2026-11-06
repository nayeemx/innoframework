/*
 * {{NAME}} — runtime auth integration test.
 *
 * Spawns the generated Express server as a child process and exercises the
 * /rpc/* auth guard against real HTTP requests. Confirms:
 *   - a valid session cookie grants 200 on /rpc/getMessage
 *   - no session → 401
 *   - a tampered session → 401
 *
 * Requires only Node built-ins (http, child_process) plus the app's
 * devDependencies (tsx, express, cookie-parser).
 */

import { describe, it, expect, afterAll, beforeAll } from "vitest";
import { spawn, ChildProcess } from "node:child_process";
import http from "node:http";
import { fileURLToPath } from "node:url";
import path from "node:path";

const PORT = process.env.AUTH_TEST_PORT || "3099";
const BASE = `http://localhost:${PORT}`;
const SERVER = path.join(path.dirname(fileURLToPath(import.meta.url)), "index.ts");
const TSX_CLI = path.join(
  path.dirname(SERVER),
  "..",
  "node_modules",
  "tsx",
  "dist",
  "cli.mjs",
);

let server: ChildProcess | null = null;

function request(method: string, url: string, headers: Record<string, string> = {}): Promise<{ status: number; body: string; headers: http.IncomingHttpHeaders }> {
  return new Promise((resolve, reject) => {
    const u = new URL(url);
    const opts = {
      hostname: u.hostname,
      port: u.port,
      path: u.pathname,
      method,
      headers,
    };
    const req = http.request(opts, (res) => {
      let data = "";
      res.on("data", (c) => (data += c));
      res.on("end", () => resolve({ status: res.statusCode ?? 0, body: data, headers: res.headers }));
    });
    req.on("error", reject);
    req.end();
  });
}

function waitForHealth(maxMs = 15000): Promise<void> {
  const start = Date.now();
  return new Promise((resolve, reject) => {
    const poll = () => {
      request("GET", `${BASE}/health`)
        .then(() => resolve())
        .catch(() => {
          if (Date.now() - start > maxMs) reject(new Error("server did not become healthy"));
          else setTimeout(poll, 200);
        });
    };
    poll();
  });
}

describe("runtime auth — /rpc/* guard", () => {
  beforeAll(async () => {
    server = spawn("node", [TSX_CLI, SERVER], {
      env: { ...process.env, PORT },
      stdio: ["ignore", "pipe", "pipe"],
      cwd: path.dirname(SERVER),
    });
    await waitForHealth();
  });

  afterAll(() => {
    if (server) server.kill("SIGTERM");
    server = null;
  });

  it("rejects unauthenticated /rpc/getMessage with 401", async () => {
    const res = await request("POST", `${BASE}/rpc/getMessage`);
    expect(res.status).toBe(401);
  });

  it("issues a session via /login", async () => {
    const res = await request("POST", `${BASE}/login`);
    expect(res.status).toBe(200);
    expect(res.body).toContain("ok");
  });

  it("grants 200 on /rpc/getMessage with a valid session cookie", async () => {
    const login = await request("POST", `${BASE}/login`);
    const cookie = login.headers["set-cookie"] ?? "";
    const setCookie = Array.isArray(cookie) ? cookie[0] : cookie;
    const res = await request("POST", `${BASE}/rpc/getMessage`, {
      Cookie: setCookie.split(";")[0],
    });
    expect(res.status).toBe(200);
    expect(res.body).toContain("Hello");
  });

  it("rejects a tampered session cookie with 401", async () => {
    const res = await request("POST", `${BASE}/rpc/getMessage`, {
      Cookie: "v1_session=tampered.token",
    });
    expect(res.status).toBe(401);
  });
});
