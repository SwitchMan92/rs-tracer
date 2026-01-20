pub mod plane;
pub mod ray;
pub mod sphere;

use glam::{Vec3, Vec4};

use crate::entity::actor::ActorTrait;
use crate::entity::geometry::ray::Ray;
use crate::entity::rendering::light::Light;

/// Base traits used to allow handling of graphical interactions by a given scene.
pub trait Geometry {
    //// Check collision with a given ray from the ray emitter, return the ray's color post-interaction with the geometry object.
    fn intersect(&self, ray: &Ray, light: &Light) -> Option<(Vec3, Vec4)>;
}

// #####################################

/// Public trait mixin used to allow operations on geomerty and position simultaneously.
pub trait ActorWithGeometry: Geometry + ActorTrait {}
