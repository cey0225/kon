//! Storage Layout Demo - Showcase memory contiguity and SparseSet packing
//!
//! This demo illustrates how components are stored in contiguous memory blocks
//! and how the engine maintains high density even after entity deletions.

use kon::prelude::*;

#[component]
struct Position {
    x: f32,
    y: f32,
}
#[component]
struct Health(f32);
#[component]
struct Name(String);
#[component]
struct Static;

#[system]
fn setup(ctx: &mut Context) {
    println!("=== Storage Layout Demo: Memory Efficiency ===\n");

    // We store IDs in a local list to demonstrate a "game-like" deletion later
    let mut ids = Vec::new();

    for i in 0..3 {
        let id = ctx
            .world_mut()
            .spawn()
            .insert(Name(format!("Entity_{}", i)))
            .insert(Position {
                x: i as f32,
                y: i as f32,
            })
            .insert(Health(100.0))
            .insert(Static)
            .id(); // Getting the real ID, not manual Entity::new

        ids.push(id);
    }

    println!("[PHASE 1] Initial memory state (Dense & Contiguous):");
    ctx.world().dump_all_memory();

    // Storing the ID we want to delete for the next system
    // Usually you'd store this in a Resource or a Component,
    // but for the demo we'll just grab it again in the next system.
}

#[system]
fn fragmentation_test(ctx: &mut Context) {
    // Let's find "Entity_1" properly through a query, like a real game logic
    let mut target = None;
    ctx.world().select::<(Name,)>().each(|entity, (name,)| {
        if name.0 == "Entity_1" {
            target = Some(entity);
        }
    });

    if let Some(entity_to_remove) = target {
        println!(
            "\n[PHASE 2] Deleting {:?} ({}) to test packing...",
            entity_to_remove, "Entity_1"
        );
        ctx.world_mut().destroy(entity_to_remove);

        println!("(Notice how the last entity's data moves to fill the gap internally)\n");
    }

    ctx.world().dump_all_memory();
}

#[system]
fn final_check(ctx: &mut Context) {
    println!("\n=== Final World Inspection ===");
    ctx.world().inspect();

    println!("\n[FINAL MEMORY LAYOUT]");
    ctx.world().dump_all_memory();

    println!("\n[RESULT] Memory remains packed and cache-friendly.");
    ctx.quit();
}

fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(fragmentation_test)
        .add_system(final_check)
        .run();
}
