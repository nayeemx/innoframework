// TR-1.3 / SEC-RPC negative: route without a guard must not compile.
#[innoframework_rpc_macro::rpc]
async fn unguarded(_input: serde_json::Value) -> Result<serde_json::Value, innoframework_rpc::AuthError> {
    Ok(serde_json::Value::Null)
}

fn main() {}
