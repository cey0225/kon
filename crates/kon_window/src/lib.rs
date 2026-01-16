//! Window management for Kon Engine
//!
//! This crate provides cross-platform window creation and management built on top of winit.
//! It handles window lifecycle, events, and provides a simplified API for common operations.
//!
//! # Features
//!
//! - Window creation with customizable configuration
//! - Fullscreen support (borderless and exclusive)
//! - Window state management (size, position, visibility)
//! - Event handling integrated with Kon's event system
//! - Custom game loop drivers via `WindowDriver`
//!
//! # Example
//! ```ignore
//! use kon::prelude::*;
//!
//! fn main() {
//!     Kon::new()
//!         .add_plugin(WindowPlugin) // DefaultPlugins
//!         .run();
//! }
//! ```

mod config;
mod driver;
mod ext;
mod plugin;
pub mod types;
mod window;
mod window_backend;

pub use plugin::WindowPlugin;
pub use driver::WindowDriver;
pub use config::WindowConfig;
pub use window::KonWindow;
pub use ext::ContextWindowExt;
pub(crate) use window_backend::WindowBackend;

pub mod prelude {
    pub use crate::{WindowPlugin, WindowDriver, WindowConfig, KonWindow, ContextWindowExt};
    pub use crate::types::*;
}
