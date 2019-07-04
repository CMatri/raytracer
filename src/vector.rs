use std::ops::{Add, Sub, Mul, Neg};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64
}

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec2 {
    pub fn from(num: f64) -> Vec2 {
        Vec2 { x: num, y: num }
    }

    pub fn zero() -> Vec2 { Self::from(0f64) }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y)
    }

    pub fn len(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        (self.x * v.x + self.y * v.y)
    }

    pub fn normalize(&self) -> Vec2 {
        let inv_len = self.len().recip();
        Vec2 {
            x: self.x * inv_len,
            y: self.y * inv_len
        }
    }
}

impl Vec3 {
    pub fn from(num: f64) -> Vec3 {
        Vec3 { x: num, y: num, z: num }
    }

    pub fn zero() -> Vec3 { Self::from(0f64) }

    pub fn norm(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z)
    }

    pub fn len(&self) -> f64 {
        self.norm().sqrt()
    }

    pub fn dot(&self, v: &Vec3) -> f64 {
        (self.x * v.x + self.y * v.y + self.z * v.z)
    }

    pub fn normalize(&self) -> Vec3 {
        let inv_len = self.len().recip();
        Vec3 {
            x: self.x * inv_len,
            y: self.y * inv_len,
            z: self.z * inv_len
        }
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul for Vec2 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * v.x,
            y: self.y * v.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    
    fn mul(self, s: f64) -> Vec2 {
        Vec2 {
            x: self.x * s,
            y: self.y * s
        }
    }
}

impl Mul<Vec2> for f64 {
    type Output = Vec2;

    fn mul(self, v: Vec2) -> Vec2 {
        self * v
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * v.x,
            y: self.y * v.y,
            z: self.z * v.z
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    
    fn mul(self, s: f64) -> Vec3 {
        Vec3 {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        self * v
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}