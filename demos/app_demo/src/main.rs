use std::{path::PathBuf, str::FromStr};
use kon::prelude::*;

#[system]
fn setup(ctx: &mut Context) {
    ctx.window().set_config(
        WindowConfig::default().with_icon(PathBuf::from_str("assets/kon_app.png").ok()),
    );

    println!("Hello");
}

#[system]
fn update(ctx: &mut Context) {
    ctx.on::<WindowCloseRequested>(|_, context| {
        context.quit();
    });
}

fn main() {
    Kon::new()
        .add_plugin(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(update)
        .run();
}
