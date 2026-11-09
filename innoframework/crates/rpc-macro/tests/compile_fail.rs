//! Compile-fail (trybuild) tests for the `#[rpc]` macro.
//!
//! TR-1.3 / SEC-RPC: an RPC endpoint without an auth guard must fail to
//! compile. These cases live under `tests/ui/*.rs` with matching `.stderr`.

#[test]
fn ui() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/missing_guard.rs");
}
