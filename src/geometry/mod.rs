use crate::rendering::{light::Light, ray::Ray};
use glam::Vec4;

pub mod actor;
pub mod scene;
pub mod sphere;

pub trait Geometry {
    fn collide(&self, ray: &Ray, light: &Light) -> Vec4;
}
