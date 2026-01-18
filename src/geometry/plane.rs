use glam::{Vec3, Vec4};

use crate::geometry::Geometry;
use crate::geometry::actor::{Actor, ActorTrait, ActorWithGeometry};
use crate::rendering::light::Light;
use crate::rendering::ray::Ray;

pub struct Plane {
    pub actor: Actor,
    pub color: Vec4,
    pub scalar: Vec3,
    pub normal: Vec3,
}

impl std::ops::Deref for Plane {
    type Target = Actor;
    fn deref(&self) -> &Self::Target {
        &self.actor
    }
}

impl ActorTrait for Plane {
    fn get_position(&self) -> Vec3 {
        self.position
    }
}

impl Geometry for Plane {
    fn collide(&self, ray: &Ray, light: &Light) -> Vec4 {
        if self.normal.dot(ray.direction) == 0. {
            return Vec4::new(0., 0., 0., 0.);
        }

        let ray_vec = (ray.origin + ray.direction).normalize();
        let light_vec = (light.position + light.direction).normalize();
        let product = ray_vec.dot(light_vec);

        self.color * product

        // let mut plane_vectors = self.normal.any_orthonormal_pair();
        // plane_vectors.0 = self.position + plane_vectors.0 * self.scalar;
        // plane_vectors.1 = self.position + plane_vectors.1 * self.scalar;
        // let t = self.normal.dot(self.position) - self.normal.dot(ray.origin) / self.normal.dot(ray.direction);
        // return linePoint.plus(lineDirection.normalize().scale(t));
    }
}

impl ActorWithGeometry for Plane {}
