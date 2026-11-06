/* ------------------------------------------------------------------ */
/* Secret check test                                                  */
/* Verifies no server-only secrets leak into client source.           */
/* ------------------------------------------------------------------ */

import { describe, it, expect } from "vitest";
import fs from "fs";
import path from "path";

const CLIENT_DIR = path.resolve(__dirname, "../../src");
const SERVER_PREFIXES = ["process.env.SERVER_", "import.meta.env.SERVER_", "SECRET_", "PRIVATE_"];

describe("build-time secret check", () => {
  it("finds no server-only secret prefixes in client source", () => {
    const hits: string[] = [];
    function walk(dir: string) {
      for (const entry of fs.readdirSync(dir, { withFileTypes: true })) {
        const full = path.join(dir, entry.name);
        if (entry.isDirectory()) {
          if (entry.name === "test") continue; // test files reference the patterns themselves
          walk(full);
        } else if (/\.(ts|tsx)$/.test(entry.name)) {
          const content = fs.readFileSync(full, "utf-8");
          for (const prefix of SERVER_PREFIXES) {
            if (content.includes(prefix)) hits.push(`${full}: contains "${prefix}"`);
          }
        }
      }
    }
    walk(CLIENT_DIR);
    expect(hits).toEqual([]);
  });
});
