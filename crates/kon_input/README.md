# kon_input

**kon_input** is the input handling module for Kon Engine. Provides unified keyboard, mouse, and action binding support with frame-accurate state tracking.

## Scope

- **Bitmask State Storage:** 256-bit array for tracking all input states efficiently.
- **Frame-Accurate Detection:** Distinguishes between pressed, just pressed, and just released.
- **Action Bindings:** Map multiple input sources to named actions.
- **Chord Support:** Key+Key and Key+Mouse combinations.

```bash
# Part of the kon-engine ecosystem
cargo add kon-engine
```

## Quick Start (With Engine)

```rust
use kon::prelude::*;

#[system]
fn player_control(ctx: &mut Context) {
    let input = ctx.input();

    // Raw key/button queries
    if input.is_key_pressed(KeyCode::Space) {
        println!("Space held");
    }

    if input.just_button_pressed(MouseButton::Left) {
        println!("Left click!");
    }

    // Action-based queries (recommended)
    if input.just_action_pressed("Jump") {
        println!("Jump triggered");
    }

    if input.is_action_pressed("MoveForward") {
        println!("Moving forward...");
    }
}

fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_system(player_control)
        .run();
}
```

## Custom Bindings

The default bindings cover common game actions, but you can add your own:

```rust
#[system]
fn setup_bindings(ctx: &mut Context) {
    let mut input = ctx.input();

    // Single key
    input.add_binding("Crouch", InputSource::Key(KeyCode::LControl));

    // Multiple keys for same action
    input.add_binding("Interact", InputSource::Key(KeyCode::E));
    input.add_binding("Interact", InputSource::Key(KeyCode::F));

    // Mouse button
    input.add_binding("Aim", InputSource::Mouse(MouseButton::Right));

    // Key + Key chord
    input.add_binding("QuickSave", InputSource::Chord(KeyCode::LControl, KeyCode::S));

    // Key + Mouse chord
    input.add_binding("ZoomShoot", InputSource::MouseChord(KeyCode::LShift, MouseButton::Left));
}
```

## Default Bindings

| Action | Default Binding |
|--------|-----------------|
| `MoveForward` | W, Up Arrow |
| `MoveBackward` | S, Down Arrow |
| `Jump` | Space |
| `Fire` | Left Mouse |
| `Sprint` | Left Shift |
| `SpecialSkill` | Left Shift + Right Mouse |

## License

MIT OR Apache-2.0
