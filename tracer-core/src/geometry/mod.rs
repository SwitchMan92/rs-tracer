use crate::rendering::{light::Light, ray::Ray};
use glam::Vec4;

pub mod actor;
pub mod plane;
pub mod scene;
pub mod sphere;

//// Base traits used to allow handling of graphical interactions by a given scene.
pub trait Geometry {
    //// Check collision with a given ray from the ray emitter, return the ray's color post-interaction with the geometry object.
    fn collide(&self, ray: &Ray, light: &Light) -> Vec4;
}
