pub mod plane;
pub mod ray;
pub mod sphere;

use glam::Vec3A;

use crate::entity::actor::ActorTrait;
use crate::entity::geometry::plane::Plane;
use crate::entity::geometry::ray::{Ray, RayType};
use crate::entity::geometry::sphere::Sphere;
use crate::entity::rendering::light::Light;
use crate::entity::rendering::material::MaterialBound;

/// Base traits used to allow handling of graphical interactions by a given scene.
pub trait Geometry {
    //// Check collision with a given ray from the ray emitter, return the ray's color post-interaction with the geometry object.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3A)>;
    fn get_surface_normal(&self, _point: &Vec3A) -> Vec3A;
}

// #####################################

pub enum GeometryType {
    Plane(Plane),
    Sphere(Sphere),
    Light(Light),
}

impl ActorTrait for GeometryType {
    fn get_position(&self) -> Vec3A {
        match self {
            GeometryType::Plane(i) => i.get_position(),
            GeometryType::Sphere(i) => i.get_position(),
            GeometryType::Light(i) => i.get_position(),
        }
    }
}

impl Geometry for GeometryType {
    fn get_surface_normal(&self, point: &Vec3A) -> Vec3A {
        match self {
            GeometryType::Plane(i) => i.get_surface_normal(point),
            GeometryType::Sphere(i) => i.get_surface_normal(point),
            GeometryType::Light(i) => i.get_surface_normal(point),
        }
    }

    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3A)> {
        match self {
            GeometryType::Plane(i) => i.intersect(ray, ray_type),
            GeometryType::Sphere(i) => i.intersect(ray, ray_type),
            GeometryType::Light(i) => i.intersect(ray, ray_type),
        }
    }
}

impl MaterialBound for GeometryType {
    fn get_material(&self) -> &super::rendering::material::MaterialType {
        match self {
            GeometryType::Plane(i) => i.get_material(),
            GeometryType::Sphere(i) => i.get_material(),
            GeometryType::Light(i) => i.get_material(),
        }
    }

    fn get_material(&self) -> &MaterialType {
        match self {
            GeometryImpl::Plane(i) => i.get_material(),
            GeometryImpl::Sphere(i) => i.get_material(),
            GeometryImpl::Light(i) => i.get_material(),
        }
    }
}
