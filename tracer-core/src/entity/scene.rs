use glam::{Vec3, Vec4};

use crate::entity::{
    actor::ActorTrait,
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
    fn intersect(&self, ray: &Ray, light: &Light) -> Option<(Vec3, Vec4)> {
        let mut result_color = Vec4::new(0., 0., 0., 0.);
        let mut result_depth = Vec3::NAN;

        self.renderables.iter().for_each(|x| {
            match x.intersect(ray, light) {
                Some(hit) => {
                    if result_depth.is_nan()
                        || hit.0.distance(ray.get_position())
                            > result_depth.distance(ray.get_position())
                    {
                        result_depth = hit.0;
                        result_color = hit.1;
                    }
                }
                None => {}
            };
        });

        match result_depth.is_nan() {
            true => None,
            false => Some((result_depth, result_color)),
        }
    }
}
