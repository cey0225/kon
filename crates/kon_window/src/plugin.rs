use kon_core::{App, Plugin};
use crate::WindowDriver;

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.set_driver(WindowDriver);
    }
}
