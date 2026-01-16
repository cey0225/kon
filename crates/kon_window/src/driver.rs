use kon_core::{App, Driver};
use winit::event_loop::{ControlFlow, EventLoop};
use crate::WindowBackend;

pub struct WindowDriver;

impl Driver for WindowDriver {
    #[track_caller]
    fn drive(self: Box<Self>, app: App) {
        let event_loop = EventLoop::new().expect("Winit EventLoop creation failed");
        event_loop.set_control_flow(ControlFlow::Poll);

        let mut window_backend = WindowBackend { app };
        event_loop
            .run_app(&mut window_backend)
            .expect("WindowBackend execution failed");
    }
}
