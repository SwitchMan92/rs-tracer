use glam::Vec4;

use crate::geometry::Geometry;
use crate::geometry::actor::ActorWithGeometry;
use crate::rendering::light::Light;
use crate::rendering::ray::Ray;

/// Container structure representing the scene's composition.
pub struct Scene<'a> {
    pub renderables: Vec<&'a dyn ActorWithGeometry>,
}

impl<'a> Scene<'a> {
    pub fn new() -> Self {
        Self {
            renderables: Vec::new(),
        }
    }
}

impl<'a> Geometry for Scene<'a> {
    //// Iterate through the scene's renderable objects, and calculates the ray emitter ray's final color.
    fn collide(&self, ray: &Ray, light: &Light) -> Vec4 {
        let mut result_color = Vec4::new(0., 0., 0., 0.);

        self.renderables.iter().for_each(|x| {
            let current_color = x.collide(ray, light);
            if current_color != Vec4::new(0., 0., 0., 0.) {
                result_color = current_color;
            }
        });

        result_color
    }
}
