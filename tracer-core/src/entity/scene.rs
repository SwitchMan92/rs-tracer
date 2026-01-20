use glam::Vec4;

use crate::entity::{
    geometry::{ActorWithGeometry, Geometry, ray::Ray},
    rendering::light::Light,
};

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
    fn intersect(&self, ray: &Ray, light: &Light) -> Option<Vec4> {
        let mut result_color = Vec4::new(0., 0., 0., 0.);

        self.renderables.iter().for_each(|x| {
            result_color += match x.intersect(ray, light) {
                None => Vec4::ZERO,
                Some(x) => x,
            }
        });

        Some(result_color)
    }
}
