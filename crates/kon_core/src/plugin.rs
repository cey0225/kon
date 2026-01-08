use crate::{App, Context};

/// Plugin trait for extending engine functionality
///
/// # Example
/// ```ignore
/// pub struct MyPlugin;
///
/// impl Plugin for MyPlugin {
///     fn build(&self, app: &mut App) {
///         app.register(MyGlobalState::new());
///     }
/// }
/// ```
pub trait Plugin: 'static {
    /// Returns the plugin name (defaults to type name)
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }

    /// Called when plugin is added to App
    fn build(&self, app: &mut App);

    /// Called after all plugins are loaded
    fn ready(&self, _ctx: &mut Context) {}

    /// Called when App is shutting down
    fn cleanup(&self, _ctx: &mut Context) {}

    /// Returns true if this is a plugin bundle that adds other plugins internally.
    /// Used for accurate plugin count logging (e.g., DefaultPlugins).
    fn is_plugin_group(&self) -> bool {
        false
    }
}
