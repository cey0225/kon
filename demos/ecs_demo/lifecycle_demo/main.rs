//! Lifecycle Demo
//!
//! Demonstrates entity spawning, destruction, and deferred operations.
//! Shows how entities can have lifetimes and be destroyed automatically.

use kon::prelude::*;

#[component]
struct Health(i32);

#[component]
struct Lifetime(u32);

#[system]
fn setup(ctx: &mut Context) {
    println!("=== Lifecycle Demo ===\n");

    ctx.world_mut()
        .spawn()
        .insert(Health(100))
        .tag("permanent")
        .id();

    for i in 1..=3 {
        ctx.world_mut()
            .spawn()
            .insert(Health(10 * i))
            .insert(Lifetime(i as u32))
            .tag("temporary")
            .id();
    }

    println!("[SETUP] Created 1 permanent + 3 temporary entities\n");
    ctx.world().inspect();
    println!();
}

#[system]
fn tick_lifetime(ctx: &mut Context) {
    let frame = ctx.time.frame_count() as u32;
    println!("[FRAME {}] Checking lifetimes...", frame);

    let mut to_destroy = Vec::new();

    ctx.world()
        .select::<(Lifetime,)>()
        .each(|entity, (lifetime,)| {
            if frame >= lifetime.0 {
                println!("  {:?} expired (lifetime was {})", entity, lifetime.0);
                to_destroy.push(entity);
            } else {
                println!("  {:?} alive ({} frames left)", entity, lifetime.0 - frame);
            }
        });

    for entity in to_destroy {
        ctx.world_mut().destroy(entity);
        println!("  {:?} destroyed!", entity);
    }

    println!();
}

#[system]
fn spawn_on_frame_2(ctx: &mut Context) {
    if ctx.time.frame_count() == 2 {
        println!("[SPAWN] Creating new entity on frame 2");

        let entity = ctx
            .world_mut()
            .spawn()
            .insert(Health(999))
            .insert(Lifetime(4))
            .tag("late_spawned")
            .id();

        println!("  Created {:?}\n", entity);
    }
}

#[system]
fn show_alive(ctx: &mut Context) {
    let count = ctx.world().entity_count();
    println!("[ALIVE] {} entities remain", count);

    ctx.world().select::<(Health,)>().each(|entity, (hp,)| {
        println!("  {:?} hp: {}", entity, hp.0);
    });

    println!();
}

#[system]
fn done(ctx: &mut Context) {
    if ctx.time.frame_count() >= 5 {
        println!("=== Final State ===");
        ctx.world().inspect();
        ctx.quit();
    }
}

fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(tick_lifetime)
        .add_system(spawn_on_frame_2)
        .add_system(show_alive)
        .add_system(done)
        .run();
}
