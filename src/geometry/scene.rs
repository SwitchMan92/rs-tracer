use glam::Vec4;

use crate::geometry::Geometry;
use crate::rendering::light::Light;
use crate::rendering::ray::Ray;

pub struct Scene {
    pub renderables: Vec<Box<dyn Geometry>>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            renderables: Vec::new(),
        }
    }
}

impl Geometry for Scene {
    fn collide(&self, ray: &Ray, light: &Light) -> Vec4 {
        self.renderables
            .iter()
            .fold(Vec4::new(0., 0., 0., 0.), |acc, x| {
                (acc + x.collide(ray, light))
                    .clamp(Vec4::new(0., 0., 0., 0.), Vec4::new(255., 255., 255., 1.))
            })
    }
}
