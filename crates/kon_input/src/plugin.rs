//! Input Plugin for registering Input as a global resource
//!
//! Handles window events and updates input state each frame.

use kon_core::{
    Context, Plugin,
    events::{KeyboardInput, MouseButtonInput, MouseMotion, MousePosition, MouseWheel},
};
use crate::{ContextInputExt, Input};

/// Input Plugin - registers Input and processes input events
///
/// This plugin:
/// - Creates and registers `Input` in Context with default bindings
/// - Subscribes to keyboard and mouse events from the window
/// - Syncs input state at frame boundaries
///
/// Required for using `ctx.input()`.
///
/// # Event Handling
/// Listens to these events from `kon_window`:
/// - `KeyboardInput`: Key press/release
/// - `MouseButtonInput`: Mouse button press/release
/// - `MousePosition`: Cursor position updates
/// - `MouseMotion`: Raw mouse movement delta
/// - `MouseWheel`: Scroll wheel movement
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut kon_core::App) {
        app.register(Input::default());
        app.add_system(input_system);
        app.add_sync_system(input_sync_system);
    }
}

/// Processes input events and updates Input state
fn input_system(ctx: &mut Context) {
    ctx.on::<KeyboardInput>(|event, context| {
        context.input().set_key(event.key, event.state);
    });

    ctx.on::<MouseButtonInput>(|event, context| {
        context.input().set_button(event.button, event.state);
    });

    ctx.on::<MousePosition>(|event, context| {
        context.input().set_mouse_position(event.x, event.y);
    });

    ctx.on::<MouseWheel>(|event, context| {
        context
            .input()
            .set_mouse_wheel(event.delta_x, event.delta_y);
    });

    ctx.on::<MouseMotion>(|event, context| {
        context
            .input()
            .add_mouse_motion(event.delta_x, event.delta_y);
    });
}

/// Syncs input state at frame end
///
/// Copies current state to previous for edge detection.
/// Resets per-frame accumulators (mouse motion, wheel).
fn input_sync_system(ctx: &mut Context) {
    ctx.input().sync();
}
