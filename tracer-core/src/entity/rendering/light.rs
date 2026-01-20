use glam::{Vec3, Vec4};

use crate::entity::actor::{ActorTrait, DirectionalActorTrait};
use crate::entity::geometry::ray::Ray;
use crate::entity::geometry::sphere::Sphere;
use crate::entity::geometry::{ActorWithGeometry, Geometry};

/// Structure holding a given light's representation.
pub struct Light {
    geometry: Sphere,
    direction: Vec3,
}

impl std::ops::Deref for Light {
    type Target = Sphere;
    fn deref(&self) -> &Self::Target {
        &self.geometry
    }
}

impl Light {
    pub const fn new(position: &Vec3, direction: &Vec3, radius: f32, color: Vec4) -> Self {
        Self {
            geometry: Sphere::new(position, radius, color),
            direction: *direction,
        }
    }
}

impl ActorTrait for Light {
    fn get_position(&self) -> Vec3 {
        self.actor.get_position()
    }
}

impl DirectionalActorTrait for Light {
    fn get_direction(&self) -> Vec3 {
        self.direction
    }
}

impl Geometry for Light {
    /// Render the light object as a sphere, mostly for scene's debugging purpose.
    fn intersect(&self, ray: &Ray, light: &Light) -> Option<Vec4> {
        self.geometry.intersect(ray, light)
    }
}

impl ActorWithGeometry for Light {}
