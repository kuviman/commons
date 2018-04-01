use ::*;

impl Color {
    /// Construct Color from RGB hex value.
    ///
    /// # Examples
    /// ```
    /// use color::*;
    /// let color = Color::rgb_hex(0xff0000); // Red
    /// ```
    pub fn rgb_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 16) & 0xff) as f32 / 255.0,
            g: ((hex >> 8) & 0xff) as f32 / 255.0,
            b: (hex & 0xff) as f32 / 255.0,
            a: 1.0,
        }
    }

    /// Construct Color from ARGB hex value.
    ///
    /// # Examples
    /// ```
    /// use color::*;
    /// let color = Color::argb_hex(0x80ffffff); // Semi-transparent white
    /// ```
    pub fn argb_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 16) & 0xff) as f32 / 255.0,
            g: ((hex >> 8) & 0xff) as f32 / 255.0,
            b: (hex & 0xff) as f32 / 255.0,
            a: (hex >> 24) as f32 / 255.0,
        }
    }
}
