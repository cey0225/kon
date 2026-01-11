use crate::{Context, Plugin};

/// Function signature for system callbacks
///
/// Systems are functions that run every frame or once at startup.
/// They receive mutable access to the engine context.
pub type SystemFn = Box<dyn FnMut(&mut Context)>;

/// Main application struct that manages the game loop and plugin lifecycle
///
/// The `App` coordinates:
/// - Plugin registration and initialization
/// - System scheduling (startup and per-frame)
/// - Main loop execution
/// - Resource cleanup on shutdown
///
/// # Example
/// ```ignore
/// use kon::prelude::*;
///
/// fn main() {
///     Kon::new()
///         .add_plugin(DefaultPlugins)
///         .add_startup_system(setup)
///         .add_system(update)
///         .run();
/// ```
pub struct App {
    /// Shared engine context (time, events, globals)
    context: Context,
    /// Systems that run once at startup
    startup_systems: Vec<SystemFn>,
    /// Systems that run every frame
    systems: Vec<SystemFn>,
    /// Registered plugins
    plugins: Vec<Box<dyn Plugin>>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Creates a new App instance
    ///
    /// Initializes logging and installs custom panic handler
    pub fn new() -> Self {
        let _ =
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
                .try_init();

        install_panic_hook();

        Self {
            context: Context::new(),
            startup_systems: Vec::new(),
            systems: Vec::new(),
            plugins: Vec::new(),
        }
    }

    /// Adds a plugin to the application
    ///
    /// Plugins extend engine functionality. Common examples:
    /// - `EcsPlugin` - Registers the World
    /// - `WindowPlugin` - Creates the game window
    /// - `DefaultPlugins` - Bundle of core plugins
    ///
    /// # Returns
    /// Self reference for method chaining
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        log::debug!("Added plugin: {}", plugin.name());
        plugin.build(self);
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Adds a startup system that runs once at application start
    ///
    /// # Returns
    /// Self reference for method chaining
    pub fn add_startup_system<F>(&mut self, system: F) -> &mut Self
    where
        F: FnMut(&mut Context) + 'static,
    {
        self.startup_systems.push(Box::new(system));
        self
    }

    /// Adds a system that runs every frame
    ///
    /// # Returns
    /// Self reference for method chaining
    pub fn add_system<F>(&mut self, system: F) -> &mut Self
    where
        F: FnMut(&mut Context) + 'static,
    {
        self.systems.push(Box::new(system));
        self
    }

    /// Registers a global resource accessible from all systems
    ///
    /// Resources are stored in Context and accessible via `ctx.global::<T>()`.
    ///
    /// # Returns
    /// Self reference for method chaining
    pub fn register<R: std::any::Any + Send + Sync + 'static>(&mut self, resource: R) -> &mut Self {
        self.context.register(resource);
        self
    }

    /// Returns an immutable reference to the engine context
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Returns a mutable reference to the engine context
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    /// Runs the application main loop
    ///
    /// # Lifecycle
    /// 1. Initialize all plugins (call `ready()`)
    /// 2. Execute startup systems once
    /// 3. Run main loop until `ctx.quit()` is called:
    ///     - Update frame timing
    ///     - Execute all systems in order
    ///     - Clear events (prevents stale data next frame)
    /// 4. Cleanup plugins on exit
    pub fn run(&mut self) {
        log::info!("Kon Engine initialized");

        let plugin_count = self.plugins.iter().filter(|p| !p.is_plugin_group()).count();

        // Initialize plugins
        log::debug!("Calling ready() on {} plugin(s)", plugin_count);
        for plugin in &self.plugins {
            plugin.ready(&mut self.context);
        }

        // Run startup systems once
        log::debug!("Executed {} startup system(s)", self.startup_systems.len());
        for system in &mut self.startup_systems {
            system(&mut self.context);
        }

        log::debug!("Registered {} active system(s)", self.systems.len());

        // Main loop - runs until context.quit() is called
        while self.context.is_running() {
            self.context.time.update();

            for system in &mut self.systems {
                system(&mut self.context);
            }

            // Clear events at end of frame
            self.context.events.clear_all();
        }

        // Cleanup phase
        log::debug!("Cleaning up {} plugin(s)", plugin_count);
        for plugin in &self.plugins {
            plugin.cleanup(&mut self.context);
        }

        log::info!("Kon Engine stopped");
    }
}

/// Type alias for App - shorter name for convenience
pub type Kon = App;

/// Installs a custom panic handler for better error reporting
///
/// The handler extracts:
/// - Panic message and payload
/// - File location (file:line:column)
/// - Thread name and platform info
///
/// Only runs once via `Once` guard. Called automatically in `App::new()`.
fn install_panic_hook() {
    use std::sync::Once;
    static START: Once = Once::new();

    START.call_once(|| {
        std::panic::set_hook(Box::new(|info| {
            let msg = if let Some(s) = info.payload().downcast_ref::<&str>() {
                *s
            } else if let Some(s) = info.payload().downcast_ref::<String>() {
                &**s
            } else {
                "Unknown engine error"
            };

            let location = if let Some(location) = info.location() {
                format!(
                    "{}:{}:{}",
                    location.file(),
                    location.line(),
                    location.column()
                )
            } else {
                "Unknown location".to_string()
            };

            let thread = std::thread::current();
            let thread_name = thread.name().unwrap_or("unknown");

            log::error!("{}", msg);
            log::error!("  at {}", location);
            log::debug!(
                "  thread: '{}', platform: {}-{}",
                thread_name,
                std::env::consts::OS,
                std::env::consts::ARCH
            );

            eprintln!("\n\x1b[31mKon Engine process terminated.\x1b[0m\n");
        }));
    });
}
