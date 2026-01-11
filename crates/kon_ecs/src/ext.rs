//! Extension trait for accessing World from Context
//!
//! Provides convenient `world()` and `world_mut()` methods on Context
//! instead of manually calling `ctx.global::<World>()`.

use crate::World;
use kon_core::Context;

/// Extension trait for convenient World access from Context
///
/// # Panics
/// Panics if World is not registered. Ensure `EcsPlugin` or `DefaultPlugins` is added.
pub trait ContextEcsExt {
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
}

impl ContextEcsExt for Context {
    /// Returns an immutable reference to the World
    ///
    /// # Panics
    /// Panics with a helpful message if EcsPlugin is not registered
    #[track_caller]
    fn world(&self) -> &World {
        self.global::<World>()
            .expect("Failed to access World. Ensure 'DefaultPlugins' or 'EcsPlugin' is added")
    }

    /// Returns a mutable reference to the World
    ///
    /// # Panics
    /// Panics with a helpful message if EcsPlugin is not registered
    #[track_caller]
    fn world_mut(&mut self) -> &mut World {
        self.global_mut::<World>()
            .expect("Failed to access World. Ensure 'DefaultPlugins' or 'EcsPlugin' is added")
    }
}
