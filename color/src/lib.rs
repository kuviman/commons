#![deny(warnings)]

extern crate prelude;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

macro_rules! rgb {
    ($r: expr, $g: expr, $b: expr) => {
        rgb!($r, $g, $b, 1.0)
    };
    ($r: expr, $g: expr, $b: expr, $a: expr) => {
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

    pub fn rgb_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 16) & 0xff) as f32 / 255.0,
            g: ((hex >> 8) & 0xff) as f32 / 255.0,
            b: (hex & 0xff) as f32 / 255.0,
            a: 1.0,
        }
    }

    pub fn argb_hex(hex: u32) -> Self {
        Color {
            r: ((hex >> 16) & 0xff) as f32 / 255.0,
            g: ((hex >> 8) & 0xff) as f32 / 255.0,
            b: (hex & 0xff) as f32 / 255.0,
            a: (hex >> 24) as f32 / 255.0,
        }
    }

    pub fn hsv(h: f32, s: f32, v: f32) -> Self {
        Self::hsva(h, s, v, 1.0)
    }

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
