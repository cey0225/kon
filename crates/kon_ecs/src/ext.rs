//! Extension trait for Context.

use crate::World;
use kon_core::Context;

/// Extension trait for World access from Context
pub trait ContextEcsExt {
    fn world(&self) -> &World;
    fn world_mut(&mut self) -> &mut World;
}

impl ContextEcsExt for Context {
    fn world(&self) -> &World {
        self.global::<World>()
            .expect("Failed to access World. Ensure 'DefaultPlugins' or 'EcsPlugin' is added")
    }

    fn world_mut(&mut self) -> &mut World {
        self.global_mut::<World>()
            .expect("Failed to access World. Ensure 'DefaultPlugins' or 'EcsPlugin' is added")
    }
}
