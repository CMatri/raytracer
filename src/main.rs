extern crate minifb;

pub mod vector;
pub mod ray;
pub mod scene;
pub mod color;

use minifb::{Key, WindowOptions, Window};
use vector::{Vec2, Vec3};
use scene::{Plane, Sphere, Viewport, Solid, Light};
use color::Color;

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
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
        light: Light {
            direction: Vec3 { x: 0.2, y: -0.3, z: -0.3 },
            color: Color { r: 1.0, g: 1.0, b: 1.0 },
            intensity: 4.0
        },
        objects: vec![
            Box::new(Sphere {
                origin: Vec3 { x: 0.0,  y: 0.0, z: -5.0 },
                radius: 1.0,
                color: Color {
                    r: 0.0,
                    g: 1.0,
                    b: 0.0
                },
                albedo: 1.0
            }),
            Box::new(Sphere {
                origin: Vec3 { x: 1.7, y: 1.0, z: -3.0 },
                radius: 0.75,
                color: Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0
                },
                albedo: 1.0
            }),
            Box::new(Sphere {
                origin: Vec3 { x: -1.2, y: 0.8, z: -4.0 },
                radius: 0.85,
                color: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 1.0
                },
                albedo: 1.0
            }),
            Box::new(Plane {
                origin: Vec3 { x: 0.0, y: -2.0, z: 0.0 },
                normal: Vec3 { x: 0.0, y: -1.0, z: 0.0 },
                color: Color {
                    r: 0.27,
                    g: 0.27,
                    b: 0.27
                },
                albedo: 1.0
            })
        ],
        update_func: Box::new(|o, t| {
            o[0].origin().y = (t * 10.0).sin() as f64;
        })
    };

    let mut t: f32 = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        t += if t < std::f32::consts::PI * 2.0 { 0.01 } else { -t };
        viewport.update(t);
        viewport.render(&mut buffer);
        window.update_with_buffer(&buffer).unwrap();
    }
}
