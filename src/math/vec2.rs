use std::{fmt, ops::*};

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        $crate::Vec2::new($x as f32, $y as f32)
    };
}

macro_rules! def {
    {$(
        $ident:ident($x:expr, $y:expr);
    )*} => {$(
        pub const $ident: Vec2 = Vec2 { x: $x, y: $y };
    )*}
}

macro_rules! self_mut {
    ($self:ident.$fn:ident($($arg:expr),*)) => {{
        *$self = $self.$fn($($arg),*);
        $self
    }}
}

macro_rules! impl_ops {
    {
        for $struct:ident;
        scalar method is $scalar_method:ident(...);
        $(
            $trait:ident
            $fn:ident
            $sub_trait:ident
            $sub_fn:ident
            |$self:ident, $other:ident|
            ($($expr:expr),*);
        )*
    } => {$(
        impl $trait<$struct> for $struct {
            type Output = $struct;
            fn $fn(self, $other: $struct) -> $struct {
                let $self = self;
                $struct::new($($expr),*)
            }
        }
        impl $trait<&$struct> for $struct {
            type Output = $struct;
            fn $fn(self, $other: &$struct) -> $struct {
                let $self = self;
                $struct::new($($expr),*)
            }
        }
        impl $trait<$struct> for &$struct {
            type Output = $struct;
            fn $fn(self, $other: $struct) -> $struct {
                let $self = self;
                $struct::new($($expr),*)
            }
        }
        impl $trait<&$struct> for &$struct {
            type Output = $struct;
            fn $fn(self, $other: &$struct) -> $struct {
                let $self = self;
                $struct::new($($expr),*)
            }
        }
        impl $trait<f32> for $struct {
            type Output = $struct;
            fn $fn(self, other: f32) -> $struct {
                let $self = self;
                let $other = $struct::$scalar_method(other);
                $struct::new($($expr),*)
            }
        }
        impl $trait<f32> for &$struct {
            type Output = $struct;
            fn $fn(self, other: f32) -> $struct {
                let $self = self;
                let $other = $struct::$scalar_method(other);
                $struct::new($($expr),*)
            }
        }
        impl $trait<$struct> for f32 {
            type Output = $struct;
            fn $fn(self, $other: $struct) -> $struct {
                let $self = $struct::$scalar_method(self);
                $struct::new($($expr),*)
            }
        }
        impl $trait<&$struct> for f32 {
            type Output = $struct;
            fn $fn(self, $other: &$struct) -> $struct {
                let $self = $struct::$scalar_method(self);
                $struct::new($($expr),*)
            }
        }
        impl $sub_trait<$struct> for $struct {
            fn $sub_fn(&mut self, $other: $struct) {
                let $self = self;
                *$self = $struct::new($($expr),*);
            }
        }
        impl $sub_trait<&$struct> for $struct {
            fn $sub_fn(&mut self, $other: &$struct)  {
                let $self = self;
                *$self = $struct::new($($expr),*);
            }
        }
        impl $sub_trait<f32> for $struct {
            fn $sub_fn(&mut self, other: f32)  {
                let $self = self;
                let $other = $struct::$scalar_method(other);
                *$self = $struct::new($($expr),*);
            }
        }
    )*}
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    def! {
        ZERO(0., 0.);
        ONE(1., 1.);
        NEG_ONE(-1., -1.);
        X(1., 0.);
        Y(0., 1.);
        NEG_X(-1., 0.);
        NEG_Y(0., -1.);
        MIN(f32::MIN, f32::MIN);
        MAX(f32::MAX, f32::MAX);
        NAN(f32::NAN, f32::NAN);
        INFINITY(f32::INFINITY, f32::INFINITY);
        NEG_INFINITY(f32::NEG_INFINITY, f32::NEG_INFINITY);
        RIGHT(1., 0.);
        LEFT(-1., 0.);
        UP(0., -1.);
        DOWN(0., 1.);
    }

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn splat(n: f32) -> Self {
        Self::new(n, n)
    }

    pub fn angled(angle: f32) -> Self {
        Self::new(angle.cos(), angle.sin())
    }

    pub fn with_x(&self, x: f32) -> Self {
        Self::new(x, self.y)
    }

    pub fn with_y(&self, y: f32) -> Self {
        Self::new(self.x, y)
    }

    pub fn set_x(&mut self, x: f32) -> &mut Self {
        self.x = x;
        self
    }

    pub fn set_y(&mut self, y: f32) -> &mut Self {
        self.y = y;
        self
    }

    pub fn normalize(&self) -> Self {
        let len = self.length();
        if len > 0. {
            Self::new(self.x / len, self.y / len)
        } else {
            Self::ZERO
        }
    }

    pub fn self_normalize(&mut self) -> &mut Self {
        self_mut!(self.normalize())
    }

    pub fn length(&self) -> f32 {
        self.x.hypot(self.y)
    }

    pub fn angle_from(&self, other: &Self) -> f32 {
        (self.y - other.y).atan2(self.x - other.x)
    }

    pub fn angle_to(&self, other: &Self) -> f32 {
        other.angle_from(self)
    }

    pub fn angle(&self) -> f32 {
        self.angle_from(&Self::ZERO)
    }

    pub fn floor(&self) -> Self {
        Self::new(self.x.floor(), self.y.floor())
    }

    pub fn ceil(&self) -> Self {
        Self::new(self.x.ceil(), self.y.ceil())
    }

    pub fn round(&self) -> Self {
        Self::new(self.x.round(), self.y.round())
    }

    pub fn abs(&self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }

    pub fn trunc(&self) -> Self {
        Self::new(self.x.trunc(), self.y.trunc())
    }

    pub fn self_floor(&mut self) -> &mut Self {
        self_mut!(self.floor())
    }

    pub fn self_ceil(&mut self) -> &mut Self {
        self_mut!(self.ceil())
    }

    pub fn self_round(&mut self) -> &mut Self {
        self_mut!(self.round())
    }

    pub fn self_abs(&mut self) -> &mut Self {
        self_mut!(self.abs())
    }

    pub fn self_trunc(&mut self) -> &mut Self {
        self_mut!(self.trunc())
    }

    pub fn is_finite(&self) -> bool {
        self.x.is_finite() && self.y.is_finite()
    }

    pub fn is_nan(&self) -> bool {
        self.x.is_nan() || self.y.is_nan()
    }

    pub fn is_normalized(&self) -> bool {
        let len = self.length();
        len.is_finite() && len == 1.
    }

    pub fn min(&self, other: &Self) -> Self {
        Self::new(self.x.min(other.x), self.y.min(other.y))
    }

    pub fn self_min(&mut self, other: &Self) -> &mut Self {
        self_mut!(self.min(other))
    }

    pub fn max(&self, other: &Self) -> Self {
        Self::new(self.x.max(other.x), self.y.max(other.y))
    }

    pub fn self_max(&mut self, other: &Self) -> &mut Self {
        self_mut!(self.max(other))
    }

    pub fn clamp(&self, min: &Self, max: &Self) -> Self {
        Self::new(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y))
    }

    pub fn self_clamp(&mut self, min: &Self, max: &Self) -> &mut Self {
        self_mut!(self.clamp(min, max))
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn yx(&self) -> Self {
        Self::new(self.y, self.x)
    }

    pub fn self_yx(&mut self) -> &mut Self {
        self_mut!(self.yx())
    }

    pub fn scale(&self, factor: f32) -> Self {
        Self::new(self.x * factor, self.y * factor)
    }

    pub fn self_scale(&mut self, factor: f32) -> &mut Self {
        self_mut!(self.scale(factor))
    }

    pub fn distance(&self, other: &Self) -> f32 {
        (self - *other).length().abs()
    }

    pub fn rotate_around(&self, pivot: &Self, angle: f32) -> Self {
        let (dx, dy) = (self.x - pivot.x, self.y - pivot.y);
        let x = angle.cos() * dx - angle.sin() * dy + pivot.x;
        let y = angle.sin() * dx + angle.cos() * dy + pivot.y;
        Self::new(x, y)
    }

    pub fn self_rotate_around(&mut self, pivot: &Self, angle: f32) -> &mut Self {
        self_mut!(self.rotate_around(pivot, angle))
    }

    pub fn rotate(&self, angle: f32) -> Self {
        self.rotate_around(&Self::ZERO, angle)
    }

    pub fn self_rotate(&mut self, angle: f32) -> &mut Self {
        self_mut!(self.rotate(angle))
    }

    pub fn lerp(&self, other: &Self, t: f32) -> Self {
        self + (other - self) * t
    }

    pub fn self_lerp(&mut self, other: &Self, t: f32) -> &mut Self {
        self_mut!(self.lerp(other, t))
    }

    pub fn move_towards(&self, other: &Self, distance: f32) -> Self {
        let angle = self.angle_to(other);
        let dx = distance * angle.cos();
        let dy = distance * angle.sin();
        self + Self::new(dx, dy)
    }

    pub fn self_move_towards(&mut self, other: &Self, distance: f32) -> &mut Self {
        self_mut!(self.move_towards(other, distance))
    }

    pub fn midpoint(&self, other: &Self) -> Self {
        self.lerp(other, 0.5)
    }

    pub fn self_midpoint(&mut self, other: &Self) -> &mut Self {
        self_mut!(self.midpoint(other))
    }

    pub fn clamp_length(&self, min: f32, max: f32) -> Self {
        let len = self.length();
        if len < min {
            self * min / len
        } else if len > max {
            self * max / len
        } else {
            *self
        }
    }

    pub fn self_clamp_length(&mut self, min: f32, max: f32) -> &mut Self {
        self_mut!(self.clamp_length(min, max))
    }

    pub fn clamp_length_max(&self, max: f32) -> Self {
        self.clamp_length(0., max)
    }

    pub fn self_clamp_length_max(&mut self, max: f32) -> &mut Self {
        self_mut!(self.clamp_length_max(max))
    }

    pub fn clamp_length_min(&self, min: f32) -> Self {
        self.clamp_length(min, f32::MAX)
    }

    pub fn self_clamp_length_min(&mut self, min: f32) -> &mut Self {
        self_mut!(self.clamp_length_min(min))
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl_ops! {
    for Vec2;
    scalar method is splat(...);
    Add add AddAssign add_assign |a, b| (a.x + b.x, a.y + b.y);
    Sub sub SubAssign sub_assign |a, b| (a.x - b.x, a.y - b.y);
    Mul mul MulAssign mul_assign |a, b| (a.x * b.x, a.y * b.y);
    Div div DivAssign div_assign |a, b| (a.x / b.x, a.y / b.y);
}
