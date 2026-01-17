//! Input state management and action bindings
//!
//! Uses a 256-bit array to track keyboard and mouse button states.
//! Stores current and previous frame states for edge detection.

use std::collections::HashMap;
use kon_core::events::{InputState, KeyCode, MouseButton};

/// Defines an input source that can trigger an action
///
/// Used with `Input::add_binding()` to map hardware inputs to named actions.
///
/// # Example
/// ```ignore
/// input.add_binding("Jump", InputSource::Key(KeyCode::Space));
/// input.add_binding("Aim", InputSource::Mouse(MouseButton::Right));
/// input.add_binding("QuickSave", InputSource::Chord(KeyCode::LControl, KeyCode::S));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputSource {
    Key(KeyCode),
    Mouse(MouseButton),
    Chord(KeyCode, KeyCode),
    MouseChord(KeyCode, MouseButton),
}

/// Internal mode for state checking
enum Mode {
    Pressed,
    Just,
    Released,
}

/// Input state manager
///
/// Tracks keyboard and mouse states using a 256-bit bitmask array.
/// Provides frame-accurate detection for pressed, just pressed, and just released states.
///
/// # State Storage
/// - Keys: bits 0-127
/// - Mouse buttons: bits 128-255
///
/// # Frame Detection
/// - `is_*_pressed`: Currently held down
/// - `just_*_pressed`: Pressed this frame (not held last frame)
/// - `just_*_released`: Released this frame (was held last frame)
///
/// # Example
/// ```ignore
/// // Direct key/button queries
/// if input.is_key_pressed(KeyCode::W) { ... }
/// if input.just_button_pressed(MouseButton::Left) { ... }
///
/// // Action-based queries (recommended)
/// if input.is_action_pressed("MoveForward") { ... }
/// if input.just_action_pressed("Fire") { ... }
/// ```
pub struct Input {
    current_state: [u64; 4],
    previous_state: [u64; 4],
    mouse_position: (f32, f32),
    mouse_motion: (f32, f32),
    mouse_wheel: (f32, f32),
    bindings: HashMap<String, Vec<InputSource>>,
}

impl Default for Input {
    fn default() -> Self {
        let mut input = Self {
            current_state: [0; 4],
            previous_state: [0; 4],
            mouse_position: (0.0, 0.0),
            mouse_motion: (0.0, 0.0),
            mouse_wheel: (0.0, 0.0),
            bindings: HashMap::new(),
        };

        // Default bindings
        input.add_binding("MoveForward", InputSource::Key(KeyCode::W));
        input.add_binding("MoveForward", InputSource::Key(KeyCode::Up));
        input.add_binding("MoveBackward", InputSource::Key(KeyCode::S));
        input.add_binding("MoveBackward", InputSource::Key(KeyCode::Down));
        input.add_binding("Jump", InputSource::Key(KeyCode::Space));
        input.add_binding("Fire", InputSource::Mouse(MouseButton::Left));
        input.add_binding("Sprint", InputSource::Key(KeyCode::LShift));
        input.add_binding(
            "SpecialSkill",
            InputSource::MouseChord(KeyCode::LShift, MouseButton::Right),
        );

        input
    }
}

impl Input {
    const MOUSE_OFFSET: usize = 128;

    // ========================================================================
    // Keyboard
    // ========================================================================

