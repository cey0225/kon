//! Extension trait for accessing Input from Context
//!
//! Provides convenient `input()` method on Context
//! instead of manually calling `ctx.global::<Input>()`.

use std::cell::RefMut;
use kon_core::Context;
use crate::Input;

/// Extension trait for convenient Input access from Context
///
/// # Example
/// ```ignore
/// fn my_system(ctx: &mut Context) {
///     let input = ctx.input();
///     if input.is_action_pressed("Jump") {
///         // ...
///     }
/// }
/// ```
///
/// # Panics
/// Panics if Input is not registered. Ensure `InputPlugin` or `DefaultPlugins` is added.
pub trait ContextInputExt {
    fn input(&self) -> RefMut<'_, Input>;
}

impl ContextInputExt for Context {
    /// Returns a reference to the Input manager
    ///
    /// # Panics
    /// Panics with a helpful message if InputPlugin is not registered
    #[track_caller]
    fn input(&self) -> RefMut<'_, Input> {
        self.global::<Input>()
            .expect("Failed to access Input. Ensure 'DefaultPlugins' or 'InputPlugin' is added")
    }
}
