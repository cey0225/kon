use crate::{Events, Time};
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Type-erased storage for engine-wide resources
///
/// Stores any type implementing `Any + Send + Sync` using TypeId as key.
/// Common use cases:
/// - `World` - ECS state (registered by EcsPlugin)
/// - `Input` - Keyboard/mouse state
/// - `Window` - Window handle
/// - Custom game state
///
/// # Example
/// ```ignore
/// #[derive(Default)]
/// struct GameConfig {
///     difficulty: u32,
/// }
///
/// // Register
/// ctx.register(GameConfig { difficulty: 2 });
///
/// // Read
/// let config = ctx.global::<GameConfig>().unwrap();
/// println!("Difficulty: {}", config.difficulty);
///
/// // Write
/// ctx.global_mut::<GameConfig>().unwrap().difficulty = 3;
/// ```
#[derive(Default)]
pub struct Globals {
    /// TypeId -> boxed resource mapping
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Globals {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a new global resource
    ///
    /// If a resource of this type already exists, it will be replaced.
    pub fn register<G: Any + Send + Sync + 'static>(&mut self, global: G) {
        self.data.insert(TypeId::of::<G>(), Box::new(global));
    }

    /// Gets an immutable reference to a global resource
    ///
    /// Returns `None` if type not registered
    pub fn get<G: Any + Send + Sync + 'static>(&self) -> Option<&G> {
        self.data
            .get(&TypeId::of::<G>())
            .and_then(|g| g.downcast_ref())
    }

    /// Gets a mutable reference to a global resource
    ///
    /// Returns `None` if type not registered
    pub fn get_mut<G: Any + Send + Sync + 'static>(&mut self) -> Option<&mut G> {
        self.data
            .get_mut(&TypeId::of::<G>())
            .and_then(|g| g.downcast_mut())
    }

    /// Checks if a global resource type is registered
    pub fn contains<G: Any + Send + Sync + 'static>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<G>())
    }

    /// Removes and returns a global resource
    ///
    /// Returns `None` if type not registered
    pub fn remove<G: Any + Send + Sync + 'static>(&mut self) -> Option<G> {
        self.data
            .remove(&TypeId::of::<G>())
            .and_then(|g| g.downcast().ok())
            .map(|g| *g)
    }
}

/// Main context passed to all systems
///
/// Centralizes engine state
/// - `time` - Frame timing (delta, fps, frame count)
/// - `events` - Event queue for inter-system communication
/// - `globals` - Type-erased resource storage
///
/// Access via system parameter:
/// ```ignore
/// #[system]
/// fn my_system(ctx: &mut Context) {
///     let delta = ctx.time.delta();
///
///     for event in ctx.events.read::<CollisionEvent>() {
///         // handle collision
///     }
///
///     let world = ctx.world_mut();
///     world.spawn().insert(Health(100));
/// }
/// ```
pub struct Context {
    /// Frame timing information
    pub time: Time,
    /// Event queue
    pub events: Events,
    /// Shared resource storage
    pub globals: Globals,
    /// Engine running state (false after quit() is called)
    running: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Creates a new Context with empty state
    pub fn new() -> Self {
        Self {
            time: Time::new(),
            events: Events::new(),
            globals: Globals::new(),
            running: true,
        }
    }

    /// Signals the engine to stop after the current frame completes
    ///
    /// The main loop will exit gracefully after all systems finish execution.
    pub fn quit(&mut self) {
        self.running = false;
        log::info!("Quit requested");
    }

    /// Returns true if the engine is still running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Registers a global resource (shorthand for `globals.register()`)
    ///
    /// If a resource of this type already exists, it will be replaced.
    pub fn register<G: Any + Send + Sync + 'static>(&mut self, global: G) {
        self.globals.register(global);
    }

    /// Gets an immutable reference to a global resource
    pub fn global<G: Any + Send + Sync + 'static>(&self) -> Option<&G> {
        self.globals.get()
    }

    /// Gets a mutable reference to a global resource
    pub fn global_mut<G: Any + Send + Sync + 'static>(&mut self) -> Option<&mut G> {
        self.globals.get_mut()
    }
}
