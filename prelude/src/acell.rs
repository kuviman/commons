use ::*;

/// Atomic cell. Just like Cell, but thread-safe.
pub struct ACell<T: Copy> {
    inner: Mutex<T>,
}

impl<T: Copy + Debug> Debug for ACell<T> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self.get())
    }
}

impl<T: Copy> ACell<T> {
    /// Construct new cell with given value.
    pub fn new(value: T) -> Self {
        Self {
            inner: Mutex::new(value),
        }
    }

    /// Get current value from the cell.
    pub fn get(&self) -> T {
        *self.inner.lock().unwrap()
    }

    /// Change cell's value.
    pub fn set(&self, value: T) {
        *self.inner.lock().unwrap() = value;
    }
}
