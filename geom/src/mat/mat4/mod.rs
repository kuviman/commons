use ::*;

mod extra;
mod ops;
mod projection;
mod transform;

/// 4x4 matrix
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat4<T>([[T; 4]; 4]);

impl<T> Mat4<T> {
    /// Construct a matrix.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let matrix = Mat4::new([
    ///     [1, 2, 3, 4],
    ///     [3, 4, 5, 6],
    ///     [5, 6, 7, 8],
    ///     [0, 5, 2, 9],
    /// ]);
    /// ```
    pub fn new(values: [[T; 4]; 4]) -> Self {
        Mat4(values)
    }
}

impl<T> Deref for Mat4<T> {
    type Target = [[T; 4]; 4];
    fn deref(&self) -> &[[T; 4]; 4] {
        &self.0
    }
}
impl<T> DerefMut for Mat4<T> {
    fn deref_mut(&mut self) -> &mut [[T; 4]; 4] {
        &mut self.0
    }
}

impl<T> Mat4<T> {
    pub fn as_flat_array(&self) -> &[T; 16] {
        unsafe { mem::transmute(self) }
    }
    pub fn as_flat_array_mut(&mut self) -> &mut [T; 16] {
        unsafe { mem::transmute(self) }
    }
}

impl<T: Num + Copy> Mat4<T> {
    /// Construct zero matrix.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let matrix = Mat4::<i32>::zero();
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert_eq!(matrix[i][j], 0);
    ///     }
    /// }
    /// ```
    pub fn zero() -> Self {
        Mat4([[T::zero(); 4]; 4])
    }

    /// Construct identity matrix.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let matrix = Mat4::<i32>::identity();
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert_eq!(matrix[i][j], if i == j { 1 } else { 0 });
    ///     }
    /// }
    /// ```
    pub fn identity() -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result[i][i] = T::one();
        }
        result
    }
}
