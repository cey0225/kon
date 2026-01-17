use log::LevelFilter;
use crate::{Context, DefaultDriver, Driver, Plugin};

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
    /// System that run at the end of every frame
    sync_systems: Vec<SystemFn>,
    /// Registered plugins
    plugins: Vec<Box<dyn Plugin>>,
    /// Custom game loop driver (defaults to DefaultDriver)
    driver: Option<Box<dyn Driver>>,
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
        let _ = env_logger::Builder::new()
            .filter_level(LevelFilter::Error)
            .filter_module("kon", LevelFilter::Debug)
            .try_init();

        install_panic_hook();

        Self {
            context: Context::new(),
            startup_systems: Vec::new(),
            systems: Vec::new(),
            sync_systems: Vec::new(),
            plugins: Vec::new(),
            driver: Some(Box::new(DefaultDriver)),
        }
    }

    /// Sets a custom driver for the game loop
    ///
    /// Replaces the default driver with a custom implementation. This allows
    /// control over how the application lifecycle executes (e.g., fixed timestep,
    /// event-driven loops, or headless simulation).
    ///
    /// Must be called before `run()`. If not set, `DefaultDriver` is used.
    ///
    /// # Returns
    /// Self reference for method chaining
    ///
    /// # Example
    /// ```ignore
    /// Kon::new()
    ///     .set_driver(CustomDriver)
    ///     .run();
    /// ```
    pub fn set_driver<D: Driver + 'static>(&mut self, driver: D) -> &mut Self {
        self.driver = Some(Box::new(driver));
        self
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

    /// Adds a system that runs at the end of every frame
    ///
    /// # Returns
    /// Self reference for method chaining
    pub fn add_sync_system<F>(&mut self, system: F) -> &mut Self
    where
        F: FnMut(&mut Context) + 'static,
    {
        self.sync_systems.push(Box::new(system));
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

    /// Initializes the application
    ///
    /// Called automatically by the driver. This method:
    /// 1. Calls `ready()` on all registered plugins
    /// 2. Executes all startup systems once
    ///
    /// Should not be called manually unless implementing a custom driver.
    pub fn initialize(&mut self) {
        let plugin_count = self.plugins.iter().filter(|p| !p.is_plugin_group()).count();
        log::debug!("Calling ready() on {} plugin(s)", plugin_count);

        for plugin in &self.plugins {
            plugin.ready(&mut self.context);
        }

        log::debug!("Registered {} active system(s)", self.systems.len());

        log::debug!("Executed {} startup system(s)", self.startup_systems.len());
        for system in &mut self.startup_systems {
            system(&mut self.context);
        }
    }

    /// Executes a single frame update
    ///
    /// Called automatically by the driver each frame. This method:
    /// 1. Updates time tracking
    /// 2. Runs all registered systems
    /// 3. Clears frame events
    ///
    /// Should not be called manually unless implementing a custom driver.
    pub fn tick(&mut self) {
        self.context.time.update();

        for system in &mut self.systems {
            system(&mut self.context);
        }

        for sync_system in &mut self.sync_systems {
            sync_system(&mut self.context);
        }

        self.context.events.clear_all();
    }

    /// Cleans up the application
    ///
    /// Called automatically by the driver on exit. This method calls `cleanup()`
    /// on all registered plugins, allowing them to release resources.
    ///
    /// Should not be called manually unless implementing a custom driver.
    pub fn cleanup(&mut self) {
        let plugin_count = self.plugins.iter().filter(|p| !p.is_plugin_group()).count();
        log::debug!("Cleaning up {} plugin(s)", plugin_count);

        for plugin in &self.plugins {
            plugin.cleanup(&mut self.context);
        }
    }

    /// Starts the application and runs the game loop
    ///
    /// This is the entry point for executing the engine. It:
    /// 1. Transfers ownership of the app to the configured driver
    /// 2. The driver handles initialization, update loop, and cleanup
    /// 3. Blocks until the application exits
    ///
    /// The driver is consumed during execution. If no custom driver was set
    /// via `set_driver()`, the `DefaultDriver` is used.
    ///
    /// # Example
    /// ```ignore
    /// Kon::new()
    ///     .add_plugin(DefaultPlugins)
    ///     .add_system(update_system)
    ///     .run();  // Blocks here until exit
    /// ```
    #[track_caller]
    pub fn run(&mut self) {
        log::info!("Kon Engine initialized");

        if let Some(driver) = self.driver.take() {
            driver.drive(std::mem::take(self));
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
