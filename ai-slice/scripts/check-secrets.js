/* ------------------------------------------------------------------ */
/* Build-time secret check                                            */
/* Blocks model API keys from client bundle.                          */
/* ------------------------------------------------------------------ */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const CLIENT_DIR = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "../src");
const SERVER_PREFIXES = [
  "process.env.GEMINI_API_KEY",
  "process.env.OPENAI_API_KEY",
  "GEMINI_API_KEY",
  "OPENAI_API_KEY",
  "ANTHROPIC_API_KEY",
  "SECRET_",
  "PRIVATE_",
];

const secrets = [];

function walk(dir) {
  for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
    const full = path.join(dir, entry.name);
    if (entry.isDirectory()) {
      if (entry.name !== "test" && entry.name !== "node_modules") walk(full);
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

walk(CLIENT_DIR);

if (secrets.length > 0) {
  console.error("BUILD FAILED: model API keys found in client code:");
  for (const s of secrets) console.error("  -", s);
  process.exit(1);
} else {
  console.log("check:secrets passed — no model API keys in client code.");
}
