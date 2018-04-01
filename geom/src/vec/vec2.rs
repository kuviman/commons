use ::*;

/// 2-d vector.
#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

/// Construct a 2-d vector with given components.
///
/// # Example
/// ```
/// use geom::*;
/// let v = vec2(1, 2);
/// ```
pub fn vec2<T>(x: T, y: T) -> Vec2<T> {
    Vec2 { x, y }
}

impl<T> From<[T; 2]> for Vec2<T> {
    fn from(mut v: [T; 2]) -> Vec2<T> {
        // TODO: just transmute?
        let x = unsafe { mem::replace(&mut v[0], mem::uninitialized()) };
        let y = unsafe { mem::replace(&mut v[1], mem::uninitialized()) };
        mem::forget(v);
        vec2(x, y)
    }
}

impl<T> Deref for Vec2<T> {
    type Target = [T; 2];
    fn deref(&self) -> &[T; 2] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> DerefMut for Vec2<T> {
    fn deref_mut(&mut self) -> &mut [T; 2] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> Vec2<T> {
    /// Extend into a 3-d vector.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// assert_eq!(vec2(1, 2).extend(3), vec3(1, 2, 3));
    /// ```
    pub fn extend(self, z: T) -> Vec3<T> {
        vec3(self.x, self.y, z)
    }
}

impl<T: Num + Copy> Vec2<T> {
    /// Calculate dot product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// assert_eq!(Vec2::dot(vec2(1, 2), vec2(3, 4)), 11);
    /// ```
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y
    }

    /// Calculate skew product of two vectors.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// assert_eq!(Vec2::skew(vec2(1, 2), vec2(3, 4)), -2);
    /// ```
    pub fn skew(a: Self, b: Self) -> T {
        a.x * b.y - a.y * b.x
    }
}

impl<T: Float> Vec2<T> {
    /// Normalize a vector.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let v: Vec2<f64> = vec2(1.0, 2.0);
    /// assert!((v.normalize().len() - 1.0).abs() < 1e-5);
    /// ```
    pub fn normalize(self) -> Self {
        self / self.len()
    }

    /// Calculate length of a vector.
    pub fn len(self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y)
    }

    /// Rotate a vector by a given angle.
    pub fn rotated(v: Self, angle: T) -> Self {
        let (sin, cos) = T::sin_cos(angle);
        Self {
            x: v.x * cos - v.y * sin,
            y: v.x * sin + v.y * cos,
        }
    }
}
