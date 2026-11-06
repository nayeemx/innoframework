#!/usr/bin/env node
/* ------------------------------------------------------------------ */
/* Stop llama-server.exe                                              */
/* ------------------------------------------------------------------ */

import { execSync } from "child_process";

try {
  execSync("taskkill /F /IM llama-server.exe", { stdio: "inherit" });
  console.log("llama-server stopped.");
} catch {
  console.log("llama-server was not running.");
}
