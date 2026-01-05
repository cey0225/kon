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
//!     // Multiple components query with fluent API
//!     ctx.world_mut()
//!         .select_mut::<Position>()
//!         .with::<Velocity>()
//!         .tagged("player")
//!         .each(|entity, (pos, vel)| {
//!             pos.x += vel.x;
//!             pos.y += vel.y;
//!         });
//! }
//! ```

mod entity;
mod ext;
mod plugin;
mod query;
mod storage;
mod world;

pub use entity::{Entity, EntityBuilder};
pub use ext::ContextEcsExt;
pub use plugin::EcsPlugin;
pub use query::{Query, QueryMut};
pub use world::World;

pub mod prelude {
    pub use crate::{ContextEcsExt, EcsPlugin, Entity, World};
}
