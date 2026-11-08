//! Integration tests for RPC request handling.
//!
//! Tests SEC-RPC-1 (401 unauthenticated), SEC-RPC-2 (422 invalid input),
//! CSRF-1/3/6 (403 CSRF failure), and 200 OK success by calling the handler
//! logic directly with constructed inputs.

use axum::{
    body::Body,
    extract::{Path, Request, State},
    http::{header, Method, StatusCode},
    response::Response,
    routing::post,
    Router as AxumRouter,
};
use http_body_util::BodyExt;
use serde_json::Value;
use std::sync::Arc;

use innoframework_core::{issue as issue_token, Role, Session, SessionKey};
use innoframework_rpc::{AuthError, ErrorResponse, Router as RpcRouter};
use innoframework_rpc_macro::rpc;

// ---------------------------------------------------------------------------
// Test routes
// ---------------------------------------------------------------------------

#[allow(dead_code)]
struct TestGuard;

#[rpc(guard = TestGuard)]
async fn echo_route(input: Value) -> Result<Value, AuthError> {
    Ok(input)
}

// ---------------------------------------------------------------------------
// Core handler logic (testable directly)
// ---------------------------------------------------------------------------

/// Core RPC request handling logic.
///
/// Parameters are plain types (no axum extractors) so this can be tested
/// directly without a running HTTP server.
async fn handle_rpc_request(
    route_name: String,
    router: &RpcRouter,
    cookie_value: Option<&str>,
    sec_fetch_site: Option<&str>,
    body: &str,
    key: &SessionKey,
    now: u64,
) -> Response {
    // SEC-RPC-1: auth guard
    let _session = match innoframework_rpc::authenticate(cookie_value, key, now) {
        Ok(s) => s,
        Err(AuthError::Invalid) => {
            return json_response(StatusCode::UNAUTHORIZED, "AUD-2003", "unauthenticated");
        }
        Err(_) => {
            return json_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "AUD-5000",
                "internal error",
            );
        }
    };

    // CSRF-1, CSRF-3, CSRF-6
    let method = Method::POST;
    if innoframework_rpc::check_csrf(sec_fetch_site, &method).is_err() {
        return json_response(StatusCode::FORBIDDEN, "AUD-2006", "csrf check failed");
    }

    // Route lookup
    let route = match router.get(&route_name) {
        Some(r) => r,
        None => {
            return json_response(StatusCode::NOT_FOUND, "AUD-2001", "route not found");
        }
    };

    // SEC-RPC-2: input validation
    let input: Value = match serde_json::from_str(body) {
        Ok(v) => v,
        Err(_) => {
            return json_response(
                StatusCode::UNPROCESSABLE_ENTITY,
                "AUD-2002",
                "invalid input",
            );
        }
    };

    // Call route handler
    match route.handle(input).await {
        Ok(value) => json_response(StatusCode::OK, "AUD-OK", &value.to_string()),
        Err(AuthError::Invalid) => {
            json_response(StatusCode::UNAUTHORIZED, "AUD-2003", "unauthenticated")
        }
        Err(AuthError::Forbidden) => json_response(StatusCode::FORBIDDEN, "AUD-2004", "forbidden"),
        Err(AuthError::WeakSecret) => json_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "AUD-2005",
            "configuration error",
        ),
    }
}

/// Axum handler: thin wrapper around `handle_rpc_request`.
#[allow(dead_code)]
async fn rpc_handler(
    Path(route_name): Path<String>,
    State(router): State<Arc<RpcRouter>>,
    mut request: Request,
) -> Response {
    let headers = request.headers().clone();
    let body_bytes = BodyExt::collect(request.body_mut())
        .await
        .unwrap()
        .to_bytes();
    let body = String::from_utf8_lossy(&body_bytes).to_string();

    let cookie_value = extract_session_cookie(&headers);
    let sec_fetch_site = headers.get("sec-fetch-site").and_then(|v| v.to_str().ok());
    let key = SessionKey::new(b"0123456789abcdef0123456789abcdef").unwrap();

    handle_rpc_request(
        route_name,
        &router,
        cookie_value.as_deref(),
        sec_fetch_site,
        &body,
        &key,
        1500,
    )
    .await
}

