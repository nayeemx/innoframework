#!/usr/bin/env node
/*
 * create-v1-app — scaffold a new Framework v1 app.
 *
 *   npx create-v1-app@latest my-app
 *   npx create-v1-app@latest my-app --template minimal
 *
 * Generates: package.json, tsconfig, vite config, one route, one server
 * function, auth guard, CSP preset, vitest config, and the secret check.
 */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const TEMPLATES = path.resolve(__dirname, "../templates");

const args = process.argv.slice(2);
const name = args[0];
const template = args.includes("--template=minimal") ? "minimal" : "default";
const force = args.includes("--force");

if (!name) {
  console.error("usage: create-v1-app <name> [--template minimal] [--force]");
  process.exit(2);
}

const target = path.resolve(name);
if (fs.existsSync(target) && fs.readdirSync(target).length > 0 && !force) {
  console.error(`directory "${name}" already exists and is not empty. Use --force to overwrite.`);
  process.exit(1);
}
fs.mkdirSync(target, { recursive: true });

const tplDir = path.join(TEMPLATES, template);
if (!fs.existsSync(tplDir)) {
  console.error(`template "${template}" not found at ${tplDir}`);
  process.exit(1);
}

// Copy template contents into the target (not the template dir itself).
for (const entry of fs.readdirSync(tplDir, { withFileTypes: true })) {
  const src = path.join(tplDir, entry.name);
  const dest = path.join(target, entry.name);
  if (entry.isDirectory()) {
    copyDir(src, dest);
  } else {
    fs.mkdirSync(path.dirname(dest), { recursive: true });
    let content = fs.readFileSync(src, "utf-8");
    content = content.replaceAll("{{NAME}}", name);
    fs.writeFileSync(dest, content);
  }
}

console.log(`Created "${name}" at ${target}`);
console.log(`  cd ${name}`);
console.log(`  npm install`);
console.log(`  npm run dev`);

function copyDir(src: string, dest: string): void {
  fs.mkdirSync(dest, { recursive: true });
  for (const entry of fs.readdirSync(src, { withFileTypes: true })) {
    const s = path.join(src, entry.name);
    const d = path.join(dest, entry.name);
    if (entry.isDirectory()) {
      copyDir(s, d);
    } else {
      fs.mkdirSync(path.dirname(d), { recursive: true });
      let content = fs.readFileSync(s, "utf-8");
      content = content.replaceAll("{{NAME}}", name);
      fs.writeFileSync(d, content);
    }
  }
}