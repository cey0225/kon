use crate::{Events, Time};
use std::any::{Any, TypeId};
use std::collections::HashMap;

/// Global state storage for shared data across systems
///
/// Stores engine-wide data like World, Input, Window etc.
/// Access via `ctx.global::<T>()` or `ctx.global_mut::<T>()`.
///
/// # Example
/// ```ignore
/// // Register
/// ctx.register(MyState::new());
///
/// // Read
/// let state = ctx.global::<MyState>().unwrap();
///
/// // Write
/// ctx.global_mut::<MyState>().unwrap().update();
/// ```
///
#[derive(Default)]
pub struct Globals {
    data: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
}

impl Globals {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register<G: Any + Send + Sync + 'static>(&mut self, global: G) {
        self.data.insert(TypeId::of::<G>(), Box::new(global));
    }

    pub fn get<G: Any + Send + Sync + 'static>(&self) -> Option<&G> {
        self.data
            .get(&TypeId::of::<G>())
            .and_then(|g| g.downcast_ref())
    }

    pub fn get_mut<G: Any + Send + Sync + 'static>(&mut self) -> Option<&mut G> {
        self.data
            .get_mut(&TypeId::of::<G>())
            .and_then(|g| g.downcast_mut())
    }

    pub fn contains<G: Any + Send + Sync + 'static>(&self) -> bool {
        self.data.contains_key(&TypeId::of::<G>())
    }

    pub fn remove<G: Any + Send + Sync + 'static>(&mut self) -> Option<G> {
        self.data
            .remove(&TypeId::of::<G>())
            .and_then(|g| g.downcast().ok())
            .map(|g| *g)
    }
}

/// Main access point for systems
///
/// Provides access to:
/// - `time` - Frame timing (delta, fps, frame count)
/// - `events` - Event sending/reading
/// - `globals` - Shared state storage
///
/// # Example
/// ```ignore
/// fn my_system(ctx: &mut Context) {
///     let delta = ctx.time.delta();
///     ctx.global_mut::<MyState>().unwrap().update(delta);
/// }
/// ```
pub struct Context {
    pub time: Time,
    pub events: Events,
    pub globals: Globals,
    running: bool,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Creates a new Context
    pub fn new() -> Self {
        Self {
            time: Time::new(),
            events: Events::new(),
            globals: Globals::new(),
            running: true,
        }
    }

    /// Signals the engine to quit
    pub fn quit(&mut self) {
        self.running = false;
        log::info!("Quit requested");
    }

    /// Returns true if engine is still running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Registers a global state
    pub fn register<G: Any + Send + Sync + 'static>(&mut self, global: G) {
        self.globals.register(global);
    }

    /// Gets a reference to a global state
    pub fn global<G: Any + Send + Sync + 'static>(&self) -> Option<&G> {
        self.globals.get()
    }

    /// Gets a mutable reference to a global state
    pub fn global_mut<G: Any + Send + Sync + 'static>(&mut self) -> Option<&mut G> {
        self.globals.get_mut()
    }
}
