//! Timer.

#![deny(warnings)]

extern crate prelude;
#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
#[macro_use]
extern crate stdweb;

/// Timer can be used to track time since some instant.
pub struct Timer {
    #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
    start: f64,
    #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
    start: std::time::Instant,
}

#[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
fn to_secs(duration: std::time::Duration) -> f64 {
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 / 1e9
}

#[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
fn now() -> f64 {
    use stdweb::unstable::TryInto;
    return js! {
        return Date.now() / 1000.0;
    }.try_into()
        .unwrap();
}

impl Timer {
    /// Constructs a new timer.
    pub fn new() -> Self {
        Self {
            #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
            start: now(),
            #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
            start: std::time::Instant::now(),
        }
    }

    /// Get time elapsed (in seconds) since last reset.
    pub fn elapsed(&self) -> f64 {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        return now() - self.start;
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        return to_secs(self.start.elapsed());
    }

    /// Reset, and get time elapsed (in seconds) since last reset.
    pub fn tick(&mut self) -> f64 {
        #[cfg(any(target_arch = "asmjs", target_arch = "wasm32"))]
        {
            let now = now();
            let delta = now - self.start;
            self.start = now;
            delta
        }
        #[cfg(not(any(target_arch = "asmjs", target_arch = "wasm32")))]
        {
            let now = std::time::Instant::now();
            let delta = now.duration_since(self.start);
            self.start = now;
            to_secs(delta)
        }
    }
}

#[test]
fn test() {
    let mut timer = Timer::new();
    timer.elapsed();
    for _ in 0..100 {
        timer.tick();
    }
}
