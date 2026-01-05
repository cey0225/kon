//! # Kon Macros
//!
//! Procedural macros for Kon Engine.

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, FnArg, ItemFn, PatType, Type};

/// Marks a function as a system
///
/// System functions must have exactly one parameter: `ctx: &mut Context`
///
/// # Example
/// ```ignore
/// #[system]
/// fn movement(ctx: &mut Context) {
///     // ...
/// }
/// ```
#[proc_macro_attribute]
pub fn system(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let params: Vec<_> = input.sig.inputs.iter().collect();

    if params.len() != 1 {
        return syn::Error::new_spanned(
            &input.sig,
            "System must have exactly one parameter: ctx: &mut Context",
        )
        .to_compile_error()
        .into();
    }

    let valid = match &params[0] {
        FnArg::Typed(PatType { ty, .. }) => match ty.as_ref() {
            Type::Reference(r) => r.mutability.is_some(),
            _ => false,
        },
        _ => false,
    };

    if !valid {
        return syn::Error::new_spanned(&params[0], "System parameter must be: ctx: &mut Context")
            .to_compile_error()
            .into();
    }

    quote! { #input }.into()
}

/// Marks a struct as a component
///
/// Automatically derives Debug, Clone, and PartialEq.
///
/// # Example
/// ```ignore
/// #[component]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
/// ```
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as syn::ItemStruct);

    let output = quote! {
        #[derive(Debug, Clone, PartialEq)]
        #[allow(dead_code)]
        #input
    };

    output.into()
}
