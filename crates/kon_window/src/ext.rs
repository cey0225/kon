use kon_core::Context;

use crate::KonWindow;

pub trait ContextWindowExt {
    fn window(&self) -> &KonWindow;
}

impl ContextWindowExt for Context {
    #[track_caller]
    fn window(&self) -> &KonWindow {
        self.global::<KonWindow>().expect(
            "Failed to access KonWindow. Ensure 'DefaultPlugins' or 'WindowPlugin' is added",
        )
    }
}
