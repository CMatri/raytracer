extern crate minifb;

pub mod vector;
pub mod ray;
pub mod scene;
pub mod color;
pub mod geom;
pub mod light;

use minifb::{Key, WindowOptions, Window};
use vector::{Vec3};
use scene::{Scene, Viewport};
use geom::{Sphere, Plane};
use color::Color;
use light::{PointLight, DirectionalLight, Light};
use std::time::{SystemTime, UNIX_EPOCH, Duration};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut window = Window::new("Raytracer - ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut viewport = Viewport {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        fov: 75.0,
        scene: Scene {
            lights: vec![
                Light::Point(PointLight {
                    pos: Vec3 { x: 0.4, y: 0.5, z: -1.5 },
                    color: Color { r: 1.0, g: 1.0, b: 1.0 },
                    intensity: 22.0
                }),
                Light::Point(PointLight {
                    pos: Vec3 { x: 0.7, y: 2.1, z: -0.4 },
                    color: Color { r: 1.0, g: 1.0, b: 1.0 },
                    intensity: 34.0
                })
            ],
            objects: vec![
                Box::new(Sphere {
                    origin: Vec3 { x: 0.0,  y: 0.0, z: -3.0 },
                    radius: 0.5,
                    color: Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0
                    },
                    albedo: 0.83
                }),
                Box::new(Sphere {
                    origin: Vec3 { x: 1.7, y: 1.0, z: -2.5 },
                    radius: 0.65,
                    color: Color {
                        r: 1.0,
                        g: 0.0,
                        b: 0.0
                    },
                    albedo: 0.3
                }),
                Box::new(Sphere {
                    origin: Vec3 { x: -1.2, y: 0.8, z: -2.0 },
                    radius: 0.25,
                    color: Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0
                    },
                    albedo: 1.0
                }),
                Box::new(Plane {
                    origin: Vec3 { x: 0.0, y: -0.1, z: 0.0 },
                    normal: Vec3 { x: 0.0, y: -1.39, z: 0.1 },
                    color: Color {
                        r: 0.27,
                        g: 0.47,
                        b: 0.27
                    },
                    albedo: 0.4
                })
            ],
            update_func: Box::new(|o, t| {
                o[0].origin().x = (t * 3.0).cos() as f64;
                o[0].origin().y = (t * 2.0).sin() as f64;
                o[1].origin().x = (t * 1.5).sin() as f64;
                o[1].origin().y = (t * 3.5).cos() as f64;
                o[2].origin().x = (t * 2.0).cos() as f64;
                o[2].origin().y = (t * 4.0).sin() as f64;
            })        
        }
    };

    let mut t: f32 = 0.0;
    let mut t0 = SystemTime::now();
    let mut frames = 0;
    let one_sec = Duration::from_secs(1);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        frames += 1;
        t += if t < std::f32::consts::PI * 2.0 { 0.01 } else { -t };
        let buffer = viewport.render(t);
        window.update_with_buffer(&buffer).unwrap();
        
        if t0.elapsed().unwrap() >= one_sec {
            println!("{:?} fps", frames);
            frames = 0;
            t0 = SystemTime::now();
        }
    }
}
