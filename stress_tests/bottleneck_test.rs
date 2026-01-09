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
struct Complex {
    data: [f32; 100],
}

fn main() {
    let mut world = World::new();

    println!("Test 1: Simple query");
    for i in 0..50_000 {
        let e = world.spawn().id();
        world.insert(
            e,
            Position {
                x: i as f32,
                y: 0.0,
            },
        );
        world.insert(e, Velocity { x: 1.0, y: 1.0 });
    }

    for _ in 0..500 {
        world
            .select_mut::<(Position, Velocity)>()
            .each(|_, (pos, vel)| {
                pos.x += vel.x;
            });
    }

    println!("Test 2: Heavy component");
    for i in 0..10_000 {
        let e = world.spawn().id();
        world.insert(
            e,
            Complex {
                data: [i as f32; 100],
            },
        );
    }

    for _ in 0..500 {
        world.select_mut::<(Complex,)>().each(|_, (c,)| {
            c.data[0] += 1.0;
        });
    }
}
