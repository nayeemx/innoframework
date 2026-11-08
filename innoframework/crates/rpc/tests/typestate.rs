//! Compile-time typestate tests for RPC routes.
//!
//! TR-1.3: "an RPC endpoint without an auth guard fails to compile".
//! The `#[rpc(guard = ...)]` proc-macro (ADR-14) requires a guard, so
//! unguarded routes are unrepresentable by the type system.
//!
//! These tests verify the positive direction: a guarded route implements
//! `Guarded` and `RpcRoute`, and can be registered.

use innoframework_rpc::{register_route, AuthError, Guarded, RpcRoute};
use innoframework_rpc_macro::rpc;
use serde_json::Value;

/// Guard types used as the `guard` argument in tests.
#[allow(dead_code)]
struct AdminGuard;
#[allow(dead_code)]
struct UserGuard;

/// A route declared with `guard`. The macro generates `AdminRoute`
/// implementing `Guarded` and `RpcRoute`.
#[rpc(guard = AdminGuard)]
async fn admin_route(_input: Value) -> Result<Value, AuthError> {
    Ok(Value::String("admin".into()))
}

/// A route declared with `guard` for a different role.
#[rpc(guard = UserGuard)]
async fn user_route(_input: Value) -> Result<Value, AuthError> {
    Ok(Value::String("user".into()))
}

/// Compile-time assertion: only `T: Guarded + RpcRoute` may be passed.
fn assert_guarded<T: Guarded + RpcRoute>() {}

#[test]
fn guarded_route_implements_guarded_trait() {
    assert_guarded::<AdminRoute>();
    assert_guarded::<UserRoute>();
}

#[test]
fn guarded_route_can_be_registered() {
    let admin = AdminRoute;
    let user = UserRoute;
    register_route(&admin);
    register_route(&user);
}

#[tokio::test]
async fn route_handle_returns_result() {
    let admin = AdminRoute;
    let result = admin.handle(Value::Null).await;
    assert_eq!(result, Ok(Value::String("admin".into())));

    let user = UserRoute;
    let result = user.handle(Value::Null).await;
    assert_eq!(result, Ok(Value::String("user".into())));
}
