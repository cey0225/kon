# Kon Engine 🦀

A modular 2D game engine in Rust with a custom SparseSet-based ECS. Built from scratch to learn how game engines actually work.

> ⚠️ **Heads up:** This is an experimental/educational project. I'm building this to understand engine internals, not to compete with existing engines.

## What's Inside

The project is split into workspace crates:

**Currently Working:**
- **`kon_core`** - App lifecycle, plugins, events
- **`kon_ecs`** - Custom ECS
- **`kon_macros`** - Proc macros for `#[system]` and `#[component]`

**Still Cooking 🚧:**
- **`kon_math`** - Math stuff with Glam integration (WIP)
- **`kon_window`** - Winit-based window management (WIP)
- **`kon_input`** - Input handling (WIP)
- **`kon_renderer`** - WGPU renderer (WIP)
- **`kon_physics`** - 2D physics (Planned)
- **`kon_editor`** - Editor tools (Planned)

## Features

- [x] Plugin-based architecture
- [x] Custom SparseSet ECS with O(1) component access
- [x] Write systems as regular Rust functions
- [x] Type-safe queries like `Query<(Pos, Vel)>`
- [x] Event system for decoupled communication
- [ ] Window context management
- [ ] Keyboard/mouse input
- [ ] Hardware-accelerated 2D rendering
- [ ] Collision detection and physics
- [ ] Integrated editor

## Performance Analysis

Below are flamegraphs from stress tests showing where the engine spends its time under heavy load.

### ECS Stress Test (100k Entities)
This test runs 100,000 entities through multiple systems each frame. Most CPU time is spent in actual component logic rather than query overhead, which shows the zero-allocation iteration is working as intended.

![ECS Stress Test](assets/ecs_stress_flamegraph.svg)

[View interactive flamegraph](https://raw.githubusercontent.com/cey0225/kon/refs/heads/main/assets/ecs_stress_flamegraph.svg)

### Heavy Component Bottleneck Test
This test uses 10,000 entities with large components (100 floats each). The results are memory-bound rather than logic-bound, which means SparseSet is doing its job keeping data contiguous.

![Bottleneck Test](assets/bottleneck_test_flamegraph.svg)

[View interactive flamegraph](https://raw.githubusercontent.com/cey0225/kon/refs/heads/main/assets/bottleneck_test_flamegraph.svg)

## Quick Example

Here's how you write a simple simulation:

```rust
use kon::prelude::*;

// Define your data
#[component]
struct Position { x: f32, y: f32 }

#[component]
struct Velocity { x: f32, y: f32 }

// Setup runs once
#[system]
fn setup(ctx: &mut Context) {
    ctx.world_mut()
        .spawn()
        .insert(Position { x: 0.0, y: 0.0 })
        .insert(Velocity { x: 1.0, y: 0.0 })
        .tag("player")
        .id();
}

// Movement runs every frame
#[system]
fn movement(ctx: &mut Context) {
    ctx.world_mut()
        .select_mut::<(Position, Velocity)>()
        .each(|entity, (pos, vel)| {
            pos.x += vel.x;
            pos.y += vel.y;
            println!("{:?} moved to ({}, {})", entity, pos.x, pos.y);
        });
}

// Update runs every frame
#[system]
fn update(ctx: &mut Context) {
    if ctx.time.frame_count() == 60 {
        ctx.quit();
    }
}

// Wire everything together
fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(movement)
        .add_system(update)
        .run();
}
```

## How to Use

**As a dependency:**

```bash
cargo add kon-engine
```

Or in your `Cargo.toml`:

```toml
[dependencies]
kon-engine = "0.1.4"
```

**From source:**

```bash
git clone https://github.com/cey0225/kon.git
cd kon

# Run examples
./kon.sh ecs_demo/query_demo
./kon.sh ecs_demo/tag_demo
```

## License

Dual-licensed under MIT or Apache 2.0, pick whichever works for you.

- MIT: [LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT
- Apache 2.0: [LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0
