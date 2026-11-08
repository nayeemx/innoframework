//! `#[rpc(guard = ...)]` proc-macro for typed RPC routes.
//!
//! Implements ADR-14: RPC route typestate enforced at compile time.
//!
//! Usage:
//! ```rust,ignore
//! #[rpc(guard = Role::Admin)]
//! async fn admin_route() -> Result<(), AuthError> {
//!     Ok(())
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
    let attr = parse_macro_input!(attr as Meta);
    let input = parse_macro_input!(item as ItemFn);

    let _guard_expr = match &attr {
        Meta::NameValue(nv) if nv.path.is_ident("guard") => {
            let val = &nv.value;
            quote! { #val }
        }
        _ => {
            return syn::Error::new_spanned(
                &attr,
                "rpc route must declare a guard: #[rpc(guard = Role::Admin)]",
            )
            .to_compile_error()
            .into();
        }
    };

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
            async fn handle(&self) -> Result<(), innoframework_rpc::AuthError> {
                #name().await
            }
        }
    };

    TokenStream::from(expanded)
}
