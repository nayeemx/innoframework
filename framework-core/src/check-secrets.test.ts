import { describe, it, expect, afterAll } from "vitest";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { execSync } from "child_process";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const dist = path.join(root, "dist");
const checkScript = path.join(dist, "check-secrets.js");

const tmp = path.join(root, "check-secrets-test-tmp");

function write(fileRel: string, content: string): void {
  const full = path.join(tmp, fileRel);
  fs.mkdirSync(path.dirname(full), { recursive: true });
  fs.writeFileSync(full, content);
}

afterAll(() => {
  fs.rmSync(tmp, { recursive: true, force: true });
});

function run(target: string): { status: number; stdout: string; stderr: string } {
  const result = execSync(`node "${checkScript}" "${target}"`, {
    encoding: "utf-8",
  });
  return { status: 0, stdout: result, stderr: "" };
}

function runExpectFail(target: string): { status: number; stderr: string } {
  try {
    execSync(`node "${checkScript}" "${target}"`, {
      encoding: "utf-8",
    });
    return { status: 0, stderr: "" };
  } catch (err: any) {
    return {
      status: err.status ?? 1,
      stderr: err.stderr ?? err.stdout ?? "",
    };
  }
}

describe("check-secrets", () => {
  it("passes when no secrets are present", () => {
    write("app.ts", "const x = 1;");
    const r = run(tmp);
    expect(r.stdout).toContain("passed");
  });

  it("fails on process.env.SERVER_API_KEY", () => {
    write("app.ts", "const k = process.env.SERVER_API_KEY;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on import.meta.env.SERVER_DB_URL", () => {
    write("app.ts", "const k = import.meta.env.SERVER_DB_URL;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on SECRET_PASSWORD", () => {
    write("app.ts", "const k = SECRET_PASSWORD;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on PRIVATE_KEY", () => {
    write("app.ts", "const k = PRIVATE_KEY;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on process.env.GEMINI_API_KEY", () => {
    write("app.ts", "const k = process.env.GEMINI_API_KEY;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on process.env.OPENAI_API_KEY", () => {
    write("app.ts", "const k = process.env.OPENAI_API_KEY;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on process.env.ANTHROPIC_API_KEY", () => {
    write("app.ts", "const k = process.env.ANTHROPIC_API_KEY;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on import.meta.env.GEMINI_API_KEY", () => {
    write("app.ts", "const k = import.meta.env.GEMINI_API_KEY;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on import.meta.env.OPENAI_API_KEY", () => {
    write("app.ts", "const k = import.meta.env.OPENAI_API_KEY;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on import.meta.env.ANTHROPIC_API_KEY", () => {
    write("app.ts", "const k = import.meta.env.ANTHROPIC_API_KEY;");
    const r = runExpectFail(tmp);
    expect(r.status).not.toBe(0);
  });

  it("fails on DATABASE_URL and JWT_SECRET", () => {
    write("app.ts", "const k = process.env.DATABASE_URL;");
    const r1 = runExpectFail(tmp);
    expect(r1.status).not.toBe(0);
    write("app.ts", "const k = process.env.JWT_SECRET;");
    const r2 = runExpectFail(tmp);
    expect(r2.status).not.toBe(0);
  });
});
