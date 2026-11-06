/* Build-time secret check.
 * Fails the build if server-only env/secrets are referenced in client code.
 * Run: node scripts/check-secrets.js ../src
 */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const CLIENT_DIR = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../src");
const SERVER_PREFIXES = ["process.env.SERVER_", "import.meta.env.SERVER_", "SECRET_", "PRIVATE_"];
const EXCLUDE = ["node_modules", "dist", ".git", "test"];

const secrets: string[] = [];

function walk(dir: string): void {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (!EXCLUDE.includes(entry.name)) walk(full);
    } else if (entry.isFile() && /\.(ts|tsx|js|jsx)$/.test(entry.name)) {
      const content = fs.readFileSync(full, "utf-8");
      for (const prefix of SERVER_PREFIXES) {
        if (content.includes(prefix)) secrets.push(`${full}: contains "${prefix}"`);
      }
    }
  }
}

walk(CLIENT_DIR);

if (secrets.length > 0) {
  console.error("BUILD FAILED: server-only secrets found in client code:");
  for (const s of secrets) console.error("  -", s);
  process.exit(1);
}
console.log("check:secrets passed — no server-only secrets in client code.");