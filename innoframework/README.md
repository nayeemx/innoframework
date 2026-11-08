# innoframework

Rust workspace for the Innoframework framework (P0 foundation).

## Crates

| Crate | Path | Purpose |
|---|---|---|
| `innoframework-core` | `crates/core` | Session tokens (HMAC-SHA256), roles, fail-closed auth checks |
| `innoframework-rpc` | `crates/rpc` | RPC layer placeholder (P2) |
| `innoframework-cli` (binary `inno`) | `crates/cli` | CLI placeholder. Binary name is provisional. |

## Requirements

- Rust stable. The toolchain is pinned by `rust-toolchain.toml`.
- No Node.js is required to build or test this workspace (ADR-06).

## Commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo deny check
cargo audit
```

CI runs each of these in `.github/workflows/innoframework-ci.yml`.

## Security docs

- `../docs/THREAT_MODEL.md`: threats, mitigations, and test IDs
- `../docs/TRACEABILITY.md`: requirement to test status matrix
- `../docs/BENCHMARK_HARNESS.md`: benchmark design (no performance claims yet)
- `../docs/ADR_LOG.md`: architecture decisions
