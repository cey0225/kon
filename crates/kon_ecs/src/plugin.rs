//! ECS Plugin for Kon Engine.

use crate::{ContextEcsExt, World};
use kon_core::{App, Context, Plugin};

/// ECS Plugin - registers World as global
pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.register(World::new());
        app.add_system(apply_deferred_system);
    }
}

fn apply_deferred_system(ctx: &mut Context) {
    ctx.world_mut().apply_deferred();
}
