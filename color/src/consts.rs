use ::*;

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        rgb!($r, $g, $b, 1.0)
    };
    ($r:expr, $g:expr, $b:expr, $a:expr) => {
        Color {
            r: $r,
            g: $g,
            b: $b,
            a: $a,
        }
    };
}

impl Color {
    pub const BLACK: Color = rgb!(0.0, 0.0, 0.0);
    pub const WHITE: Color = rgb!(1.0, 1.0, 1.0);

    pub const RED: Color = rgb!(1.0, 0.0, 0.0);
    pub const GREEN: Color = rgb!(0.0, 1.0, 0.0);
    pub const BLUE: Color = rgb!(0.0, 0.0, 1.0);

    pub const YELLOW: Color = rgb!(1.0, 1.0, 0.0);
    pub const MAGENTA: Color = rgb!(1.0, 0.0, 1.0);
    pub const CYAN: Color = rgb!(0.0, 1.0, 1.0);
}
