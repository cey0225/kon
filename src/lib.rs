//! # Kon Engine
//!
//! Modular, plugin-based 2D game engine.
//!
//! # Example
//! ```ignore
//! use kon::prelude::*;
//!
//! #[component]
//! struct Position { x: f32, y: f32 }
//!
//! #[system]
//! fn setup(ctx: &mut Context) {
//!     ctx.world_mut()
//!         .spawn()
//!         .insert(Position { x: 0.0, y: 0.0 })
//!         .tag("player")
//!         .id();
//! }
//!
//! fn main() {
//!     kon::init_logger();
//!
//!     Kon::new()
//!         .add_plugin(DefaultPlugins)
//!         .add_startup_system(setup)
//!         .run();
//! }
//! ```

pub use kon_core;
pub use kon_ecs;
pub use kon_macros::{component, system};
pub use log;

use kon_core::Plugin;

pub mod prelude {
    //! Common imports for Kon Engine
    pub use crate::DefaultPlugins;
    pub use crate::{component, system};
    pub use kon_core::{App, Context, Event, Events, Globals, Kon, Plugin, Time};
    pub use kon_ecs::{ContextEcsExt, EcsPlugin, Entity, EntityBuilder, Query, World};
}

/// Engine version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Default plugins bundle
///
/// Includes:
/// - `EcsPlugin` - Entity Component System
pub struct DefaultPlugins;

impl Plugin for DefaultPlugins {
    fn build(&self, app: &mut kon_core::App) {
        app.add_plugin(kon_ecs::EcsPlugin);
    }

    fn is_plugin_group(&self) -> bool {
        true
    }
}
