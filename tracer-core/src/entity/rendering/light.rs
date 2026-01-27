use glam::{Vec3AA, Vec4};

use crate::entity::actor::{ActorTrait, DirectionalActorTrait};
use crate::entity::geometry::ray::Ray;
use crate::entity::geometry::sphere::Sphere;
use crate::entity::geometry::{Geometry, RayType};

/// Structure holding a given light's representation.
pub struct Light {
    geometry: Sphere,
    direction: Vec3AA,
}

impl std::ops::Deref for Light {
    type Target = Sphere;
    fn deref(&self) -> &Self::Target {
        &self.geometry
    }
}

impl Light {
    pub const fn new(position: &Vec3AA, direction: &Vec3AA, radius: f32, color: Vec4) -> Self {
        Self {
            geometry: Sphere::new(position, radius, color),
            direction: *direction,
        }
    }
}

impl ActorTrait for Light {
    fn get_position(&self) -> Vec3AA {
        self.actor.get_position()
    }
}

impl DirectionalActorTrait for Light {
    fn get_direction(&self) -> Vec3AA {
        self.direction
    }
}

impl Geometry for Light {
    fn get_surface_normal(&self, point: &Vec3AA) -> Vec3AA {
        self.geometry.get_surface_normal(point)
    }

    /// Render the light object as a sphere, mostly for scene's debugging purpose.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3AA, Vec4)> {
        self.geometry.intersect(ray, ray_type)
    }
}
