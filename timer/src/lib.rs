//! Timer.

#![deny(warnings)]

extern crate prelude;

/// Timer can be used to track time since some instant.
pub struct Timer {
    start: std::time::Instant,
}

fn to_secs(duration: std::time::Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9
}

impl Timer {
    /// Constructs a new timer.
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }

    /// Get time elapsed (in seconds) since last reset.
    pub fn elapsed(&self) -> f64 {
        to_secs(self.start.elapsed())
    }

    /// Reset, and get time elapsed (in seconds) since last reset.
    pub fn tick(&mut self) -> f64 {
        let now = std::time::Instant::now();
        let delta = now.duration_since(self.start);
        self.start = now;
        to_secs(delta)
    }
}
