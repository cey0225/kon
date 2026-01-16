use kon_core::{
    App,
    events::{
        CursorEntered, CursorLeft, InputState, KeyboardInput, MouseButtonInput, MouseMotion,
        MouseWheel, TextInput, WindowCloseRequested, WindowFocused, WindowMoved, WindowResized,
        WindowScaleFactorChanged,
    },
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, Ime, MouseScrollDelta, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::{WindowAttributes, WindowId},
};
use crate::{KonWindow, WindowConfig, ContextWindowExt};

pub(crate) struct WindowBackend {
    pub app: App,
}

impl ApplicationHandler for WindowBackend {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let config = WindowConfig::default();
        let attributes = WindowAttributes::default()
            .with_title(config.title)
            .with_inner_size(PhysicalSize::new(config.size.width, config.size.height))
            .with_resizable(config.resizable)
            .with_decorations(config.decorations)
            .with_visible(config.visible)
            .with_maximized(config.maximized);
        let window = event_loop
            .create_window(attributes)
            .expect("Window creation failed");
        let kon_window = KonWindow::new(window);
        kon_window.set_fullscreen(config.fullscreen);

        if let Some(icon) = config.icon {
            kon_window.set_icon(icon);
        }

        self.app.register(kon_window);

        self.app.initialize();

        log::info!("Window created");
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.app.context().window().raw().request_redraw();
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        self.app.cleanup();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.app.context_mut().events.send(WindowCloseRequested);
                self.app.context().window().raw().request_redraw();
            }
            WindowEvent::RedrawRequested => {
                self.app.tick();

                if !self.app.context().is_running() {
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(size) => {
                self.app.context_mut().events.send(WindowResized {
                    width: size.width,
                    height: size.height,
                });
            }
            WindowEvent::Focused(focused) => {
                self.app
                    .context_mut()
                    .events
                    .send(WindowFocused { focused });
            }
            WindowEvent::Moved(pos) => {
                self.app
                    .context_mut()
                    .events
                    .send(WindowMoved { x: pos.x, y: pos.y });
            }
            WindowEvent::ScaleFactorChanged { scale_factor, .. } => self
                .app
                .context_mut()
                .events
                .send(WindowScaleFactorChanged { scale_factor }),
            WindowEvent::Ime(Ime::Commit(text)) => {
                self.app.context_mut().events.send(TextInput { text });
            }
            WindowEvent::KeyboardInput { event, .. } => {
                if let Some(key) = match event.physical_key {
                    PhysicalKey::Code(key_code) => map_winit_key(key_code),
                    PhysicalKey::Unidentified(_) => None,
                } {
                    let state = map_winit_state(event.state);

                    self.app
                        .context_mut()
                        .events
                        .send(KeyboardInput { key, state });
                }
            }
            WindowEvent::MouseInput { button, state, .. } => {
                let button = map_winit_button(button);
                let state = map_winit_state(state);

                self.app
                    .context_mut()
                    .events
                    .send(MouseButtonInput { button, state });
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.app.context_mut().events.send(MouseMotion {
                    x: position.x as f32,
                    y: position.y as f32,
                })
            }
            WindowEvent::MouseWheel { delta, .. } => {
                let (delta_x, delta_y) = match delta {
                    MouseScrollDelta::LineDelta(x, y) => (x, y),
                    MouseScrollDelta::PixelDelta(pos) => (pos.x as f32, pos.y as f32),
                };

                self.app
                    .context_mut()
                    .events
                    .send(MouseWheel { delta_x, delta_y });
            }
            WindowEvent::CursorEntered { .. } => {
                self.app.context_mut().events.send(CursorEntered);
            }
            WindowEvent::CursorLeft { .. } => {
                self.app.context_mut().events.send(CursorLeft);
            }
            _ => (),
        }
    }
}

fn map_winit_state(state: ElementState) -> InputState {
    match state {
        ElementState::Pressed => InputState::Pressed,
        ElementState::Released => InputState::Released,
    }
}

