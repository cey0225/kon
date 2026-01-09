//! ECS Stress Test - Pure profiling binary for Flamegraph analysis
//!
//! This binary performs heavy ECS operations without any benchmarking overhead
//! to provide clean profiling data for memory access and iteration patterns.

use kon::prelude::*;

#[component]
struct Position {
    x: f32,
    y: f32,
}

#[component]
struct Velocity {
    x: f32,
    y: f32,
}

#[component]
struct Health(f32);

#[system]
fn setup(ctx: &mut Context) {
    println!("Spawning 100,000 entities for profiling...");

    for i in 0..100_000 {
        let world = ctx.world_mut();
        let entity = world.spawn().id();

        world.insert(
            entity,
            Position {
                x: i as f32,
                y: i as f32,
            },
        );
        world.insert(entity, Velocity { x: 1.0, y: 1.0 });

        if i % 2 == 0 {
            world.insert(entity, Health(100.0));
        }
    }
    println!("Setup complete. Running stress systems...");
}

#[system]
fn stress_movement_system(ctx: &mut Context) {
    ctx.world_mut()
        .select_mut::<(Position, Velocity)>()
        .each(|_entity, (pos, vel)| {
            pos.x += vel.x;
            pos.y += vel.y;
        });
}

#[system]
fn stress_health_system(ctx: &mut Context) {
    ctx.world_mut()
        .select_mut::<(Health,)>()
        .each(|_entity, (hp,)| {
            hp.0 -= 1.0;
        });
}

#[system]
fn frame_counter(ctx: &mut Context) {
    if ctx.time.frame_count() > 1000 {
        println!("Stress test finished (1000 frames). Quitting...");
        ctx.quit();
    }
}

fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(stress_movement_system)
        .add_system(stress_health_system)
        .add_system(frame_counter)
        .run();
}
