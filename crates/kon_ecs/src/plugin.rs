//! ECS Plugin for registering World as a global resource

use crate::{ContextEcsExt, World};
use kon_core::{App, Context, Plugin};

/// ECS Plugin - registers World as a global resource
///
/// This plugin:
/// - Creates and registers `World` in Context
/// - Adds `apply_deferred_system` to execute deferred operations each frame
///
/// Required for using `ctx.world()`.
pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.register(World::new());
        app.add_sync_system(apply_deferred_system);
    }
}

/// System that applies deferred World operations each frame
///
/// Operations deferred via `world.defer()` are executed here.
/// This prevents issues with modifying the World during queries.
fn apply_deferred_system(ctx: &mut Context) {
    ctx.world().apply_deferred();
}