    /// Returns true if the key is currently held down
    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        is_bit_set(&self.current_state, bit_index(key as usize, 0))
    }

    /// Returns true if the key was pressed this frame
    ///
    /// Only returns true on the first frame the key is pressed.
    pub fn just_key_pressed(&self, key: KeyCode) -> bool {
        self.is_key_pressed(key) && !self.was_key_pressed(key)
    }

    /// Returns true if the key was released this frame
    ///
    /// Only returns true on the first frame the key is released.
    pub fn just_key_released(&self, key: KeyCode) -> bool {
        !self.is_key_pressed(key) && self.was_key_pressed(key)
    }

    pub(crate) fn was_key_pressed(&self, key: KeyCode) -> bool {
        is_bit_set(&self.previous_state, bit_index(key as usize, 0))
    }

    // ========================================================================
    // Mouse Buttons
    // ========================================================================

    /// Returns true if the mouse button is currently held down
    pub fn is_button_pressed(&self, button: MouseButton) -> bool {
        is_bit_set(
            &self.current_state,
            bit_index(mouse_button_index(button), Self::MOUSE_OFFSET),
        )
    }

    /// Returns true if the mouse button was pressed this frame
    pub fn just_button_pressed(&self, button: MouseButton) -> bool {
        self.is_button_pressed(button) && !self.was_button_pressed(button)
    }

    /// Returns true if the mouse button was released this frame
    pub fn just_button_released(&self, button: MouseButton) -> bool {
        !self.is_button_pressed(button) && self.was_button_pressed(button)
    }

    pub(crate) fn was_button_pressed(&self, button: MouseButton) -> bool {
        is_bit_set(
            &self.previous_state,
            bit_index(mouse_button_index(button), Self::MOUSE_OFFSET),
        )
    }

    // ========================================================================
    // Action Bindings
    // ========================================================================

    /// Returns true if any input bound to the action is currently held
    ///
    /// Checks all input sources registered for the action.
    /// For chords, both keys must be held.
    pub fn is_action_pressed(&self, action: &str) -> bool {
        self.bindings.get(action).map_or(false, |sources| {
            sources
                .iter()
                .any(|source| self.check_source(source, Mode::Pressed))
        })
    }

    /// Returns true if any input bound to the action was triggered this frame
    ///
    /// For single keys/buttons: true on the frame they're pressed.
    /// For chords: true when the final key/button completes the chord.
    pub fn just_action_pressed(&self, action: &str) -> bool {
        self.bindings.get(action).map_or(false, |sources| {
            sources
                .iter()
                .any(|source| self.check_source(source, Mode::Just))
        })
    }

    /// Returns true if any input bound to the action was released this frame
    ///
    /// For chords: true when the non-modifier key/button is released
    /// while the modifier is still held.
    pub fn just_action_released(&self, action: &str) -> bool {
        self.bindings.get(action).map_or(false, |sources| {
            sources
                .iter()
                .any(|source| self.check_source(source, Mode::Released))
        })
    }

    /// Registers an input source for an action
    ///
    /// Multiple sources can be bound to the same action.
    /// The action triggers if any of its sources are activated.
    ///
    /// # Example
    /// ```ignore
    /// // Multiple keys for the same action
    /// input.add_binding("MoveForward", InputSource::Key(KeyCode::W));
    /// input.add_binding("MoveForward", InputSource::Key(KeyCode::Up));
    ///
    /// // Key + mouse chord
    /// input.add_binding("SpecialAttack", InputSource::MouseChord(KeyCode::LShift, MouseButton::Left));
    /// ```
    pub fn add_binding(&mut self, action: &str, source: InputSource) {
        self.bindings
            .entry(action.to_string())
            .or_insert_with(Vec::new)
            .push(source);
    }

    fn check_source(&self, source: &InputSource, mode: Mode) -> bool {
        match (source, mode) {
            (InputSource::Key(k), Mode::Pressed) => self.is_key_pressed(*k),
            (InputSource::Key(k), Mode::Just) => self.just_key_pressed(*k),
            (InputSource::Key(k), Mode::Released) => self.just_key_released(*k),

            (InputSource::Mouse(b), Mode::Pressed) => self.is_button_pressed(*b),
            (InputSource::Mouse(b), Mode::Just) => self.just_button_pressed(*b),
            (InputSource::Mouse(b), Mode::Released) => self.just_button_released(*b),

            (InputSource::Chord(m, k), Mode::Pressed) => {
                self.is_key_pressed(*m) && self.is_key_pressed(*k)
            }
            (InputSource::Chord(m, k), Mode::Just) => {
                self.is_key_pressed(*m) && self.just_key_pressed(*k)
            }
            (InputSource::Chord(m, k), Mode::Released) => {
                self.is_key_pressed(*m) && self.just_key_released(*k)
            }

            (InputSource::MouseChord(m, b), Mode::Pressed) => {
                self.is_key_pressed(*m) && self.is_button_pressed(*b)
            }
            (InputSource::MouseChord(m, b), Mode::Just) => {
                self.is_key_pressed(*m) && self.just_button_pressed(*b)
            }
            (InputSource::MouseChord(m, b), Mode::Released) => {
                self.is_key_pressed(*m) && self.just_button_released(*b)
            }
        }
    }

    // ========================================================================
    // Internal (called by InputPlugin)
    // ========================================================================

    pub(crate) fn set_key(&mut self, key: KeyCode, state: InputState) {
        set_bit(
            &mut self.current_state,
            bit_index(key as usize, 0),
            matches!(state, InputState::Pressed),
        );
    }

    pub(crate) fn set_button(&mut self, button: MouseButton, state: InputState) {
        set_bit(
            &mut self.current_state,
            bit_index(mouse_button_index(button), Self::MOUSE_OFFSET),
            matches!(state, InputState::Pressed),
        );
    }

    pub(crate) fn set_mouse_position(&mut self, x: f32, y: f32) {
        self.mouse_position = (x, y);
    }

    pub(crate) fn add_mouse_motion(&mut self, dx: f32, dy: f32) {
        self.mouse_motion.0 += dx;
        self.mouse_motion.1 += dy;
    }

    pub(crate) fn set_mouse_wheel(&mut self, dx: f32, dy: f32) {
        self.mouse_wheel = (dx, dy);
    }

    /// Syncs state between frames
    ///
    /// Called at frame end by InputPlugin. Copies current state to previous
    /// and resets per-frame accumulators (mouse motion, wheel).
    pub(crate) fn sync(&mut self) {
        self.previous_state = self.current_state;
        self.mouse_motion = (0.0, 0.0);
        self.mouse_wheel = (0.0, 0.0);
    }
}

// ============================================================================
// Bit Operations
// ============================================================================

#[inline(always)]
fn bit_index(raw: usize, offset: usize) -> usize {
    offset + raw
}

#[inline(always)]
fn is_bit_set(bits: &[u64; 4], bit: usize) -> bool {
    (bits[bit >> 6] & (1u64 << (bit & 63))) != 0
}

#[inline(always)]
fn set_bit(bits: &mut [u64; 4], bit: usize, pressed: bool) {
    debug_assert!(bit < 256, "Bit index out of bounds: {}", bit);

    if pressed {
        bits[bit >> 6] |= 1u64 << (bit & 63);
    } else {
        bits[bit >> 6] &= !(1u64 << (bit & 63))
    }
}

#[inline]
fn mouse_button_index(button: MouseButton) -> usize {
    match button {
        MouseButton::Forward => 0,
        MouseButton::Back => 1,
        MouseButton::Left => 2,
        MouseButton::Right => 3,
        MouseButton::Middle => 4,
        MouseButton::Other(n) => n as usize + 5,
    }
}
