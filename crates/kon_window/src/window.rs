use std::path::Path;
use image::GenericImageView;
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    window::{Fullscreen as FS, Icon, Window, WindowLevel},
};
use crate::{
    WindowConfig,
    types::{Fullscreen, WindowPosition, WindowSize},
};

/// Window wrapper providing engine-level window operations
///
/// Wraps winit's Window and provides a simplified API for common
/// window management tasks like setting title, resizing, and
/// querying window properties.
pub struct KonWindow {
    raw: Window,
}

impl KonWindow {
    /// Creates a new window from a raw winit Window
    pub fn new(raw: Window) -> Self {
        Self { raw }
    }

    /// Returns a reference to the underlying winit Window
    pub fn raw(&self) -> &Window {
        &self.raw
    }

    /// Sets the window config
    ///
    /// # Example
    /// ```ignore
    /// ctx.window().set_config(WindowConfig::default().with_title("Custom Title"));
    /// ```
    pub fn set_config(&self, config: WindowConfig) {
        self.set_title(config.title);
        self.set_size(config.size);
        self.set_resizable(config.resizable);
        self.set_decorations(config.decorations);
        self.set_visible(config.visible);

        if config.maximized {
            self.maximize();
        }

        self.set_fullscreen(config.fullscreen);

        if let Some(icon) = config.icon {
            self.set_icon(icon);
        }
    }

    /// Sets the window title
    pub fn set_title(&self, title: &str) {
        self.raw.set_title(title);
    }

    /// Returns the window's content area size
    pub fn size(&self) -> WindowSize {
        let size = self.raw.inner_size();

        WindowSize {
            width: size.width,
            height: size.height,
        }
    }

    /// Sets the window's content area size
    pub fn set_size(&self, size: WindowSize) {
        let _ = self
            .raw
            .request_inner_size(PhysicalSize::new(size.width, size.height));
    }

    /// Returns the window's total size including decorations
    pub fn outer_size(&self) -> WindowSize {
        let size = self.raw.outer_size();

        WindowSize {
            width: size.width,
            height: size.height,
        }
    }

    /// Returns the window's position on screen
    pub fn position(&self) -> Option<WindowPosition> {
        self.raw
            .outer_position()
            .ok()
            .map(|pos| WindowPosition { x: pos.x, y: pos.y })
    }

    /// Sets the window's position on screen
    pub fn set_position(&self, position: WindowPosition) {
        self.raw
            .set_outer_position(PhysicalPosition::new(position.x, position.y));
    }

    /// Sets whether the window is resizable
    pub fn set_resizable(&self, resizable: bool) {
        self.raw.set_resizable(resizable);
    }

    /// Returns whether the window is resizable
    pub fn is_resizable(&self) -> bool {
        self.raw.is_resizable()
    }

    /// Sets fullscreen mode
    pub fn set_fullscreen(&self, mode: Option<Fullscreen>) {
        match mode {
            Some(Fullscreen::Borderless) => self.raw.set_fullscreen(Some(FS::Borderless(None))),
            Some(Fullscreen::Exclusive) => {
                if let Some(monitor) = self
                    .raw
                    .current_monitor()
                    .or_else(|| self.raw.primary_monitor())
                    && let Some(video_mode) = monitor
                        .video_modes()
                        .max_by_key(|mode| mode.size().width * mode.size().height)
                {
                    self.raw.set_fullscreen(Some(FS::Exclusive(video_mode)));
                }
            }
            None => self.raw.set_fullscreen(None),
        }
    }

    /// Returns current fullscreen mode
    pub fn fullscreen(&self) -> Option<Fullscreen> {
        match self.raw.fullscreen() {
            Some(FS::Borderless(_)) => Some(Fullscreen::Borderless),
            Some(FS::Exclusive(_)) => Some(Fullscreen::Exclusive),
            None => None,
        }
    }

    /// Minimizes the window
    pub fn minimize(&self) {
        self.raw.set_minimized(true);
    }

    /// Maximizes the window
    pub fn maximize(&self) {
        self.raw.set_maximized(true);
    }

    /// Returns whether the window is maximized
    pub fn is_maximized(&self) -> bool {
        self.raw.is_maximized()
    }

    /// Shows or hides the window
    pub fn set_visible(&self, visible: bool) {
        self.raw.set_visible(visible);
    }

    /// Returns whether the window is visible
    ///
    /// Returns None if visibility state cannot be determined.
    pub fn is_visible(&self) -> Option<bool> {
        self.raw.is_visible()
    }

    /// Returns whether the window has input focus
    pub fn has_focus(&self) -> bool {
        self.raw.has_focus()
    }

    /// Sets minimum window size
    pub fn set_min_size(&self, size: WindowSize) {
        self.raw
            .set_min_inner_size(Some(PhysicalSize::new(size.width, size.height)));
    }

    /// Sets maximum window size
    pub fn set_max_size(&self, size: WindowSize) {
        self.raw
            .set_max_inner_size(Some(PhysicalSize::new(size.width, size.height)));
    }

    /// Returns the window's DPI scale factor
    pub fn scale_factor(&self) -> f64 {
        self.raw.scale_factor()
    }

    /// Sets whether the window stays on top of others
    pub fn set_always_on_top(&self, always_on_top: bool) {
        self.raw.set_window_level(if always_on_top {
            WindowLevel::AlwaysOnTop
        } else {
            WindowLevel::Normal
        });
    }

    /// Shows or hides window decorations (title bar, borders)
    pub fn set_decorations(&self, decorations: bool) {
        self.raw.set_decorations(decorations);
    }

    /// Returns whether decorations are visible
    pub fn has_decorations(&self) -> bool {
        self.raw.is_decorated()
    }

    /// Sets the window icon
    pub fn set_icon<P: AsRef<Path>>(&self, path: P) {
        self.raw.set_window_icon(load_icon(path));
    }
}

fn load_icon<P: AsRef<Path>>(path: P) -> Option<Icon> {
    let img = image::open(path).ok()?;
    let (width, height) = img.dimensions();
    let rgba_pixels = img.to_rgba8().into_raw();
    Icon::from_rgba(rgba_pixels, width, height).ok()
}
