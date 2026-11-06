/* ------------------------------------------------------------------ */
/* v4-shell native-vs-JS benchmark                                    */
/*                                                                    */
/* Compares a Rust-native fibonacci (compiled to a standalone binary) */
/* against the same algorithm in JavaScript. Phase 2 exit criterion:  */
/* native path beats JS at the realistic input size, or is removed.   */
/*                                                                    */
/*   npx tsx benchmark.ts                                             */
/* ------------------------------------------------------------------ */

import { spawn } from "node:child_process";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

/* ---- JavaScript fibonacci ---- */
function fibJS(n: number): number {
  const MOD = 1_000_000;
  if (n <= 0) return 0;
  if (n === 1) return 1;
  let a = 0;
  let b = 1;
  for (let i = 2; i <= n; i++) {
    const c = (a + b) % MOD;
    a = b;
    b = c;
  }
  return b;
}

/* ---- Benchmark runner ---- */
async function benchJS(n: number, iterations: number): Promise<number> {
  // Warm up
  fibJS(n);
  const start = performance.now();
  for (let i = 0; i < iterations; i++) {
    fibJS(n);
  }
  return (performance.now() - start) / iterations;
}

/* ---- Rust-native fibonacci (standalone binary) ---- */
const rustSource = `
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let n: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(20);
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    for _ in 0..n {
        let c = a.wrapping_add(b);
        a = b;
        b = c;
    }
    println!("{}", a);
}
`;

async function benchRust(n: number, iterations: number): Promise<number> {
  const rustFile = path.join(__dirname, "fib_native.rs");
  const binFile = path.join(
    __dirname,
    "fib_native" + (process.platform === "win32" ? ".exe" : ""),
  );
  fs.writeFileSync(rustFile, rustSource);

  // Compile
  const compile = spawn("rustc", [rustFile, "-o", binFile], {
    stdio: ["pipe", "pipe", "pipe"],
  });
  await new Promise<void>((resolve, reject) => {
    compile.on("close", (code) => {
      if (code === 0) resolve();
      else reject(new Error(`rustc failed with code ${code}`));
    });
    compile.on("error", reject);
  });

  // Warm up
  await runRustBin(binFile, n);

  // Benchmark
  const start = performance.now();
  for (let i = 0; i < iterations; i++) {
    await runRustBin(binFile, n);
  }
  return (performance.now() - start) / iterations;
}

async function runRustBin(binFile: string, n: number): Promise<number> {
  return new Promise((resolve, reject) => {
    const child = spawn(binFile, [n.toString()], {
      stdio: ["pipe", "pipe", "pipe"],
    });
    let output = "";
    child.stdout.on("data", (d) => (output += d.toString()));
    child.on("close", (code) => {
      if (code === 0) resolve(parseInt(output.trim(), 10));
      else reject(new Error(`binary failed with code ${code}`));
    });
    child.on("error", reject);
  });
}

async function main() {
  const N = 20_000;
  const ITERATIONS = 20;

  console.log(`fibonacci(n=${N}) over ${ITERATIONS} iterations\n`);

  const jsMean = await benchJS(N, ITERATIONS);
  console.log(`JS mean:   ${jsMean.toFixed(3)} ms`);

  let rustMean: number;
  try {
    rustMean = await benchRust(N, ITERATIONS);
    console.log(`Native mean: ${rustMean.toFixed(3)} ms`);
    const speedup = jsMean / rustMean;
    console.log(`Speedup:   ${speedup.toFixed(2)}x`);
    if (speedup > 1.0) {
      console.log("\nPhase 2 exit criterion: MET — native beats JS.");
    } else {
      console.log("\nPhase 2 exit criterion: NOT MET — native does not beat JS.");
    }
  } catch (err) {
    console.error(`Native benchmark failed: ${err}`);
    process.exit(1);
  }
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
