use crate::{Context, Plugin};

/// System function type
pub type SystemFn = Box<dyn FnMut(&mut Context)>;

/// Main application struct that manages the game loop
///
/// # Example
/// ```ignore
/// Kon::new()
///     .add_plugin(DefaultPlugins)
///     .add_startup_system(setup)
///     .add_system(update)
///     .run();
/// ```
pub struct App {
    context: Context,
    startup_systems: Vec<SystemFn>,
    systems: Vec<SystemFn>,
    plugins: Vec<Box<dyn Plugin>>,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    /// Creates a new App instance
    pub fn new() -> Self {
        let _ =
            env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug"))
                .try_init();

        Self {
            context: Context::new(),
            startup_systems: Vec::new(),
            systems: Vec::new(),
            plugins: Vec::new(),
        }
    }

    /// Adds a plugin to the application
    pub fn add_plugin<P: Plugin>(&mut self, plugin: P) -> &mut Self {
        log::debug!("Added plugin: {}", plugin.name());
        plugin.build(self);
        self.plugins.push(Box::new(plugin));
        self
    }

    /// Adds a startup system (runs once at start)
    pub fn add_startup_system<F>(&mut self, system: F) -> &mut Self
    where
        F: FnMut(&mut Context) + 'static,
    {
        self.startup_systems.push(Box::new(system));
        self
    }

    /// Adds a system (runs every frame)
    pub fn add_system<F>(&mut self, system: F) -> &mut Self
    where
        F: FnMut(&mut Context) + 'static,
    {
        self.systems.push(Box::new(system));
        self
    }

    /// Registers a global state
    pub fn register<R: std::any::Any + Send + Sync + 'static>(&mut self, resource: R) -> &mut Self {
        self.context.register(resource);
        self
    }

    /// Returns a reference to the Context
    pub fn context(&self) -> &Context {
        &self.context
    }

    /// Returns a mutable reference to the Context
    pub fn context_mut(&mut self) -> &mut Context {
        &mut self.context
    }

    /// Runs the application main loop
    pub fn run(&mut self) {
        log::info!("Kon Engine initialized");

        // Startup systems
        log::debug!("Executed {} startup system(s)", self.startup_systems.len());
        for system in &mut self.startup_systems {
            system(&mut self.context);
        }

        // Main loop
        log::debug!("Registered {} active system(s)", self.systems.len());

        while self.context.is_running() {
            self.context.time.update();

            for system in &mut self.systems {
                system(&mut self.context);
            }

            self.context.events.clear_all();

            // Stop after 100 frames for testing
            /*if self.context.time.frame_count() >= 100 {
                log::info!("Test completed (100 frames)");
                self.context.quit();
            }*/
        }

        log::info!("Kon Engine stopped");
    }
}

pub type Kon = App;
