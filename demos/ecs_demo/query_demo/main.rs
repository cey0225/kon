//! Query Demo - Demonstrates tuple-based query syntax

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
struct Name(String);

#[system]
fn setup(ctx: &mut Context) {
    println!("=== Query Demo ===\n");

    // Entity with all components
    ctx.world_mut()
        .spawn()
        .insert(Name("Player".into()))
        .insert(Position { x: 0.0, y: 0.0 })
        .insert(Velocity { x: 1.0, y: 2.0 })
        .id();

    // Entity with only Position
    ctx.world_mut()
        .spawn()
        .insert(Name("Rock".into()))
        .insert(Position { x: 10.0, y: 10.0 })
        .id();

    // Entity with Position and Velocity
    ctx.world_mut()
        .spawn()
        .insert(Name("Enemy".into()))
        .insert(Position { x: 5.0, y: 5.0 })
        .insert(Velocity { x: -1.0, y: 0.0 })
        .id();

    println!("[SETUP] Created 3 entities\n");
    ctx.world().inspect();
    println!();
}

#[system]
fn query_single(ctx: &mut Context) {
    println!("[Single Component Query] - All entities with Position:");

    ctx.world().select::<(Position,)>().each(|entity, (pos,)| {
        println!("  {:?} at ({}, {})", entity, pos.x, pos.y);
    });

    println!();
}

#[system]
fn query_multiple(ctx: &mut Context) {
    println!("[Multi Component Query] - Entities with Position AND Velocity:");

    ctx.world()
        .select::<(Position, Velocity)>()
        .each(|entity, (pos, vel)| {
            println!(
                "  {:?} at ({}, {}) moving ({}, {})",
                entity, pos.x, pos.y, vel.x, vel.y
            );
        });

    println!();
}

#[system]
fn query_mutate(ctx: &mut Context) {
    println!("[Mutable Query] - Moving entities:");

    ctx.world_mut()
        .select_mut::<(Position, Velocity)>()
        .each(|entity, (pos, vel)| {
            let old_x = pos.x;
            let old_y = pos.y;
            pos.x += vel.x;
            pos.y += vel.y;
            println!(
                "  {:?} ({}, {}) → ({}, {})",
                entity, old_x, old_y, pos.x, pos.y
            );
        });

    println!();
}

#[system]
fn done(ctx: &mut Context) {
    println!("=== Final State ===");
    ctx.world().inspect();
    ctx.quit();
}

fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(query_single)
        .add_system(query_multiple)
        .add_system(query_mutate)
        .add_system(done)
        .run();
}
