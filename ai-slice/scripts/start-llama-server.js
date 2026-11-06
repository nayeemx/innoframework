#!/usr/bin/env node
/* ------------------------------------------------------------------ */
/* Startup script for llama-server.exe                                */
/* Starts a local model server for the LlamaCppProvider.              */
/*                                                                    */
/* Usage:                                                             */
/*   node scripts/start-llama-server.js                               */
/*                                                                    */
/* Environment variables:                                             */
/*   LLAMA_SERVER_EXE   - path to llama-server.exe                    */
/*   LLAMA_MODEL        - path to GGUF model file                     */
/*   LLAMA_PORT         - port to listen on (default 11434)           */
/*   LLAMA_HOST         - host to bind (default 127.0.0.1)            */
/*   LLAMA_CTX_SIZE     - context size (default 4096)                 */
/* ------------------------------------------------------------------ */

import { spawn } from "child_process";
import path from "path";

const LLAMA_SERVER_EXE =
  process.env.LLAMA_SERVER_EXE ??
  "D:\\models\\llama.cpp\\b9142\\llama-server.exe";

const LLAMA_MODEL =
  process.env.LLAMA_MODEL ??
  "D:\\models\\blobs\\sha256-dde5aa3fc5ffc17176b5e8bdc82f587b24b2678c6c66101bf7da77af9f7ccdff";

const LLAMA_PORT = process.env.LLAMA_PORT ?? "11434";
const LLAMA_HOST = process.env.LLAMA_HOST ?? "127.0.0.1";
const LLAMA_CTX_SIZE = process.env.LLAMA_CTX_SIZE ?? "4096";

/* Verify the executable exists */
const fs = await import("fs");
if (!fs.existsSync(LLAMA_SERVER_EXE)) {
  console.error(`llama-server.exe not found: ${LLAMA_SERVER_EXE}`);
  process.exit(1);
}

if (!fs.existsSync(LLAMA_MODEL)) {
  console.error(`Model file not found: ${LLAMA_MODEL}`);
  process.exit(1);
}

console.log(`Starting llama-server...`);
console.log(`  exe:  ${LLAMA_SERVER_EXE}`);
console.log(`  model: ${LLAMA_MODEL}`);
console.log(`  host: ${LLAMA_HOST}:${LLAMA_PORT}`);
console.log(`  ctx:  ${LLAMA_CTX_SIZE}`);

const child = spawn(
  `"${LLAMA_SERVER_EXE}"`,
  [
    "--model", LLAMA_MODEL,
    "--host", LLAMA_HOST,
    "--port", LLAMA_PORT,
    "--ctx-size", LLAMA_CTX_SIZE,
  ],
  { shell: true, stdio: "inherit", cwd: path.dirname(LLAMA_SERVER_EXE) },
);

child.on("error", (err) => {
  console.error("Failed to start llama-server:", err.message);
  process.exit(1);
});

child.on("exit", (code, signal) => {
  if (signal) {
    console.log(`llama-server terminated by signal ${signal}`);
  } else if (code !== 0) {
    console.error(`llama-server exited with code ${code}`);
    process.exit(code ?? 1);
  } else {
    console.log("llama-server exited cleanly");
  }
});

/* Keep the process alive */
process.on("SIGINT", () => {
  child.kill("SIGINT");
});
process.on("SIGTERM", () => {
  child.kill("SIGTERM");
});
