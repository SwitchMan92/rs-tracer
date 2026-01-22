use core::f32;
use glam::Vec4;

use crate::entity::{
    actor::{ActorTrait, DirectionalActorTrait},
    geometry::{Geometry, GeometryImpl, RayType, ray::Ray},
    rendering::light::Light,
};

/// Container structure representing the scene's composition.
pub struct Scene {
    pub renderables: Vec<GeometryImpl>,
    pub ambient: Vec4,
}

pub trait Renderable {
    fn render(&self, ray: &Ray, light: &Light, ray_type: &RayType) -> Option<Vec4>;
}

impl Renderable for Scene {
    //// Iterate through the scene's renderable objects, and calculates the ray emitter ray's final color.
    fn render(&self, ray: &Ray, light: &Light, ray_type: &RayType) -> Option<Vec4> {
        let mut result_color = Vec4::new(0., 0., 0., 0.);
        let mut t_min = f32::NAN;
        let mut renderable_index: usize = 0;

        self.renderables.iter().enumerate().for_each(|x| {
            if let Some(hit) = x.1.intersect(ray, ray_type)
                && (t_min.is_nan() || hit.0 < t_min)
            {
                t_min = hit.0;
                renderable_index = x.0;
                result_color = hit.2;
            }
        });

        match t_min.is_nan() {
            true => Some(self.ambient),
            false => {
                let intersection_point = ray.get_direction() * t_min + ray.get_position();
                let normal =
                    self.renderables[renderable_index].get_surface_normal(&intersection_point);
                let light_ray = Ray::new(
                    &intersection_point,
                    &(light.get_position() - intersection_point),
                );

                let see_light = f32::min(
                    self.renderables
                        .iter()
                        .enumerate()
                        .filter(|x| x.0 != renderable_index)
                        .filter(|x| match x.1.intersect(&light_ray, &RayType::Light) {
                            Some(hit) => hit.0 >= t_min,
                            None => true,
                        })
                        .count() as f32,
                    1.,
                );
                let dot = light_ray.get_direction().dot(normal);
                let diffuse = result_color * f32::max(0., dot);
                Some(diffuse * self.ambient * see_light)
            }
        }
    }
}

impl Scene {
    pub const fn new(ambient: &Vec4) -> Self {
        Self {
            renderables: Vec::new(),
            ambient: *ambient,
        }
    }
}
