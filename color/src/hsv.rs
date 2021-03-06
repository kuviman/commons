use ::*;

impl Color {
    /// Construct Color converting from HSV to RGB. Alpha value will be equal to 1.0.
    ///
    /// # Examples
    /// ```
    /// use color::*;
    /// let color = Color::hsv(0.33, 1.0, 1.0); // Green
    /// ```
    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        Self::hsva(h, s, v, 1.0)
    }

    /// Construct Color converting from HSV to RGB and given alpha value.
    ///
    /// # Examples
    /// ```
    /// use color::*;
    /// let color = Color::hsva(0.5, 1.0, 1.0, 0.5);
    /// ```
    pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Self {
        let h = (h - h.floor()) * 6.0;
        let r;
        let g;
        let b;
        let f = h - h.floor();
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);
        if h < 1.0 {
            r = v;
            g = t;
            b = p;
        } else if h < 2.0 {
            r = q;
            g = v;
            b = p;
        } else if h < 3.0 {
            r = p;
            g = v;
            b = t;
        } else if h < 4.0 {
            r = p;
            g = q;
            b = v;
        } else if h < 5.0 {
            r = t;
            g = p;
            b = v;
        } else {
            r = v;
            g = p;
            b = q;
        }
        Color { r, g, b, a }
    }
}
