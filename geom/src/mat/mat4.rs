use ::*;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mat4<T = f64>([[T; 4]; 4]);

impl<T> Mat4<T> {
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

impl<T: Num + Copy> Add for Mat4<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        let mut result = self;
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = result[i][j] + rhs[i][j];
            }
        }
        result
    }
}

impl<T: Num + Copy + AddAssign> AddAssign for Mat4<T> {
    fn add_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            for j in 0..4 {
                self[i][j] += rhs[i][j];
            }
        }
    }
}

impl<T: Num + Copy> Sub for Mat4<T> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        let mut result = self;
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = result[i][j] - rhs[i][j];
            }
        }
        result
    }
}

impl<T: Num + Copy + SubAssign> SubAssign for Mat4<T> {
    fn sub_assign(&mut self, rhs: Self) {
        for i in 0..4 {
            for j in 0..4 {
                self[i][j] -= rhs[i][j];
            }
        }
    }
}

impl<T: Num + Copy + Neg<Output = T>> Neg for Mat4<T> {
    type Output = Self;
    fn neg(self) -> Self {
        let mut result = self;
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = -result[i][j];
            }
        }
        result
    }
}

impl<T: Num + Copy + AddAssign> Mul for Mat4<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        let mut result = Mat4([[T::zero(); 4]; 4]);
        for i in 0..4 {
            for j in 0..4 {
                let cur = &mut result[i][j];
                for t in 0..4 {
                    *cur += self[i][t] * rhs[t][j];
                }
            }
        }
        result
    }
}

impl<T: Num + Copy + AddAssign> MulAssign for Mat4<T> {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T: Num + Copy> Mul<T> for Mat4<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        let mut result = self;
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = result[i][j] * rhs;
            }
        }
        result
    }
}

impl<T: Num + Copy + MulAssign> MulAssign<T> for Mat4<T> {
    fn mul_assign(&mut self, rhs: T) {
        for i in 0..4 {
            for j in 0..4 {
                self[i][j] *= rhs;
            }
        }
    }
}

impl<T: Num + Copy> Div<T> for Mat4<T> {
    type Output = Self;
    fn div(self, rhs: T) -> Self {
        let mut result = self;
        for i in 0..4 {
            for j in 0..4 {
                result[i][j] = result[i][j] / rhs;
            }
        }
        result
    }
}

impl<T: Num + Copy + DivAssign> DivAssign<T> for Mat4<T> {
    fn div_assign(&mut self, rhs: T) {
        for i in 0..4 {
            for j in 0..4 {
                self[i][j] /= rhs;
            }
        }
    }
}

impl<T: Float> Mul<Vec4<T>> for Mat4<T> {
    type Output = Vec4<T>;

    fn mul(self, rhs: Vec4<T>) -> Vec4<T> {
        vec4(
            Vec4::dot(self[0].into(), rhs),
            Vec4::dot(self[1].into(), rhs),
            Vec4::dot(self[2].into(), rhs),
            Vec4::dot(self[3].into(), rhs),
        )
    }
}

impl<T> Mat4<T> {
    pub fn transpose(mut self) -> Self {
        let mut result: Self = unsafe { mem::uninitialized() };
        for i in 0..4 {
            for j in 0..4 {
                mem::swap(&mut result[i][j], &mut self[i][j]);
            }
        }
        mem::forget(self);
        result
    }
}

impl<T: Num + Copy> Mat4<T> {
    pub fn zero() -> Self {
        Mat4([[T::zero(); 4]; 4])
    }
    pub fn identity() -> Self {
        let mut result = Self::zero();
        for i in 0..4 {
            result[i][i] = T::one();
        }
        result
    }

    pub fn scale_uniform(factor: T) -> Self {
        Self::scale(vec3(factor, factor, factor))
    }
    pub fn scale(factor: Vec3<T>) -> Self {
        let mut result = Self::zero();
        result[0][0] = factor.x;
        result[1][1] = factor.y;
        result[2][2] = factor.z;
        result[3][3] = T::one();
        result
    }

    pub fn translate(dv: Vec3<T>) -> Self {
        let mut result = Self::identity();
        result[0][3] = dv.x;
        result[1][3] = dv.y;
        result[2][3] = dv.z;
        result
    }
}

impl<T: Float> Mat4<T> {
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

            Mat4([
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

    pub fn perspective(fov: T, aspect: T, near: T, far: T) -> Self {
        let ymax = near * (fov / (T::one() + T::one())).tan();
        let xmax = ymax * aspect;
        Self::frustum(-xmax, xmax, -ymax, ymax, near, far)
    }

    pub fn frustum(left: T, right: T, bottom: T, top: T, near: T, far: T) -> Self {
        let double_near = near + near;
        let width = right - left;
        let height = top - bottom;
        let depth = far - near;
        Mat4([
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
