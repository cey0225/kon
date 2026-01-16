use std::path::PathBuf;
use crate::types::{Fullscreen, WindowSize};

/// Window configuration settings
///
/// Used to configure window properties during creation.
pub struct WindowConfig {
    /// Window title displayed in the title bar
    pub title: &'static str,
    /// Initial window size in pixels
    pub size: WindowSize,
    /// Whether the window is resizable
    pub resizable: bool,
    /// Whether to show window decorations (title bar, borders)
    pub decorations: bool,
    /// Whether the window is visible on creation
    pub visible: bool,
    /// Whether the window is maximized on creation
    pub maximized: bool,
    /// Fullscreen mode on creation
    pub fullscreen: Option<Fullscreen>,
    /// Window icon
    pub icon: Option<PathBuf>,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "Kon Engine",
            size: WindowSize {
                width: 1280,
                height: 720,
            },
            resizable: true,
            decorations: true,
            visible: true,
            maximized: false,
            fullscreen: None,
            icon: None,
        }
    }
}

impl WindowConfig {
    pub fn with_title(mut self, title: &'static str) -> Self {
        self.title = title;
        self
    }

    pub fn with_size(mut self, size: WindowSize) -> Self {
        self.size = size;
        self
    }

    pub fn with_resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }

    pub fn with_decorations(mut self, decorations: bool) -> Self {
        self.decorations = decorations;
        self
    }

    pub fn with_visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn with_maximized(mut self, maximized: bool) -> Self {
        self.maximized = maximized;
        self
    }

    pub fn with_fullscreen(mut self, fullscreen: Option<Fullscreen>) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    pub fn with_icon(mut self, icon: Option<PathBuf>) -> Self {
        self.icon = icon;
        self
    }
}
