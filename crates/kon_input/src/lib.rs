//! #[system]
//! fn player_control(ctx: &mut Context) {
//!     let input = ctx.input();
//!
//!     // Raw input
//!     if input.just_key_pressed(KeyCode::Space) {
//!         jump();
//!     }
//!
//!     // Action-based (recommended)
//!     if input.is_action_pressed("MoveForward") {
//!         move_forward();
//!     }
//!
//!     if input.just_action_pressed("Fire") {
//!         shoot();
//!     }
//! }
//!
//! fn main() {
//!     Kon::new()
//!         .add_plugin(DefaultPlugins)
//!         .add_system(player_control)
//!         .run();
//! }
//! ```

mod ext;
mod input;
mod plugin;

pub use input::{Input, InputSource};
pub use plugin::InputPlugin;
pub use ext::ContextInputExt;

pub mod prelude {
    pub use crate::{InputPlugin, ContextInputExt, Input, InputSource};
}
