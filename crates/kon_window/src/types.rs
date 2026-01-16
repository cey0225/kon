/// Window size in pixels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowSize {
    pub width: u32,
    pub height: u32,
}

/// Window position in screen coordinates
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
}

/// Fullscreen mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Fullscreen {
    /// Exclusive fullscreen (changes video mode)
    Exclusive,
    /// Borderless windowed fullscreen
    Borderless,
}
