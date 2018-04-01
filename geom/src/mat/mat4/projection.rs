use ::*;

impl<T: Float> Mat4<T> {
    /// Construct prespective projection matrix.
    pub fn perspective(fov: T, aspect: T, near: T, far: T) -> Self {
        let ymax = near * (fov / (T::one() + T::one())).tan();
        let xmax = ymax * aspect;
        Self::frustum(-xmax, xmax, -ymax, ymax, near, far)
    }

    /// Construct frustum projection matrix.
    pub fn frustum(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let double_near = near + near;
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        Mat4::new([
            [
                double_near / width,
                T::zero(),
                (right + left) / width,
                T::zero(),
            ],
            [
                T::zero(),
                double_near / height,
                (top + bottom) / height,
                T::zero(),
            ],
            [
                T::zero(),
                T::zero(),
                (-far - near) / depth,
                (-double_near * far) / depth,
            ],
            [T::zero(), T::zero(), -T::one(), T::zero()],
        ])
    }
}
