use std::ops::{Add, Sub, Mul, Neg};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32
}

impl Color {
    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
        }
    }

    pub fn black() -> &'static Color {
        &Color { r: 0.0, g: 0.0, b: 0.0 }
    }

    pub fn sky() -> &'static Color {
        &Color { r: 0.0, g: 0.0, b: 0.0 }
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b
        }
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            r: self.r - other.r,
            g: self.g - other.g,
            b: self.b - other.b
        }
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, v: Self) -> Self {
        Self {
            r: self.r * v.r,
            g: self.g * v.g,
            b: self.b * v.b
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, c: Color) -> Color {
        c * self
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    
    fn mul(self, s: f32) -> Self {
        Self {
            r: self.r * s,
            g: self.g * s,
            b: self.b * s
        }
    }
}

impl Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            r: 1.0 - self.r,
            g: 1.0 - self.g,
            b: 1.0 - self.b
        }
    }
}