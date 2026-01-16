use std::fmt::{Display, Formatter, Result};

/// Application lifecycle event for quit requests
///
/// Sent when the application should terminate (e.g., window close button,
/// Alt+F4, or programmatic exit request).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AppQuit;

/// Window resized event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowResized {
    pub width: u32,
    pub height: u32,
}

/// Window focus changed event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowFocused {
    pub focused: bool,
}

/// Window moved event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowMoved {
    pub x: i32,
    pub y: i32,
}

/// Window DPI scale factor changed
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WindowScaleFactorChanged {
    pub scale_factor: f64,
}

/// Window close requested event
///
/// Sent when the user requests to close the window. The application can
/// prevent closing by not sending `AppQuit` in response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCloseRequested;

/// Text input event with unicode and IME support
///
/// Represents actual text characters entered by the user, including:
/// - Unicode characters (ü, ş, ğ, emoji, etc.)
/// - IME composition results (Chinese, Japanese, Korean)
/// - Dead key combinations (é, ñ, etc.)
///
/// Use `KeyboardInput` for game controls and shortcuts, and `TextInput`
/// for text editing interfaces like chat boxes and input fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextInput {
    pub text: String,
}

/// Keyboard key state change event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardInput {
    pub key: KeyCode,
    pub state: InputState,
}

/// Keyboard key codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyCode {
    // Letters
    Q,
    W,
    E,
    R,
    T,
    Y,
    U,
    I,
    O,
    P,
    A,
    S,
    D,
    F,
    G,
    H,
    J,
    K,
    L,
    Z,
    X,
    C,
    V,
    B,
    N,
    M,

    // Numbers
    Num0,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,

    // Function keys
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,

    // Navigation
    Up,
    Down,
    Left,
    Right,
    Home,
    End,
    PageUp,
    PageDown,

    // Editing
    Backspace,
    Delete,
    Insert,
    Tab,
    Enter,
    Escape,
    Space,

    // Modifiers
    LShift,
    RShift,
    LControl,
    RControl,
    LAlt,
    RAlt,
    LSuper,
    RSuper,

    // Special
    CapsLock,
    NumLock,
    ScrollLock,
    PrintScreen,
    Pause,

    // Punctuation
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Comma,
    Period,
    Slash,
    Grave,
}

impl Display for KeyCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        use KeyCode as K;
        match self {
            // Letters
            K::A => write!(f, "A"),
            K::B => write!(f, "B"),
            K::C => write!(f, "C"),
            K::D => write!(f, "D"),
            K::E => write!(f, "E"),
            K::F => write!(f, "F"),
            K::G => write!(f, "G"),
            K::H => write!(f, "H"),
            K::I => write!(f, "I"),
            K::J => write!(f, "J"),
            K::K => write!(f, "K"),
            K::L => write!(f, "L"),
            K::M => write!(f, "M"),
            K::N => write!(f, "N"),
            K::O => write!(f, "O"),
            K::P => write!(f, "P"),
            K::Q => write!(f, "Q"),
            K::R => write!(f, "R"),
            K::S => write!(f, "S"),
            K::T => write!(f, "T"),
            K::U => write!(f, "U"),
            K::V => write!(f, "V"),
            K::W => write!(f, "W"),
            K::X => write!(f, "X"),
            K::Y => write!(f, "Y"),
            K::Z => write!(f, "Z"),

            // Numbers
            K::Num0 => write!(f, "0"),
            K::Num1 => write!(f, "1"),
            K::Num2 => write!(f, "2"),
            K::Num3 => write!(f, "3"),
            K::Num4 => write!(f, "4"),
            K::Num5 => write!(f, "5"),
            K::Num6 => write!(f, "6"),
            K::Num7 => write!(f, "7"),
            K::Num8 => write!(f, "8"),
            K::Num9 => write!(f, "9"),

            // Function keys
            K::F1 => write!(f, "F1"),
            K::F2 => write!(f, "F2"),
            K::F3 => write!(f, "F3"),
            K::F4 => write!(f, "F4"),
            K::F5 => write!(f, "F5"),
            K::F6 => write!(f, "F6"),
            K::F7 => write!(f, "F7"),
            K::F8 => write!(f, "F8"),
            K::F9 => write!(f, "F9"),
            K::F10 => write!(f, "F10"),
            K::F11 => write!(f, "F11"),
            K::F12 => write!(f, "F12"),

            // Navigation
            K::Up => write!(f, "Up"),
            K::Down => write!(f, "Down"),
            K::Left => write!(f, "Left"),
            K::Right => write!(f, "Right"),
            K::Home => write!(f, "Home"),
            K::End => write!(f, "End"),
            K::PageUp => write!(f, "PageUp"),
            K::PageDown => write!(f, "PageDown"),

            // Editing
            K::Backspace => write!(f, "Backspace"),
            K::Delete => write!(f, "Delete"),
            K::Insert => write!(f, "Insert"),
            K::Tab => write!(f, "Tab"),
            K::Enter => write!(f, "Enter"),
            K::Escape => write!(f, "Escape"),
            K::Space => write!(f, "Space"),

            // Modifiers
            K::LShift => write!(f, "Left Shift"),
            K::RShift => write!(f, "Right Shift"),
            K::LControl => write!(f, "Left Ctrl"),
            K::RControl => write!(f, "Right Ctrl"),
            K::LAlt => write!(f, "Left Alt"),
            K::RAlt => write!(f, "Right Alt"),
            K::LSuper => write!(f, "Left Super"),
            K::RSuper => write!(f, "Right Super"),

            // Special
            K::CapsLock => write!(f, "CapsLock"),
            K::NumLock => write!(f, "NumLock"),
            K::ScrollLock => write!(f, "ScrollLock"),
            K::PrintScreen => write!(f, "PrintScreen"),
            K::Pause => write!(f, "Pause"),

            // Punctuation
            K::Minus => write!(f, "-"),
            K::Equals => write!(f, "="),
            K::LeftBracket => write!(f, "["),
            K::RightBracket => write!(f, "]"),
            K::Backslash => write!(f, "\\"),
            K::Semicolon => write!(f, ";"),
            K::Apostrophe => write!(f, "'"),
            K::Comma => write!(f, ","),
            K::Period => write!(f, "."),
            K::Slash => write!(f, "/"),
            K::Grave => write!(f, "`"),
        }
    }
}

/// Mouse button input event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseButtonInput {
    pub button: MouseButton,
    pub state: InputState,
}

/// Key/button state (pressed or released)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputState {
    Pressed,
    Released,
}

/// Mouse buttons
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButton {
    Forward,
    Back,
    Left,
    Right,
    Middle,
    Other(u16),
}

/// Mouse cursor moved event
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseMotion {
    pub x: f32,
    pub y: f32,
}

/// Mouse wheel scrolled event
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MouseWheel {
    pub delta_x: f32,
    pub delta_y: f32,
}

/// Mouse cursor entered window event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorEntered;

/// Mouse cursor left window event
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CursorLeft;
