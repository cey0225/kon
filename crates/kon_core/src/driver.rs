use crate::App;

/// Trait for custom game loop implementations
///
/// Drivers control how the application lifecycle is executed. Different drivers
/// can implement different loop strategies (e.g., fixed timestep, event-driven,
/// or headless simulation).
///
/// The driver takes ownership of the App and manages:
/// - Initialization (`app.initialize()`)
/// - Frame updates (`app.tick()`)
/// - Cleanup (`app.cleanup()`)
///
/// # Example
/// ```ignore
/// struct CustomDriver;
///
/// impl Driver for CustomDriver {
///     fn drive(self: Box<Self>, mut app: App) {
///         app.initialize();
///
///         // Custom loop logic
///         for _ in 0..100 {
///             app.tick();
///         }
///
///         app.cleanup();
///     }
/// }
///
/// Kon::new()
///     .set_driver(CustomDriver)
///     .run();
/// ```
pub trait Driver {
    /// Executes the game loop with the provided application
    ///
    /// Takes ownership of both the driver and the app. Called automatically
    /// by `App::run()`. Implementations should handle initialization, update
    /// loop, and cleanup.
    fn drive(self: Box<Self>, app: App);
}

/// Default game loop driver with simple while-loop execution
///
/// Runs the standard game loop:
/// 1. Initialize all plugins and startup systems
/// 2. Update frame-by-frame while the app is running
/// 3. Clean up plugins on exit
///
/// This is the driver used when no custom driver is set.
pub struct DefaultDriver;

impl Driver for DefaultDriver {
    fn drive(self: Box<Self>, mut app: App) {
        app.initialize();

        while app.context().is_running() {
            app.tick();
        }

        app.cleanup();
    }
}
