use ::*;

impl<T> Mat4<T> {
    /// Get transposed matrix.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let matrix = Mat4::translate(vec3(1, 2, 3));
    /// let matrix_transposed = matrix.transpose();
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert_eq!(matrix[i][j], matrix_transposed[j][i]);
    ///     }
    /// }
    /// ```
    pub fn transpose(mut self) -> Self {
        let mut result: Self = unsafe { mem::uninitialized() };
        for i in 0..4 {
            for j in 0..4 {
                mem::swap(&mut result[i][j], &mut self[j][i]);
            }
        }
        mem::forget(self);
        result
    }
}

impl<T: Float> Mat4<T> {
    /// Get inverse matrix.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let matrix = Mat4::<f64>::rotate_x(0.123);
    /// let matrix_inv = matrix.inverse();
    /// let mult = matrix * matrix_inv;
    /// for i in 0..4 {
    ///     for j in 0..4 {
    ///         assert!((mult[i][j] - if i == j { 1.0 } else { 0.0 }).abs() < 1e-5);
    ///     }
    /// }
    /// ```
    pub fn inverse(self) -> Self {
        let b00 = self[0][0] * self[1][1] - self[1][0] * self[0][1];
        let b01 = self[0][0] * self[2][1] - self[2][0] * self[0][1];
        let b02 = self[0][0] * self[3][1] - self[3][0] * self[0][1];
        let b03 = self[1][0] * self[2][1] - self[2][0] * self[1][1];
        let b04 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
        let b05 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
        let b06 = self[0][2] * self[1][3] - self[1][2] * self[0][3];
        let b07 = self[0][2] * self[2][3] - self[2][2] * self[0][3];
        let b08 = self[0][2] * self[3][3] - self[3][2] * self[0][3];
        let b09 = self[1][2] * self[2][3] - self[2][2] * self[1][3];
        let b10 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
        let b11 = self[2][2] * self[3][3] - self[3][2] * self[2][3];

        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        if det == T::zero() {
            Self::identity()
        } else {
            det = T::one() / det;

            Mat4::new([
                [
                    (self[1][1] * b11 - self[2][1] * b10 + self[3][1] * b09) * det,
                    (self[2][1] * b08 - self[0][1] * b11 - self[3][1] * b07) * det,
                    (self[0][1] * b10 - self[1][1] * b08 + self[3][1] * b06) * det,
                    (self[1][1] * b07 - self[0][1] * b09 - self[2][1] * b06) * det,
                ],
                [
                    (self[2][0] * b10 - self[1][0] * b11 - self[3][0] * b09) * det,
                    (self[0][0] * b11 - self[2][0] * b08 + self[3][0] * b07) * det,
                    (self[1][0] * b08 - self[0][0] * b10 - self[3][0] * b06) * det,
                    (self[0][0] * b09 - self[1][0] * b07 + self[2][0] * b06) * det,
                ],
                [
                    (self[1][3] * b05 - self[2][3] * b04 + self[3][3] * b03) * det,
                    (self[2][3] * b02 - self[0][3] * b05 - self[3][3] * b01) * det,
                    (self[0][3] * b04 - self[1][3] * b02 + self[3][3] * b00) * det,
                    (self[1][3] * b01 - self[0][3] * b03 - self[2][3] * b00) * det,
                ],
                [
                    (self[2][2] * b04 - self[1][2] * b05 - self[3][2] * b03) * det,
                    (self[0][2] * b05 - self[2][2] * b02 + self[3][2] * b01) * det,
                    (self[1][2] * b02 - self[0][2] * b04 - self[3][2] * b00) * det,
                    (self[0][2] * b03 - self[1][2] * b01 + self[2][2] * b00) * det,
                ],
            ])
        }
    }
}
