use crate::vector::{Vec2, Vec3};
use crate::scene::{Solid, Viewport};

pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3
}

impl Ray {
    pub fn new(px: u32, py: u32, viewport: &Viewport) -> Ray {
        assert!(viewport.width > viewport.height); // for aspect ratio calc width must be greater than height
        let fov_coeff = (viewport.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (viewport.width as f64) / (viewport.height as f64);
        let dir_x = ((((px as f64 + 0.5) / viewport.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_coeff; // ((center of pixel) / screen width) * 2.0 - 1.0 = normalized screen coords [-1.0, 1.0]
        let dir_y = (1.0 - ((py as f64 + 0.5) / viewport.height as f64) * 2.0) * fov_coeff;
        
        Ray {
            origin: Vec3::zero(),
            dir: Vec3 {
                x: dir_x,
                y: dir_y,
                z: -1.0
            }.normalize()
        }
    }
}