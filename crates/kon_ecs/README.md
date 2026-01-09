# kon_ecs

**kon_ecs** is a high-performance SparseSet-based Entity Component System (ECS) core. While it serves as the heart of the Kon Engine, its core architecture is designed to be standalone-ready.

## Technical Highlights

- **Bitmask Tagging:** O(1) entity filtering for lightning-fast queries.
- **Zero-Allocation Iteration:** Optimized internal slices to ensure no memory overhead during systems execution.
- **SparseSet Storage:** High-speed, cache-friendly component management.
- **Standalone Core:** Essential ECS logic is decoupled from engine-specific modules.

```bash
# Use it as part of the engine
cargo add kon-engine

# Or use the ECS core directly
cargo add kon_ecs
```

## Quick Start (Standalone)

You can use the ECS core independently from the engine.

```rust
use kon_ecs::World;

// Define your components
#[derive(Debug, Clone, PartialEq)]
struct Position { x: f32, y: f32 }

#[derive(Debug, Clone, PartialEq)]
struct Velocity { x: f32, y: f32 }

fn main() {
    let mut world = World::new();

    // Spawn entities with components
    world.spawn().insert(Position { x: 0.0, y: 0.0 }).insert(Velocity { x: 1.0, y: 1.0 }));

    // Query and iterate (O(1) filtering)
    world.select_mut::<(Position, Velocity)>().each(|_, (pos, vel)| {
        pos.x += vel.x;
        pos.y += vel.y;
    });
}
```

## Component Definition

You can define components in two ways:

1. **Manual (No macro required):** Simply use your structs as components.
2. **With Macros (Optional):** Use the `kon_macros` crate for a cleaner syntax.

```rust
#[component] // Optional with kon_macros
struct Position { x: f32, y: f32 }
```

## License

MIT OR Apache-2.0
