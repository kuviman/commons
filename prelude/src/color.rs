#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => { rgb!($r, $g, $b, 1.0) };
    ($r:expr, $g:expr, $b:expr, $a: expr) => {
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

    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Color { r, g, b, a: 1.0 }
    }
    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }

    pub fn argb_hex(value: u32) -> Self {
        Color {
            r: ((value >> 16) & 0xff) as f32 / 255.0,
            g: ((value >> 8) & 0xff) as f32 / 255.0,
            b: (value & 0xff) as f32 / 255.0,
            a: (value >> 24) as f32 / 255.0,
        }
    }
}
