import { describe, it, expect, afterAll } from "vitest";
import { execSync } from "node:child_process";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const out = path.join(root, "smoke-app");

afterAll(() => {
  fs.rmSync(out, { recursive: true, force: true });
});

describe("create-v1-app scaffolder", () => {
  it("generates a complete app layout", () => {
    execSync(`node dist/create-v1-app.js smoke-app`, { cwd: root, stdio: "ignore" });

    const want = [
      "package.json",
      "tsconfig.json",
      "vite.config.ts",
      "vitest.config.ts",
      "index.html",
      "server/index.ts",
      "scripts/check-secrets.js",
      "src/main.tsx",
      "src/App.tsx",
      "src/lib/rpc.ts",
      "src/routes/index.ts",
    ];
    for (const rel of want) {
      expect(fs.existsSync(path.join(out, rel))).toBe(true);
    }
  });

  it("substitutes the app name into generated files", () => {
    const server = fs.readFileSync(path.join(out, "server/index.ts"), "utf-8");
    expect(server).toContain("smoke-app listening");
    expect(server).not.toContain("{{NAME}}");

    const html = fs.readFileSync(path.join(out, "index.html"), "utf-8");
    expect(html).toContain("<title>smoke-app</title>");

    const pkg = JSON.parse(
      fs.readFileSync(path.join(out, "package.json"), "utf-8"),
    );
    expect(pkg.name).toBe("smoke-app");
    expect(pkg.dependencies["@innologybd/core"]).toBeTruthy();
  });

  it("generated app consumes the package, not its own copy", () => {
    const rpc = fs.readFileSync(path.join(out, "src/lib/rpc.ts"), "utf-8");
    expect(rpc).toContain("@innologybd/core/rpc");
    expect(rpc).not.toContain("document.cookie.match");
  });
});