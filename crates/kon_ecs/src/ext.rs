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
        if self.global::<World>().is_none() {
            handle_missing_plugin();
        }

        self.global::<World>().unwrap()
    }

    fn world_mut(&mut self) -> &mut World {
        if self.global_mut::<World>().is_none() {
            handle_missing_plugin();
        }

        self.global_mut::<World>().unwrap()
    }
}

fn handle_missing_plugin() -> ! {
    log::error!("Failed to access World: EcsPlugin is missing");
    log::info!("Kon Engine stopped");
    std::process::exit(1);
}
