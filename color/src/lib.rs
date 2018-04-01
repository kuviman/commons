//! Provides Color definition and operations

#![deny(warnings)]

extern crate prelude;

pub(crate) use prelude::*;

mod consts;
mod hsv;
mod hex;
mod extra;

/// RGBA color. Component values should be in range [0.0, 1.0].
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    /// Construct Color with given RGB components. Alpha value will be equal to 1.0.
    ///
    /// # Examples
    /// ```
    /// use color::*;
    /// let color = Color::rgb(1.0, 1.0, 0.0); // Yellow
    /// ```
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b, a: 1.0 }
    }

    /// Construct Color with given RGBA components.
    ///
    /// # Examples
    /// ```
    /// use color::*;
    /// let color = Color::rgba(1.0, 1.0, 1.0, 0.0); // Transparent white
    /// ```
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}

impl Deref for Color {
    type Target = [f32; 4];
    fn deref(&self) -> &[f32; 4] {
        unsafe { mem::transmute(self) }
    }
}

impl DerefMut for Color {
    fn deref_mut(&mut self) -> &mut [f32; 4] {
        unsafe { mem::transmute(self) }
    }
}
