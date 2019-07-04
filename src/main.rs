extern crate minifb;

pub mod vector;
pub mod ray;
pub mod scene;

use minifb::{Key, WindowOptions, Window};
use vector::{Vec2, Vec3};
use scene::{Plane, Sphere, Viewport, Color, Solid};

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

    let viewport = Viewport {
        width: WIDTH as u32,
        height: HEIGHT as u32,
        fov: 75.0,
        objects: vec![
            Box::new(Sphere {
                origin: Vec3 { x: 0.0,  y: 0.0, z: -5.0 },
                radius: 1.0,
                color: Color {
                    r: 0x00,
                    g: 0xFF,
                    b: 0x00
                }
            }),
            Box::new(Sphere {
                origin: Vec3 { x: 1.7, y: 1.0, z: -3.0 },
                radius: 0.75,
                color: Color {
                    r: 0xFF,
                    g: 0x00,
                    b: 0x00
                }
            }),
            Box::new(Sphere {
                origin: Vec3 { x: -1.2, y: 0.8, z: -4.0 },
                radius: 0.85,
                color: Color {
                    r: 0x00,
                    g: 0x00,
                    b: 0xFF
                }
            }),
            Box::new(Plane {
                origin: Vec3 { x: 0.0, y: -2.0, z: 0.0 },
                normal: Vec3 { x: 0.0, y: -1.0, z: 0.0 },
                color: Color {
                    r: 0x44,
                    g: 0x44,
                    b: 0x44
                }
            })
        ]
    };

    while window.is_open() && !window.is_key_down(Key::Escape) {
        viewport.render(&mut buffer);
        window.update_with_buffer(&buffer).unwrap();
    }
}
