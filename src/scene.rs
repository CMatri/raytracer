use crate::ray::Ray;
use crate::vector::{Vec3};
use crate::color::Color;
use crate::geom::{Solid};
use crate::light::{PointLight, DirectionalLight, Light};

pub struct Hit<'a> {
    pub dist: f64,
    pub obj: &'a Box<Solid + Send + Sync>
}

impl<'a> Hit<'a> {
    pub fn new(dist: f64, obj: &'a Box<Solid + Send + Sync>) -> Hit<'a> {
        Hit {
            dist,
            obj
        }
    }
}

pub struct Scene {
    pub objects: Vec<Box<Solid + Send + Sync>>,
    pub lights: Vec<Light>,
    pub update_func: Box<FnMut(&mut Vec<Box<Solid + Send + Sync>>, f32) + Send + Sync>
}

impl Scene {
    fn trace(&self, ray: &Ray) -> Option<Hit> {
        self.objects.iter()
            .filter_map(|s| s.collides(ray).map(|d| Hit::new(d, s))) 
            .min_by(|i, j| i.dist.partial_cmp(&j.dist).unwrap())
    }

    fn trace_color(&self, ray: &Ray, hit: &Hit) -> Color {
        let hit_pos = ray.origin + (ray.dir * hit.dist);
        let normal = hit.obj.normal(&hit_pos);
        let mut color = *Color::black();

        for light in &self.lights {
            let inv_light_dir = light.inv_dir(&hit_pos);
            let shadow_ray = Ray {
                origin: hit_pos + (inv_light_dir * 1e-13),
                dir: inv_light_dir
            };
            let shadow_intersection = self.trace(&shadow_ray);
            let in_light = shadow_intersection.is_none() || shadow_intersection.unwrap().dist > light.distance(&hit_pos);        
            let light_intensity = if in_light { (normal.dot(&inv_light_dir) as f32).max(0.0) * light.intensity(&hit_pos) } else { 0.0 }; // dot product stand  in for lambert cosine law possible due to normalized vec length 
            let light_reflected = hit.obj.albedo() / std::f32::consts::PI;
            color = color + *hit.obj.color() * light.color() * light_intensity * light_reflected;
        }
        color.clamp()
    }
}

pub struct Viewport {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub scene: Scene
}

impl Viewport {
    pub fn render(&mut self, buffer: &mut Vec<u32>, t: f32) {
        (self.scene.update_func)(&mut self.scene.objects, t);
        
        /*crossbeam::scope(|scope| { // ugh, just completely rewrite this multithreading attempt
            let nthreads = self.height;
            let mut threads = vec![];
            for t in 0..nthreads {
                let y = Arc::new(t);
                let width = Arc::new(&self.width);
                let scene = Arc::new(Mutex::new(&self.scene));
                let s = Arc::new(Mutex::new(&self));
                let final_buf = Arc::new(Mutex::new(Vec::new()));

                threads.push(scope.spawn(move |_| {
                    let thread_y = y.clone();
                    let thread_buf = final_buf.clone();

                    let mut buf = vec![];
                    for x in 0..**width {
                        let i = x;//x + y * self.width;
                        let ray = Ray::new(x, *y, &s.lock().unwrap());
                        let color = if let Some(hit) = scene.lock().unwrap().trace(&ray) {
                            scene.lock().unwrap().trace_color(&ray, &hit)
                        } else {
                            *Color::sky()
                        };
                        buf.push(((color.r * 255.0) as u32) << 16 | ((color.g * 255.0) as u32) << 8 | ((color.b * 255.0) as u32));
                    }

                    final_buf.lock().unwrap().append(&mut buf);
                }));
            }
        });*/
        for x in 0..self.width {
            for y in 0..self.height {
                let i = x + y * self.width;
                let ray = Ray::new(x, y, &self);
                let color = if let Some(hit) = self.scene.trace(&ray) {
                    self.scene.trace_color(&ray, &hit)
                } else {
                    *Color::sky()
                };
                buffer[i as usize] = ((color.r * 255.0) as u32) << 16 | ((color.g * 255.0) as u32) << 8 | ((color.b * 255.0) as u32);
            }
        }
    }
}