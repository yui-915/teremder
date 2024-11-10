use crate::style;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub(crate) struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl From<&Pixel> for style::Color {
    fn from(p: &Pixel) -> Self {
        style::Color::Rgb {
            r: p.r,
            g: p.g,
            b: p.b,
        }
    }
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 255 }
    }

    pub const fn rgba_f(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
            a: (a.clamp(0.0, 1.0) * 255.0) as u8,
        }
    }

    pub const fn rgb_f(r: f32, g: f32, b: f32) -> Self {
        Self {
            r: (r.clamp(0.0, 1.0) * 255.0) as u8,
            g: (g.clamp(0.0, 1.0) * 255.0) as u8,
            b: (b.clamp(0.0, 1.0) * 255.0) as u8,
            a: 255,
        }
    }

    pub const fn hex(hex: u32) -> Self {
        Self {
            r: ((hex >> 16) & 0xFF) as u8,
            g: ((hex >> 8) & 0xFF) as u8,
            b: (hex & 0xFF) as u8,
            a: 255,
        }
    }

    pub const fn hex_a(hex: u32) -> Self {
        Self {
            r: ((hex >> 24) & 0xFF) as u8,
            g: ((hex >> 16) & 0xFF) as u8,
            b: ((hex >> 8) & 0xFF) as u8,
            a: (hex & 0xFF) as u8,
        }
    }
}

pub const RED: Color = Color::rgb(255, 0, 0);
pub const GREEN: Color = Color::rgb(0, 255, 0);
pub const BLUE: Color = Color::rgb(0, 0, 255);
pub const BLACK: Color = Color::rgb(0, 0, 0);
pub const WHITE: Color = Color::rgb(255, 255, 255);
