use crate::color::Color;
use crate::vector::Vec3;

pub struct PointLight {
    pub pos: Vec3,
    pub color: Color,
    pub intensity: f32
}

pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f32
}

pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight)
}

impl Light {
    pub fn color(&self) -> Color {
        match *self {
            Light::Directional(ref d) => d.color,
            Light::Point(ref p) => p.color
        }
    }

    pub fn inv_dir(&self, pos: &Vec3) -> Vec3 {
        match *self {
            Light::Directional(ref d) => -d.direction,
            Light::Point(ref p) => (p.pos - *pos).normalize()
        }
    }

    pub fn intensity(&self, pos: &Vec3) -> f32 {
        match *self {
            Light::Directional(ref d) => d.intensity,
            Light::Point(ref p) => {
                let r2 = (p.pos - *pos).norm() as f32;
                p.intensity / (4.0 * std::f32::consts::PI * r2)
            }
        }
    }

    pub fn distance(&self, pos: &Vec3) -> f64 {
        match *self {
            Light::Directional(_) => std::f64::INFINITY,
            Light::Point(ref p) => (p.pos - *pos).len()
        }
    }
}