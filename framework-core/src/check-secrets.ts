/*
 * @framework/core — build-time secret check.
 *
 * Fails the build if server-only env/secrets are referenced in client
 * source files. Run as `node dist/check-secrets.js <clientDir>`.
 *
 *   "check:secrets": "node dist/check-secrets.js src"
 */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const SERVER_PREFIXES = [
  "process.env.SERVER_",
  "import.meta.env.SERVER_",
  "SECRET_",
  "PRIVATE_",
  "process.env.DATABASE_URL",
  "process.env.JWT_SECRET",
  "process.env.GEMINI_API_KEY",
  "process.env.OPENAI_API_KEY",
  "process.env.ANTHROPIC_API_KEY",
  "import.meta.env.GEMINI_API_KEY",
  "import.meta.env.OPENAI_API_KEY",
  "import.meta.env.ANTHROPIC_API_KEY",
];
const EXCLUDE = ["node_modules", "dist", ".git", "test", "__tests__"];

function walk(dir: string, secrets: string[]): void {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (!EXCLUDE.includes(entry.name)) walk(full, secrets);
    } else if (entry.isFile() && /\.(ts|tsx|js|jsx)$/.test(entry.name)) {
      const content = fs.readFileSync(full, "utf-8");
      for (const prefix of SERVER_PREFIXES) {
        if (content.includes(prefix)) {
          secrets.push(`${full}: contains "${prefix}"`);
        }
      }
    }
  }
}

const target = process.argv[2];
if (!target) {
  console.error("usage: node dist/check-secrets.js <clientDir>");
  process.exit(2);
}

const clientDir = path.resolve(target);
if (!fs.existsSync(clientDir)) {
  console.error(`directory not found: ${clientDir}`);
  process.exit(2);
}

const secrets: string[] = [];
walk(clientDir, secrets);

if (secrets.length > 0) {
  console.error("BUILD FAILED: server-only secrets found in client code:");
  for (const s of secrets) console.error("  -", s);
  process.exit(1);
}
console.log("check:secrets passed — no server-only secrets in client code.");