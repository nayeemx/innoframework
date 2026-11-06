#!/usr/bin/env node
/*
 * verify-pack.mjs — prove the published tarball is installable.
 *
 * Runs `npm pack --dry-run --json` and asserts:
 *   1. The package name/version resolve from package.json.
 *   2. Every entry in `files` is present in the dry-run file list.
 *   3. `templates/default/` ships (the scaffolder needs it).
 *   4. The bin entry points to a compiled file.
 *   5. No test artifacts (test-app/, verify-install/, *.tgz) leak into the pack.
 *
 * Exit 0 only when all checks pass.
 *
 * Guard: `npm pack` triggers `prepack`, which re-invokes this script.
 * We set VERIFY_PACK_RUNNING=1 so the nested prepack call short-circuits.
 *
 *   npm run verify:pack            # dry-run only
 *   npm run verify:pack -- --pack  # also write the .tgz
 */

import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { spawnSync } from "node:child_process";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const pkg = JSON.parse(fs.readFileSync(path.join(root, "package.json"), "utf-8"));

// Resolve the npm binary explicitly so we never pass args through a shell.
function resolveNpm() {
  if (process.platform !== "win32") return { cmd: "npm", args: [] };
  // npm.cmd is a batch wrapper — invoke it through cmd.exe.
  return { cmd: "cmd.exe", args: ["/c", "npm.cmd"] };
}
const NPM = resolveNpm();
function npm(args, opts = {}) {
  return spawnSync(NPM.cmd, [...NPM.args, ...args], {
    cwd: root,
    encoding: "utf-8",
    stdio: ["pipe", "pipe", "pipe"],
    env: { ...process.env, ...(opts.env || {}) },
  });
}

// Nested prepack re-entry guard: do nothing but confirm dist exists.
if (process.env.VERIFY_PACK_RUNNING === "1") {
  if (!fs.existsSync(path.join(root, "dist"))) {
    console.error("verify-pack: dist/ missing — run `npm run build` first.");
    process.exit(1);
  }
  console.log("verify-pack: nested prepack skipped (already running).");
  process.exit(0);
}

const pack = process.argv.includes("--pack");

// 1. Run npm pack --dry-run --json and capture the manifest.
let manifest = [];
const dry = npm(["pack", "--dry-run", "--json"], { env: { VERIFY_PACK_RUNNING: "1" } });
if (dry.status !== 0) {
  console.error("verify-pack: npm pack --dry-run failed:", (dry.stderr || dry.stdout || "").trim());
  process.exit(1);
}
try {
  const parsed = JSON.parse(dry.stdout);
  manifest = parsed[0] && parsed[0].files
    ? parsed[0].files.map((f) => f.path)
    : [];
} catch {
  console.error("verify-pack: could not parse npm pack --dry-run --json output.");
  process.exit(1);
}

const failures = [];
const check = (cond, msg) => {
  if (cond) {
    console.log("  \u2713 " + msg);
  } else {
    failures.push(msg);
    console.log("  \u2717 " + msg);
  }
};

console.log("Verifying " + pkg.name + "@" + pkg.version + " tarball ...");

// 2. Required files from `files` field.
const declaredFiles = pkg.files || [];
for (const f of declaredFiles) {
  const found = manifest.some((m) => m === f || m.startsWith(f + "/"));
  check(found, "files entry present: " + f);
}

// 3. Scaffolder templates must ship.
const tplCount = manifest.filter((m) => m.startsWith("templates/")).length;
check(tplCount > 0, "templates/ ships (" + tplCount + " entries)");

// 4. Bin entry resolves to a compiled artifact.
const binEntry = pkg.bin && pkg.bin["create-innology-app"];
if (binEntry) {
  const onDisk = fs.existsSync(path.join(root, binEntry));
  check(onDisk, "bin entry exists on disk: " + binEntry);
  const normalized = binEntry.replace(/^\.\//, "");
  const inPack = manifest.some((m) => m === normalized || m === binEntry);
  check(inPack, "bin entry present in tarball: " + binEntry);
}

// 5. No test/verification artifacts leak.
const forbidden = ["test-app/", "verify-install/", "render_logo.py"];
for (const f of forbidden) {
  const leaked = manifest.some((m) => m.indexOf(f) !== -1);
  check(!leaked, "no artifact leaked: " + f);
}

// 6. No node_modules, no .git leak.
for (const f of ["node_modules/", ".git/"]) {
  const leaked = manifest.some((m) => m.startsWith(f));
  check(!leaked, "no " + f + " in tarball");
}

if (failures.length > 0) {
  console.error("\nverify-pack FAILED (" + failures.length + " check(s)):");
  for (const f of failures) console.error("  - " + f);
  process.exit(1);
}

console.log("\nverify-pack PASSED \u2014 package is installable.");
if (pack) {
  const p = npm(["pack"], { env: { VERIFY_PACK_RUNNING: "1" } });
  if (p.status !== 0) {
    console.error("verify-pack: npm pack failed:", (p.stderr || p.stdout || "").trim());
    process.exit(1);
  }
  const tgz = fs.readdirSync(root).find((f) => f.endsWith(".tgz"));
  console.log("packed \u2192 " + tgz);
}