import { nativeClient } from "./client.ts";

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

const N = 1_000_000;
const ITERATIONS = 50;
const expected = fibJS(N);

// Warm up the WASM module (triggers load + compile).
await nativeClient.fibonacciNative({ n: 1 });

const loadStart = performance.now();
await nativeClient.fibonacciNative({ n: N });
const loadTime = performance.now() - loadStart;

// Native bench.
let nativeSum = 0;
const nativeStart = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  const r = await nativeClient.fibonacciNative({ n: N });
  nativeSum += r.result;
}
const nativeMean = (performance.now() - nativeStart) / ITERATIONS;

// JS bench.
let jsSum = 0;
const jsStart = performance.now();
for (let i = 0; i < ITERATIONS; i++) {
  jsSum += fibJS(N);
}
const jsMean = (performance.now() - jsStart) / ITERATIONS;

console.log(`fibonacci(n=${N}) mod 1_000_000 over ${ITERATIONS} iterations`);
console.log(`expected result: ${expected}`);
console.log(`native mean: ${nativeMean.toFixed(3)} ms`);
console.log(`js mean:     ${jsMean.toFixed(3)} ms`);
console.log(`speedup:     ${(jsMean / nativeMean).toFixed(2)}x`);
console.log(`module load time (first call incl. compile): ${loadTime.toFixed(3)} ms`);
console.log(`native result match: ${nativeSum / ITERATIONS === expected}`);
console.log(`js result match:     ${jsSum / ITERATIONS === expected}`);