//! # Kon Core
//!
//! Core module for Kon Engine.

mod app;
mod context;
mod driver;
mod event;
mod plugin;
mod time;

pub use app::{App, Kon};
pub use context::{Context, Globals};
pub use event::{AppExit, Event, Events};
pub use plugin::Plugin;
pub use time::Time;
pub use driver::{DefaultDriver, Driver};

pub mod prelude {
    pub use crate::{App, Context, Event, Events, Kon, Plugin, Time, Driver};
}
