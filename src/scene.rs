use crate::ray::Ray;
use crate::vector::{Vec3, Vec2};
use crate::color::Color;

pub struct Light {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f32
}

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

    fn normal(&self, pos: &Vec3) -> Vec3 {
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
    pub objects: Vec<Box<Solid>>,
    pub light: Light,
    pub update_func: Box<FnMut(&mut Vec<Box<Solid>>, f32)>
}

impl Viewport {
    fn trace(&self, ray: &Ray) -> Option<Hit> {
        self.objects.iter()
            .filter_map(|s| s.collides(ray).map(|d| Hit::new(d, s))) 
            .min_by(|i, j| i.dist.partial_cmp(&j.dist).unwrap())
    }

    fn trace_color(&self, ray: &Ray, hit: &Hit) -> Color {
        let hit_pos = ray.origin + (ray.dir * hit.dist);
        let normal = hit.obj.normal(&hit_pos);
        let inv_light_dir = -self.light.direction.normalize();
        let light_intensity = (normal.dot(&inv_light_dir) as f32).max(0.0) * self.light.intensity; // dot product stand  in for lambert cosine law possible due to normalized vec length 
        let light_reflected = hit.obj.albedo() / std::f32::consts::PI;
        let color = *hit.obj.color() * self.light.color * light_intensity * light_reflected;
        color.clamp()
    }
    
    pub fn update(&mut self, t: f32) {
         (self.update_func)(&mut self.objects, t);
    }

    pub fn render(&mut self, buffer: &mut Vec<u32>) {
        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + y * self.width;
                let ray = Ray::new(x, y, &self);
                let color = if let Some(hit) = self.trace(&ray) {
                    self.trace_color(&ray, &hit)
                } else {
                    *Color::sky()
                };

                buffer[i as usize] = ((color.r * 255.0) as u32) << 16 | ((color.g * 255.0) as u32) << 8 | ((color.b * 255.0) as u32);
            }
        }
    }
}