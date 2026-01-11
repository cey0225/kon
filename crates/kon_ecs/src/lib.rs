//! # Kon ECS
//!
//! Entity Component System for Kon Engine.
//!
//! # Example
//! ```ignore
//! use kon::prelude::*;
//!
//! #[component]
//! struct Position { x: f32, y: f32 }
//!
//! #[component]
//! struct Velocity { x: f32, y: f32 }
//!
//! #[system]
//! fn setup(ctx: &mut Context) {
//!     ctx.world_mut()
//!         .spawn()
//!         .insert(Position { x: 0.0, y: 0.0 })
//!         .insert(Velocity { x: 1.0, y: 0.0 })
//!         .tag("player")
//!         .id();
//! }
//!
//! #[system]
//! fn movement(ctx: &mut Context) {
//!     ctx.world_mut()
//!         .select_mut::<(Position, Velocity)>()
//!         .each(|entity, (pos, vel)| {
//!             pos.x += vel.x;
//!             pos.y += vel.y;
//!         });
//! }
//!
//! fn main() {
//!     Kon::new()
//!         .add_plugin(DefaultPlugins)
//!         .add_startup_system(setup)
//!         .add_system(movement)
//!         .run();
//! }
//! ```

mod entity;
mod ext;
mod plugin;
mod query;
mod storage;
mod world;

use std::{any::Any, fmt::Debug};

/// Base trait for all components
///
/// Automatically implemented for types that are:
/// - `Any + Send + Sync + Debug + 'static`
///
/// You don't need to implement this manually. Just use `#[component]` macro
/// or derive the required traits:
///
/// ```ignore
/// #[component]
/// struct Position { x: f32, y: f32 }
///
/// // Or manually:
/// #[derive(Debug, Clone, PartialEq)]
/// struct Velocity { x: f32, y: f32 }
/// ```
pub trait Component: Any + Send + Sync + Debug + 'static {}
impl<T: Any + Send + Sync + Debug + 'static> Component for T {}

pub use entity::{Entity, EntityBuilder};
pub use ext::ContextEcsExt;
pub use plugin::EcsPlugin;
pub use query::{Query, QueryMut};
pub use world::World;

pub mod prelude {
    pub use crate::{ContextEcsExt, EcsPlugin, Entity, World};
}
