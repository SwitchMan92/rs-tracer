pub mod plane;
pub mod ray;
pub mod sphere;

use glam::{Vec3, Vec4};

use crate::entity::actor::ActorTrait;
use crate::entity::geometry::ray::Ray;

pub enum RayType {
    Camera,
    Light,
}

/// Base traits used to allow handling of graphical interactions by a given scene.
pub trait Geometry {
    //// Check collision with a given ray from the ray emitter, return the ray's color post-interaction with the geometry object.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3, Vec4)>;
    fn get_surface_normal(&self, point: &Vec3) -> Vec3;
}

// #####################################

/// Public trait mixin used to allow operations on geomerty and position simultaneously.
pub trait ActorWithGeometry: Geometry + ActorTrait {}
