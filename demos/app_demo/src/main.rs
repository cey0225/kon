use kon::prelude::*;

#[system]
fn setup(_ctx: &mut Context) {
    println!("Hello");
}

#[system]
fn update(ctx: &mut Context) {
    if ctx.time.frame_count() == 100 {
        ctx.quit();
    }
}

fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}
