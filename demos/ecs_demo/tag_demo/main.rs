//! Tag Demo - Demonstrates tag-based filtering

use kon::prelude::*;

#[component]
struct Health(i32);

#[component]
struct Damage(i32);

#[system]
fn setup(ctx: &mut Context) {
    println!("=== Tag Demo ===\n");

    // Player
    ctx.world_mut()
        .spawn()
        .insert(Health(100))
        .insert(Damage(10))
        .tag("player")
        .tag("friendly")
        .id();

    // Friendly NPC
    ctx.world_mut()
        .spawn()
        .insert(Health(50))
        .tag("npc")
        .tag("friendly")
        .id();

    // Enemy
    ctx.world_mut()
        .spawn()
        .insert(Health(30))
        .insert(Damage(5))
        .tag("enemy")
        .id();

    // Frozen enemy (can't act)
    ctx.world_mut()
        .spawn()
        .insert(Health(80))
        .insert(Damage(15))
        .tag("enemy")
        .tag("frozen")
        .id();

    println!(
        "[SETUP] Created 1 player&friendly + 1 npc&friendly + 1 enemy + 1 enemy&frozen entities\n"
    );
    ctx.world().inspect();
    println!();
}

#[system]
fn show_friendlies(ctx: &mut Context) {
    println!("[tagged(\"friendly\")] - Friendly entities:");

    ctx.world()
        .select::<(Health,)>()
        .tagged("friendly")
        .each(|entity, (hp,)| {
            println!("  {:?} hp: {}", entity, hp.0);
        });

    println!();
}

#[system]
fn show_enemies(ctx: &mut Context) {
    println!("[tagged(\"enemy\")] - Enemy entities:");

    ctx.world()
        .select::<(Health,)>()
        .tagged("enemy")
        .each(|entity, (hp,)| {
            println!("  {:?} hp: {}", entity, hp.0);
        });

    println!();
}

#[system]
fn show_active_enemies(ctx: &mut Context) {
    println!("[tagged(\"enemy\").not_tagged(\"frozen\")] - Active enemies:");

    ctx.world()
        .select::<(Health, Damage)>()
        .tagged("enemy")
        .not_tagged("frozen")
        .each(|entity, (hp, dmg)| {
            println!("  {:?} hp: {}, damage: {}", entity, hp.0, dmg.0);
        });

    println!();
}

#[system]
fn damage_non_friendly(ctx: &mut Context) {
    println!("[not_tagged(\"friendly\")] - Damaging non-friendly entities:");

    ctx.world_mut()
        .select_mut::<(Health,)>()
        .not_tagged("friendly")
        .each(|entity, (hp,)| {
            let old = hp.0;
            hp.0 -= 10;
            println!("  {:?} hp: {} → {}", entity, old, hp.0);
        });

    println!();
}

#[system]
fn done(ctx: &mut Context) {
    ctx.quit();
}

fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(show_friendlies)
        .add_system(show_enemies)
        .add_system(show_active_enemies)
        .add_system(damage_non_friendly)
        .add_system(done)
        .run();
}
