use crate::ray::Ray;
use crate::vector::{Vec3};
use crate::color::Color;

pub trait Solid {
    fn collides(&self, ray: &Ray) -> Option<f64>;
    fn color(&self) -> &Color;
    fn normal(&self, pos: &Vec3) -> Vec3;
    fn albedo(&self) -> f32;
    fn origin(&mut self) -> &mut Vec3;
}

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub color: Color,
    pub albedo: f32
}

impl Solid for Plane {
    fn collides(&self, ray: &Ray) -> Option<f64> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.dir);
        if denom > 1e-6 {
            let v = self.origin - ray.origin;
            let dist = v.dot(&normal) / denom;
            if dist >= 0.0 {
                return Some(dist)
            }
        }
        None
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn normal(&self, _pos: &Vec3) -> Vec3 {
        -self.normal
    }

    fn albedo(&self) -> f32 {
        self.albedo
    }

    fn origin(&mut self) -> &mut Vec3 {
        &mut self.origin
    }
}

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub color: Color,
    pub albedo: f32,
}

impl Solid for Sphere {
    fn collides(&self, ray: &Ray) -> Option<f64> {
        let l: Vec3 = self.origin - ray.origin;
        let adj = l.dot(&ray.dir);
        let opp = l.dot(&l) - (adj * adj); // l dot l = |l|*|l|
        let radius2 = self.radius * self.radius;
        
        if opp > radius2 {
            return None;
        }

        let inner_adj = (radius2 - opp).sqrt();
        let t0 = adj - inner_adj;
        let t1 = adj + inner_adj; 
        
        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        Some(if t0 < t1 { t0 } else { t1 })
    }

    fn color(&self) -> &Color {
        &self.color
    }

    fn normal(&self, pos: &Vec3) -> Vec3 {
        (*pos - self.origin).normalize()
    }

    fn albedo(&self) -> f32 {
        self.albedo
    }

    fn origin(&mut self) -> &mut Vec3 {
        &mut self.origin
    }
}
