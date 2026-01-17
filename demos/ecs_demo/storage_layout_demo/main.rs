//! Storage Layout Demo
//!
//! Demonstrates SparseSet memory layout and swap-remove behavior.
//! Uses `dump_all_memory()` to visualize contiguous memory and cache efficiency.
//! Only works in debug builds.

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

    let mut ids = Vec::new();

    for i in 0..3 {
        let id = ctx
            .world()
            .spawn()
            .insert(Name(format!("Entity_{}", i)))
            .insert(Position {
                x: i as f32,
                y: i as f32,
            })
            .insert(Health(100.0))
            .insert(Static)
            .id();

        ids.push(id);
    }

    println!("[PHASE 1] Initial memory state (Dense & Contiguous):");
    ctx.world().dump_all_memory();
}

#[system]
fn fragmentation_test(ctx: &mut Context) {
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
        ctx.world().destroy(entity_to_remove);

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
        .add_plugin(EcsPlugin)
        .add_startup_system(setup)
        .add_system(fragmentation_test)
        .add_system(final_check)
        .run();
}
