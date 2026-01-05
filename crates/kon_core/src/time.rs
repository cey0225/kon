use std::time::{Duration, Instant};

/// Frame timing information
///
/// Updated every frame automatically.
///
/// # Example
/// ```ignore
/// fn my_system(ctx: &mut Context) {
///     let delta = ctx.time.delta();        // Seconds since last frame
///     let fps = ctx.time.fps();            // Current FPS
///     let frame = ctx.time.frame_count();  // Total frames
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Time {
    startup: Instant,
    last_frame: Instant,
    delta: Duration,
    elapsed: Duration,
    frame_count: u64,
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    /// Creates a new Time instance
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            startup: now,
            last_frame: now,
            delta: Duration::ZERO,
            elapsed: Duration::ZERO,
            frame_count: 0,
        }
    }

    pub(crate) fn update(&mut self) {
        let now = Instant::now();
        self.delta = now - self.last_frame;
        self.last_frame = now;
        self.elapsed = now - self.startup;
        self.frame_count += 1;
    }

    /// Returns delta time in seconds
    #[inline]
    pub fn delta(&self) -> f32 {
        self.delta.as_secs_f32()
    }

    /// Returns delta time as Duration
    #[inline]
    pub fn delta_duration(&self) -> Duration {
        self.delta
    }

    /// Returns elapsed time since startup in seconds
    #[inline]
    pub fn elapsed(&self) -> f32 {
        self.elapsed.as_secs_f32()
    }

    /// Returns elapsed time as Duration
    #[inline]
    pub fn elapsed_duration(&self) -> Duration {
        self.elapsed
    }

    /// Returns total frame count
    #[inline]
    pub fn frame_count(&self) -> u64 {
        self.frame_count
    }

    /// Returns current frames per second
    #[inline]
    pub fn fps(&self) -> f32 {
        if self.delta.as_secs_f32() > 0.0 {
            1.0 / self.delta.as_secs_f32()
        } else {
            0.0
        }
    }
}