fn map_winit_key(key: winit::keyboard::KeyCode) -> Option<kon_core::events::KeyCode> {
    use winit::keyboard::KeyCode as WK;
    use kon_core::events::KeyCode as K;

    Some(match key {
        WK::KeyA => K::A,
        WK::KeyB => K::B,
        WK::KeyC => K::C,
        WK::KeyD => K::D,
        WK::KeyE => K::E,
        WK::KeyF => K::F,
        WK::KeyG => K::G,
        WK::KeyH => K::H,
        WK::KeyI => K::I,
        WK::KeyJ => K::J,
        WK::KeyK => K::K,
        WK::KeyL => K::L,
        WK::KeyM => K::M,
        WK::KeyN => K::N,
        WK::KeyO => K::O,
        WK::KeyP => K::P,
        WK::KeyQ => K::Q,
        WK::KeyR => K::R,
        WK::KeyS => K::S,
        WK::KeyT => K::T,
        WK::KeyU => K::U,
        WK::KeyV => K::V,
        WK::KeyW => K::W,
        WK::KeyX => K::X,
        WK::KeyY => K::Y,
        WK::KeyZ => K::Z,

        WK::Digit0 => K::Num0,
        WK::Digit1 => K::Num1,
        WK::Digit2 => K::Num2,
        WK::Digit3 => K::Num3,
        WK::Digit4 => K::Num4,
        WK::Digit5 => K::Num5,
        WK::Digit6 => K::Num6,
        WK::Digit7 => K::Num7,
        WK::Digit8 => K::Num8,
        WK::Digit9 => K::Num9,

        WK::F1 => K::F1,
        WK::F2 => K::F2,
        WK::F3 => K::F3,
        WK::F4 => K::F4,
        WK::F5 => K::F5,
        WK::F6 => K::F6,
        WK::F7 => K::F7,
        WK::F8 => K::F8,
        WK::F9 => K::F9,
        WK::F10 => K::F10,
        WK::F11 => K::F11,
        WK::F12 => K::F12,

        WK::ArrowUp => K::Up,
        WK::ArrowDown => K::Down,
        WK::ArrowLeft => K::Left,
        WK::ArrowRight => K::Right,

        WK::Home => K::Home,
        WK::End => K::End,
        WK::PageUp => K::PageUp,
        WK::PageDown => K::PageDown,

        WK::Backspace => K::Backspace,
        WK::Delete => K::Delete,
        WK::Insert => K::Insert,
        WK::Tab => K::Tab,
        WK::Enter => K::Enter,
        WK::Escape => K::Escape,
        WK::Space => K::Space,

        WK::ShiftLeft => K::LShift,
        WK::ShiftRight => K::RShift,
        WK::ControlLeft => K::LControl,
        WK::ControlRight => K::RControl,
        WK::AltLeft => K::LAlt,
        WK::AltRight => K::RAlt,
        WK::SuperLeft => K::LSuper,
        WK::SuperRight => K::RSuper,

        WK::CapsLock => K::CapsLock,
        WK::NumLock => K::NumLock,
        WK::ScrollLock => K::ScrollLock,
        WK::PrintScreen => K::PrintScreen,
        WK::Pause => K::Pause,

        WK::Minus => K::Minus,
        WK::Equal => K::Equals,
        WK::BracketLeft => K::LeftBracket,
        WK::BracketRight => K::RightBracket,
        WK::Backslash => K::Backslash,
        WK::Semicolon => K::Semicolon,
        WK::Quote => K::Apostrophe,
        WK::Comma => K::Comma,
        WK::Period => K::Period,
        WK::Slash => K::Slash,
        WK::Backquote => K::Grave,

        _ => return None,
    })
}

fn map_winit_button(button: winit::event::MouseButton) -> kon_core::events::MouseButton {
    use winit::event::MouseButton as WB;
    use kon_core::events::MouseButton as K;

    match button {
        WB::Forward => K::Forward,
        WB::Back => K::Back,
        WB::Left => K::Left,
        WB::Right => K::Right,
        WB::Middle => K::Middle,
        WB::Other(id) => K::Other(id),
    }
}
