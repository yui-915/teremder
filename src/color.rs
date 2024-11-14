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

macro_rules! def {
    {$($name:ident($($value:expr),*);)*} => {$(
        pub const $name: Color = Color::rgb($($value),*);
    )*}
}

// based on raylib
def! {
    LIGHTGRAY  (200, 200, 200);
    GRAY       (130, 130, 130);
    DARKGRAY   (80,  80,  80 );
    YELLOW     (253, 249, 0  );
    GOLD       (255, 203, 0  );
    ORANGE     (255, 161, 0  );
    PINK       (255, 109, 194);
    RED        (230, 41,  55 );
    MAROON     (190, 33,  55 );
    GREEN      (0,   228, 48 );
    LIME       (0,   158, 47 );
    DARKGREEN  (0,   117, 44 );
    SKYBLUE    (102, 191, 255);
    BLUE       (0,   121, 241);
    DARKBLUE   (0,   82,  172);
    PURPLE     (200, 122, 255);
    VIOLET     (135, 60,  190);
    DARKPURPLE (112, 31,  126);
    BEIGE      (211, 176, 131);
    BROWN      (127, 106, 79 );
    DARKBROWN  (76,  63,  47 );
    WHITE      (255, 255, 255);
    BLACK      (0,   0,   0  );
    MAGENTA    (255, 0,   255);
}
pub const BLANK: Color = Color::rgba(0, 0, 0, 0);
