//! Extension trait for accessing World from Context
//!
//! Provides convenient `world()` method on Context
//! instead of manually calling `ctx.global::<World>()`.

use std::cell::RefMut;
use crate::World;
use kon_core::Context;

/// Extension trait for convenient World access from Context
///
/// # Panics
/// Panics if World is not registered. Ensure `EcsPlugin` or `DefaultPlugins` is added.
pub trait ContextEcsExt {
    fn world(&self) -> RefMut<'_, World>;
}

impl ContextEcsExt for Context {
    /// Returns a reference to the World
    ///
    /// # Panics
    /// Panics with a helpful message if EcsPlugin is not registered
    #[track_caller]
    fn world(&self) -> RefMut<'_, World> {
        self.global::<World>()
            .expect("Failed to access World. Ensure 'DefaultPlugins' or 'EcsPlugin' is added")
    }
}
