use glam::{Vec3, Vec4};

use crate::geometry::Geometry;
use crate::geometry::actor::{Actor, ActorTrait, ActorWithGeometry};
use crate::rendering::light::Light;
use crate::rendering::ray::Ray;

//// Structure representing a Planar surface.
pub struct Plane {
    pub actor: Actor,
    pub color: Vec4,
    pub normal: Vec3,
}

impl std::ops::Deref for Plane {
    type Target = Actor;
    fn deref(&self) -> &Self::Target {
        &self.actor
    }
}

impl Geometry for Plane {
    //// check if the ray intersects with the current plane structure and return the ray's color post-interaction.
    fn collide(&self, ray: &Ray, light: &Light) -> Vec4 {
        if self.normal.dot(ray.direction) == 0. {
            return Vec4::new(0., 0., 0., 0.);
        }

        let ray_vec = ray.origin + ray.direction;
        let light_vec = light.get_location() + light.direction;
        let product = ray_vec.dot(light_vec);

        self.color * product
    }
}

impl ActorTrait for Plane {
    fn get_location(&self) -> Vec3 {
        self.actor.get_location()
    }
    fn get_rotation_angles(&self) -> Vec3 {
        self.actor.get_rotation_angles()
    }
    fn get_scaling_factors(&self) -> Vec3 {
        self.actor.get_scaling_factors()
    }
    fn set_location(&mut self, position: &Vec3) {
        self.actor.set_location(position);
    }
    fn set_rotation(&mut self, euler_angles: &Vec3) {
        self.actor.set_rotation(euler_angles);
    }
    fn set_scaling(&mut self, scale_factors: &Vec3) {
        self.actor.set_scaling(scale_factors);
    }
}

impl ActorWithGeometry for Plane {}
