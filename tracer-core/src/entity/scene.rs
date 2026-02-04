use core::f32;
use glam::Vec4;

use crate::entity::{
    actor::{ActorTrait, DirectionalActorTrait},
    geometry::{
        Geometry, GeometryType,
        ray::{Ray, RayType},
    },
    rendering::{
        Renderable,
        light::Light,
        material::{MaterialBound, MaterialTrait},
    },
};

/// Container structure representing the scene's composition.
pub struct Scene {
    pub renderables: Vec<GeometryType>,
    pub ambient: Vec4,
}

impl Renderable for Scene {
    //// Iterate through the scene's renderable objects, and calculates the ray emitter ray's final color.
    fn render(
        &self,
        ray: &Ray,
        light: &Light,
        ray_type: &RayType,
        _current_depth: &usize,
    ) -> Option<Vec4> {
        let mut t_min = f32::NAN;
        let mut renderable_index: usize = 0;

        self.renderables.iter().enumerate().for_each(|x| {
            if let Some(hit) = x.1.intersect(ray, ray_type)
                && (t_min.is_nan() || hit.0 < t_min)
            {
                t_min = hit.0;
                renderable_index = x.0;
            }
        });

        match t_min.is_nan() {
            true => Some(self.ambient),
            false => {
                let intersection_point = ray.get_direction() * t_min + ray.get_position();
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
                let renderable = &self.renderables[renderable_index];

                match see_light {
                    0. => None,
                    x => Some(
                        renderable.get_material().calculate_illumination(
                            &self,
                            &renderable.get_surface_normal(&intersection_point),
                            &ray,
                            &light,
                            &light_ray,
                            &self.ambient,
                            &0,
                        ) * x,
                    ),
                }
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

    pub fn get_renderables(&self) -> &Vec<GeometryType> {
        &self.renderables
    }
}
