#!/usr/bin/env node
/*
 * clean.mjs — remove build output before a fresh compile.
 *
 *   npm run clean
 */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const root = path.dirname(fileURLToPath(import.meta.url));
const dist = path.resolve(root, "../dist");

if (fs.existsSync(dist)) {
  fs.rmSync(dist, { recursive: true, force: true });
  console.log(`clean: removed ${dist}`);
} else {
  console.log("clean: dist/ already absent");
}