#[allow(dead_code)]
fn extract_session_cookie(headers: &http::HeaderMap) -> Option<String> {
    headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|cookie_header| {
            cookie_header.split(';').find_map(|part| {
                let part = part.trim();
                part.strip_prefix("v1_session=").map(|v| v.to_string())
            })
        })
}

fn json_response(status: StatusCode, code: &str, message: &str) -> Response {
    let body = serde_json::to_string(&ErrorResponse {
        code: code.to_string(),
        message: message.to_string(),
    })
    .unwrap();
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body))
        .unwrap()
}

// ---------------------------------------------------------------------------
// Test setup
// ---------------------------------------------------------------------------

fn make_session_token() -> String {
    issue_token(
        &SessionKey::new(b"0123456789abcdef0123456789abcdef").unwrap(),
        &Session {
            user_id: "u1".into(),
            role: Role::User,
            issued_at: 1000,
            expires_at: 2000,
        },
    )
}

fn build_router() -> RpcRouter {
    let mut rpc_router = RpcRouter::new();
    rpc_router.register(EchoRoute).unwrap();
    rpc_router
}

#[allow(dead_code)]
fn build_app() -> AxumRouter {
    AxumRouter::new()
        .route("/rpc/{name}", post(rpc_handler))
        .with_state(Arc::new(build_router()))
}

// ---------------------------------------------------------------------------
// Tests (call handle_rpc_request directly)
// ---------------------------------------------------------------------------

#[tokio::test]
async fn rpc_without_session_returns_401() {
    let router = build_router();
    let key = SessionKey::new(b"0123456789abcdef0123456789abcdef").unwrap();
    let mut resp = handle_rpc_request(
        "echo".to_string(),
        &router,
        None,
        Some("same-origin"),
        r#"{"key":"value"}"#,
        &key,
        1500,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    let body = BodyExt::collect(resp.body_mut()).await.unwrap().to_bytes();
    let body_str = String::from_utf8_lossy(&body).to_string();
    assert!(body_str.contains("unauthenticated"));
}

#[tokio::test]
async fn rpc_with_cross_site_csrf_returns_403() {
    let router = build_router();
    let key = SessionKey::new(b"0123456789abcdef0123456789abcdef").unwrap();
    let resp = handle_rpc_request(
        "echo".to_string(),
        &router,
        Some(&make_session_token()),
        Some("cross-site"),
        r#"{"key":"value"}"#,
        &key,
        1500,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn rpc_with_valid_session_returns_200() {
    let router = build_router();
    let key = SessionKey::new(b"0123456789abcdef0123456789abcdef").unwrap();
    let mut resp = handle_rpc_request(
        "echo".to_string(),
        &router,
        Some(&make_session_token()),
        Some("same-origin"),
        r#"{"hello":"world"}"#,
        &key,
        1500,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::OK);
    let body = BodyExt::collect(resp.body_mut()).await.unwrap().to_bytes();
    let body_str = String::from_utf8_lossy(&body).to_string();
    assert!(body_str.contains("hello"));
}

#[tokio::test]
async fn rpc_with_invalid_json_returns_422() {
    let router = build_router();
    let key = SessionKey::new(b"0123456789abcdef0123456789abcdef").unwrap();
    let resp = handle_rpc_request(
        "echo".to_string(),
        &router,
        Some(&make_session_token()),
        Some("same-origin"),
        "not valid json{{{",
        &key,
        1500,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn rpc_nonexistent_route_returns_404() {
    let router = build_router();
    let key = SessionKey::new(b"0123456789abcdef0123456789abcdef").unwrap();
    let resp = handle_rpc_request(
        "nonexistent_route".to_string(),
        &router,
        Some(&make_session_token()),
        Some("same-origin"),
        r#"{}"#,
        &key,
        1500,
    )
    .await;
    assert_eq!(resp.status(), StatusCode::NOT_FOUND);
}
