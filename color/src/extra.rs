use ::*;

impl Color {
    pub fn distance_to(self, other: Color) -> f32 {
        let dr = self.r - other.r;
        let dg = self.g - other.g;
        let db = self.b - other.b;
        let da = self.a - other.a;
        (dr * dr + dg * dg + db * db + da * da).sqrt()
    }
}
