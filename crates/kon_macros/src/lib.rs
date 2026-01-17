//! # Kon Macros
//!
//! Procedural macros for Kon Engine that reduce boilerplate.
//!
//! ## `#[system]`
//! Validates system function signatures at compile time.
//! Systems must have exactly one parameter: `ctx: &mut Context`
//!
//! ## `#[component]`
//! Automatically derives Debug, Clone, and PartialEq for component types.
//! Components must be simple data structures.

use proc_macro::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, PatType, Type, parse_macro_input};

/// Marks a function as a system
///
/// Validates that the function has the correct signature for a system:
/// - Exactly one parameter
/// - Parameter type must be `&mut Context`
///
/// This macro doesn't transform the function, it only validates at compile time.
/// The actual system registration happens via `add_system()` or `add_startup_system()`.
///
/// # Example
/// ```ignore
/// #[system]
/// fn movement(ctx: &mut Context) {
///     ctx.world()
///         .select_mut::<(Position, Velocity)>()
///         .each(|_, (pos, vel)| {
///             pos.x += vel.x;
///         });
/// }
/// ```
///
/// # Errors
/// Compile error if:
/// - Function has zero or multiple parameters
/// - Parameter is not `&mut Context`
#[proc_macro_attribute]
pub fn system(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let params: Vec<_> = input.sig.inputs.iter().collect();

    // Check parameter count
    if params.len() != 1 {
        return syn::Error::new_spanned(
            &input.sig,
            "System must have exactly one parameter: ctx: &mut Context",
        )
        .to_compile_error()
        .into();
    }

    // Validate parameter type is &mut Context
    let valid = match &params[0] {
        FnArg::Typed(PatType { ty, .. }) => match ty.as_ref() {
            Type::Reference(r) => {
                r.mutability.is_some()
                    && matches!(r.elem.as_ref(), Type::Path(type_path) if type_path.path.segments.last().is_some_and(|s| s.ident == "Context"))
            }
            _ => false,
        },
        _ => false,
    };

    if !valid {
        return syn::Error::new_spanned(params[0], "System parameter must be: ctx: &mut Context")
            .to_compile_error()
            .into();
    }

    // Return function unchanged (macro is validation-only)
    quote! { #input }.into()
}

/// Marks a struct as a component
///
/// Automatically derives:
/// - `Debug` - Required by Component trait
/// - `Clone` - Useful for component copying
/// - `PartialEq` - Useful for testing and comparison
///
/// Also adds `#[allow(dead_code)]` to prevent warnings on unused fields.
///
/// # Example
/// ```ignore
/// #[component]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
///
/// // Expands to:
/// #[derive(Debug, Clone, PartialEq)]
/// #[allow(dead_code)]
/// struct Position {
///     x: f32,
///     y: f32,
/// }
/// ```
///
/// # Requirements
/// Component types must be:
/// - Simple structs (no enums or unions)
/// - All fields must implement Debug, Clone, PartialEq
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
