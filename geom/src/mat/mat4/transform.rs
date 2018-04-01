use ::*;

impl<T: Num + Copy> Mat4<T> {
    /// Construct a uniform scale matrix.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let matrix = Mat4::scale_uniform(2);
    /// assert_eq!(matrix * vec4(1, 2, 3, 1), vec4(2, 4, 6, 1));
    /// ```
    pub fn scale_uniform(factor: T) -> Self {
        Self::scale(vec3(factor, factor, factor))
    }

    /// Construct a scale matrix.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let matrix = Mat4::scale(vec3(1, 2, 3));
    /// assert_eq!(matrix * vec4(1, 2, 3, 1), vec4(1, 4, 9, 1));
    /// ```
    pub fn scale(factor: Vec3<T>) -> Self {
        let mut result = Self::zero();
        result[0][0] = factor.x;
        result[1][1] = factor.y;
        result[2][2] = factor.z;
        result[3][3] = T::one();
        result
    }

    /// Construct a translation matrix.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let matrix = Mat4::translate(vec3(3, 2, 1));
    /// assert_eq!(matrix * vec4(1, 2, 3, 1), vec4(4, 4, 4, 1));
    /// ```
    pub fn translate(dv: Vec3<T>) -> Self {
        let mut result = Self::identity();
        result[0][3] = dv.x;
        result[1][3] = dv.y;
        result[2][3] = dv.z;
        result
    }
}

impl<T: Float> Mat4<T> {
    /// Construct matrix rotating around x axis.
    pub fn rotate_x(angle: T) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[1][1] = cs;
        result[1][2] = -sn;
        result[2][1] = sn;
        result[2][2] = cs;
        result
    }

    /// Construct matrix rotating around y axis.
    pub fn rotate_y(angle: T) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[2][2] = cs;
        result[2][0] = -sn;
        result[0][2] = sn;
        result[0][0] = cs;
        result
    }

    /// Construct matrix rotating around z axis.
    pub fn rotate_z(angle: T) -> Self {
        let mut result = Self::identity();
        let cs = angle.cos();
        let sn = angle.sin();
        result[0][0] = cs;
        result[0][1] = -sn;
        result[1][0] = sn;
        result[1][1] = cs;
        result
    }
}
