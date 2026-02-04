use glam::Vec3A;

use crate::entity::actor::{ActorTrait, DirectionalActorTrait};
use crate::entity::geometry::Geometry;
use crate::entity::geometry::ray::{Ray, RayType};
use crate::entity::geometry::sphere::Sphere;
use crate::entity::rendering::material::MaterialType;

/// Structure holding a given light's representation.
pub struct Light {
    geometry: Sphere,
    direction: Vec3A,
}

impl std::ops::Deref for Light {
    type Target = Sphere;
    fn deref(&self) -> &Self::Target {
        &self.geometry
    }
}

impl Light {
    pub fn new(position: &Vec3A, direction: &Vec3A, radius: f32, material: &MaterialType) -> Self {
        Self {
            geometry: Sphere::new(position, radius, material),
            direction: *direction,
        }
    }
}

impl ActorTrait for Light {
    fn get_position(&self) -> Vec3A {
        self.actor.get_position()
    }
}

impl DirectionalActorTrait for Light {
    fn get_direction(&self) -> Vec3A {
        self.direction
    }
}

impl Geometry for Light {
    /// Render the light object as a sphere, mostly for scene's debugging purpose.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3A)> {
        self.geometry.intersect(ray, ray_type)
    }

    fn get_surface_normal(&self, point: &Vec3A) -> Vec3A {
        self.geometry.get_surface_normal(point)
    }
}
