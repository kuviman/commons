use ::*;

#[repr(C)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Vec4<T = f64> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

pub fn vec4<T>(x: T, y: T, z: T, w: T) -> Vec4<T> {
    Vec4 { x, y, z, w }
}

impl<T> From<[T; 4]> for Vec4<T> {
    fn from(mut v: [T; 4]) -> Vec4<T> {
        // TODO: just transmute?
        let x = unsafe { mem::replace(&mut v[0], mem::uninitialized()) };
        let y = unsafe { mem::replace(&mut v[1], mem::uninitialized()) };
        let z = unsafe { mem::replace(&mut v[2], mem::uninitialized()) };
        let w = unsafe { mem::replace(&mut v[3], mem::uninitialized()) };
        mem::forget(v);
        vec4(x, y, z, w)
    }
}

impl<T> Deref for Vec4<T> {
    type Target = [T; 4];
    fn deref(&self) -> &[T; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<T> DerefMut for Vec4<T> {
    fn deref_mut(&mut self) -> &mut [T; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl<T: Copy + num::Num> Vec4<T> {
    pub fn dot(a: Self, b: Self) -> T {
        a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w
    }
}
