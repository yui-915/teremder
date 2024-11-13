use std::{fmt, ops::*};

#[macro_export]
macro_rules! rect {
    ($x:expr, $y:expr, $w:expr, $h:expr) => {
        $crate::Rectangle::new($x as f32, $y as f32, $w as f32, $h as f32)
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rectangle {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn is_colliding_with(&self, other: &Self) -> bool {
        self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y
    }
}
