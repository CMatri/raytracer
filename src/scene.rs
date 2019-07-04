use crate::ray::Ray;
use crate::vector::{Vec3, Vec2};

pub struct Color {
    pub r: u32,
    pub g: u32,
    pub b: u32
}

impl Color {
    pub fn black() -> &'static Color {
        &Color { r: 0x00, g: 0x00, b: 0x00 }
    }

    pub fn sky() -> &'static Color {
        &Color { r: 0x87, g: 0xCE, b: 0xFA }
    }
}

pub trait Solid {
    fn collides(&self, ray: &Ray) -> Option<f64>;
    fn color(&self) -> &Color;
}

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub color: Color
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
}

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f64,
    pub color: Color
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
}

pub struct Hit<'a> {
    pub dist: f64,
    pub obj: &'a Box<Solid>
}

impl<'a> Hit<'a> {
    pub fn new(dist: f64, obj: &'a Box<Solid>) -> Hit<'a> {
        Hit {
            dist,
            obj
        }
    }
}

pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub objects: Vec<Box<Solid>>
}

impl Viewport {
    fn trace(&self, ray: &Ray) -> Option<Hit> {
        self.objects.iter()
            .filter_map(|s| s.collides(ray).map(|d| Hit::new(d, s))) 
            .min_by(|i, j| i.dist.partial_cmp(&j.dist).unwrap())
    }

    pub fn render(&self, buffer: &mut Vec<u32>) {
        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + y * self.width;
                let ray = Ray::new(x, y, &self);
                let color = if let Some(hit) = self.trace(&ray) {
                    hit.obj.color()
                } else {
                    Color::sky()
                };

                buffer[i as usize] = color.r << 16 | color.g << 8 | color.b;
            }
        }
    }
}