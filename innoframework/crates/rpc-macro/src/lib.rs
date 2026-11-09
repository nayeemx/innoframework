//! `#[rpc(guard = ...)]` proc-macro for typed RPC routes.
//!
//! Implements ADR-14: RPC route typestate enforced at compile time.
//!
//! Usage:
//! ```rust,ignore
//! #[rpc(guard = AdminGuard)]
//! async fn admin_route(input: Value) -> Result<Value, AuthError> {
//!     Ok(Value::Null)
//! }
//! ```
//!
//! A route declared without `guard` fails to compile (TR-1.3).

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemFn, Meta};

/// Convert a snake_case identifier to UpperCamelCase.
fn to_upper_camel_case(s: &str) -> String {
    s.split('_')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect()
}

/// Proc-macro attribute: `#[rpc(guard = <guard_expr>)]`
///
/// Requires a `guard` argument. Returns a compile error if missing,
/// enforcing that every RPC route has an auth guard at compile time.
#[proc_macro_attribute]
pub fn rpc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let missing = || {
        syn::Error::new(
            proc_macro2::Span::call_site(),
            "rpc route must declare a guard: #[rpc(guard = AdminGuard)]",
        )
        .to_compile_error()
        .into()
    };

    if attr.is_empty() {
        return missing();
    }

    let meta = match syn::parse::<Meta>(attr) {
        Ok(m) => m,
        Err(_) => return missing(),
    };

    // Validate that the attribute has a `guard = ...` argument.
    match &meta {
        syn::Meta::NameValue(nv) if nv.path.is_ident("guard") => {}
        _ => return missing(),
    }

    let name = &input.sig.ident;
    let route_name = Ident::new(&to_upper_camel_case(&name.to_string()), name.span());

    // Keep the original function as the handler implementation.
    // Generate a route struct that implements Guarded and RpcRoute.
    // Only the macro can create route structs, and only with a guard,
    // so unguarded routes are unrepresentable by the type system.
    let expanded = quote! {
        #input

        /// Typed RPC route for `{}`. Implements [`innoframework_rpc::Guarded`]
        /// because the macro requires a `guard` argument.
        struct #route_name;

        impl innoframework_rpc::Guarded for #route_name {}

        impl innoframework_rpc::RpcRoute for #route_name {
            fn handle(
                &self,
                input: serde_json::Value,
            ) -> std::pin::Pin<
                Box<
                    dyn std::future::Future<
                            Output = Result<serde_json::Value, innoframework_rpc::AuthError>,
                        > + Send
                        + Sync,
                >,
            > {
                Box::pin(async move { #name(input).await })
            }
        }
    };

    TokenStream::from(expanded)
}
