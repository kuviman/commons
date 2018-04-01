use ::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec3<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub fn vec3<T>(x: T, y: T, z: T) -> Vec3<T> {
    Vec3 { x, y, z }
}

impl<T> From<[T; 3]> for Vec3<T> {
    fn from(mut v: [T; 3]) -> Vec3<T> {
        // TODO: just transmute?
        let x = unsafe { mem::replace(&mut v[0], mem::uninitialized()) };
        let y = unsafe { mem::replace(&mut v[1], mem::uninitialized()) };
        let z = unsafe { mem::replace(&mut v[2], mem::uninitialized()) };
        mem::forget(v);
        vec3(x, y, z)
    }
}

impl<T> Deref for Vec3<T> {
    type Target = [T; 3];
    fn deref(&self) -> &[T; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> DerefMut for Vec3<T> {
    fn deref_mut(&mut self) -> &mut [T; 3] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> Vec3<T> {
    pub fn extend(self, w: T) -> Vec4<T> {
        vec4(self.x, self.y, self.z, w)
    }
}

impl<T: Copy + Num> Vec3<T> {
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
}

impl<T: Float> Vec3<T> {
    pub fn normalize(self) -> Self {
        self / self.len()
    }
    pub fn len(self) -> T {
        T::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }
}
