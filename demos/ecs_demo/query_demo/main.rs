//! Query Demo
//!
//! Demonstrates advanced query filtering with `.with()` and `.without()`.
//! Shows how to filter entities by component presence without fetching them.

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

#[component]
struct Name(String);

#[component]
struct Armor;

#[system]
fn setup(ctx: &mut Context) {
    println!("=== Query Demo: Advanced Filtering ===\n");

    ctx.world()
        .spawn()
        .insert(Name("Player".into()))
        .insert(Position { x: 0.0, y: 0.0 })
        .insert(Velocity { x: 1.0, y: 1.0 })
        .insert(Health(100.0))
        .insert(Armor)
        .id();

    ctx.world()
        .spawn()
        .insert(Name("Enemy".into()))
        .insert(Position { x: 5.0, y: 5.0 })
        .insert(Velocity { x: -1.0, y: 0.0 })
        .insert(Health(100.0))
        .id();

    ctx.world()
        .spawn()
        .insert(Name("Rock".into()))
        .insert(Position { x: 10.0, y: 10.0 })
        .id();

    println!("[SETUP] Entities initialized\n");
    ctx.world().inspect();
}

#[system]
fn movement_system(ctx: &mut Context) {
    println!("[Movement] Updating moving entities:");

    ctx.world()
        .select_mut::<(Position, Velocity)>()
        .each(|entity, (pos, vel)| {
            pos.x += vel.x;
            pos.y += vel.y;
            println!("  {:?} moved to ({:.1}, {:.1})", entity, pos.x, pos.y);
        });
    println!();
}

#[system]
fn combat_system(ctx: &mut Context) {
    println!("[Combat] Applying environmental damage:");

    ctx.world()
        .select_mut::<(Health,)>()
        .without::<Armor>()
        .each(|entity, (hp,)| {
            hp.0 -= 20.0;
            println!("  {:?} (No Armor) took 20 damage. HP: {:.1}", entity, hp.0);
        });

    ctx.world()
        .select_mut::<(Health,)>()
        .with::<Armor>()
        .each(|entity, (hp,)| {
            hp.0 -= 5.0;
            println!("  {:?} (Armored) took 5 damage. HP: {:.1}", entity, hp.0);
        });
    println!();
}

#[system]
fn debug_system(ctx: &mut Context) {
    println!("[Filter Debug] Entities with Name but no Velocity (Static):");

    ctx.world()
        .select::<(Name,)>()
        .without::<Velocity>()
        .each(|entity, (name,)| {
            println!("  {:?} is static: {}", entity, name.0);
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
        .add_plugin(EcsPlugin)
        .add_startup_system(setup)
        .add_system(movement_system)
        .add_system(combat_system)
        .add_system(debug_system)
        .add_system(done)
        .run();
}
