# kon_ecs

**kon_ecs** is the ECS implementation for Kon Engine, built around SparseSet storage for fast iteration.

## Technical Highlights

- **Bitmask Tagging:** O(1) entity filtering with bit operations.
- **Zero-Allocation Iteration:** Queries don't allocate - they iterate directly over contiguous memory.
- **SparseSet Storage:** Cache-friendly component storage.
- **Standalone Core:** Can be used without the rest of Kon Engine.

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
    world.spawn()
        .insert(Position { x: 0.0, y: 0.0 })
        .insert(Velocity { x: 1.0, y: 1.0 })
        .id();

    // Query and iterate
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
