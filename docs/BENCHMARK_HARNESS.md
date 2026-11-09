# Benchmark Harness Design (P0)

Status: design only. No benchmark code exists yet. Per ARCHITECTURE_PLAN.md §6,
no performance claim is made until the benchmark for it is in the repo and has run.

## 1. Principles

1. A metric is reported only when a script in the repo produces it.
2. Every result records the machine, OS, toolchain, and commit it came from.
3. Results are compared against the budget in §3, not against other frameworks.
4. Micro-benchmarks that time a single function are not used as evidence for
   end-to-end claims. Earlier withdrawn claims (4584x, 180x) came from this.

## 2. Layout (planned)

```
innoframework/
  benches/            # Rust criterion benches (RPC round trip, token verify)
  bench/
    fixtures/         # minimal web and desktop fixture apps
    scripts/          # build size, cold start, RSS, Lighthouse runner
    results/          # JSON output, one file per run, committed only as baselines
```

`benches/` is Rust-only and uses no Node. Lighthouse CI is the one optional
Node tool, needed only for the web FCP budget; it runs under nvm with a pinned
Node version when that phase starts.

## 3. Budgets and measurement

Source of truth: ARCHITECTURE_PLAN.md §6.

| Metric | Budget | Method | Phase |
|---|---|---|---|
| RPC round trip, localhost, small JSON, p95 | <= 10 ms | Rust load script against running server | P2 |
| Session token verify, one call | recorded baseline, no budget yet | criterion bench in `innoframework-core` | P0 |
| Web FCP, fixture, 4G throttle | <= 1.5 s | Lighthouse CI on fixture | P3 |
| Web bundle gzipped vs Vite React fixture | <= 2x fixture | build size script | P3 |
| Desktop cold start to first paint | <= 1.5 s | scripted timing, reference Windows VM | P4 |
| Desktop idle RSS | <= 150 MB | process metric | P4 |
| Mobile cold start and frame time | per P9 gate | device lab | P9 |

## 4. Reference hardware

- Low tier: owner machine, i5-8350U, 16 GB RAM, Intel UHD 620, Windows 11 (ADR-12).
- Mid and high tiers: not defined. Results on them are not reported until
  ADR-12 is completed.

Results from the low tier are labeled as such. They are not extrapolated.

## 5. Conflicting targets to resolve

Two documents give different numbers for the same metric:

| Metric | ARCHITECTURE_PLAN.md §6 | INNOFRAMEWORK_ARCHITECTURE.md §2.3 |
|---|---|---|
| Desktop cold start | <= 1.5 s | < 500 ms |
| Desktop idle RSS | <= 150 MB | < 50 MB |
| Web bundle | <= 2x Vite fixture | < 5 MB gzipped |

This harness uses the ARCHITECTURE_PLAN.md column because that document defines
the phase gates. The owner must confirm which set applies before P4.

## 6. Report format

Each run writes one JSON file:

```json
{
  "metric": "rpc_roundtrip_p95_ms",
  "value": 0.0,
  "budget": 10,
  "machine": "i5-8350U / 16GB / UHD620 / Win11",
  "commit": "<sha>",
  "toolchain": "<rustc -V>",
  "date": "<iso8601>"
}
```

## 7. Open questions

- Owner must confirm the §5 conflict.
- Mid and high reference machines (ADR-12).
- Whether benchmark results run in CI or only on the reference machine. CI
  runners are shared and too noisy for latency budgets, so the default is local.