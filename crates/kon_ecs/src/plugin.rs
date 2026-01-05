//! ECS Plugin for Kon Engine.

use crate::World;
use kon_core::{App, Plugin};

/// ECS Plugin - registers World as global
pub struct EcsPlugin;

impl Plugin for EcsPlugin {
    fn build(&self, app: &mut App) {
        app.register(World::new());
    }
}
