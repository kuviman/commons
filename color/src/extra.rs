use ::*;

impl Color {
    /// Calculate distance between colors in RGBA space.
    ///
    /// # Examples
    /// ```
    /// use color::*;
    /// let color1 = Color::rgb(1.0, 0.0, 0.0);
    /// let color2 = Color::hsv(0.0, 1.0, 1.0);
    /// assert!(color1.distance_to(color2) < 1e-5);
    /// ```
    pub fn distance_to(self, other: Color) -> f32 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;
        let da = self.a - other.a;
        (dr * dr + dg * dg + db * db + da * da).sqrt()
    }
}
