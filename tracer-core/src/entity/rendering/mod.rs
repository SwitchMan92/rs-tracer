pub mod light;
pub mod material;

use glam::Vec4;

use crate::entity::{
    geometry::ray::{Ray, RayType},
    rendering::light::Light,
};

pub trait Renderable {
    fn render(
        &self,
        ray: &Ray,
        light: &Light,
        ray_type: &RayType,
        current_depth: &usize,
    ) -> Option<Vec4>;
}
