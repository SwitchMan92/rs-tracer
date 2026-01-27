pub mod plane;
pub mod ray;
pub mod sphere;

use glam::{Vec3AA, Vec4};

use crate::entity::actor::ActorTrait;
use crate::entity::geometry::plane::Plane;
use crate::entity::geometry::ray::Ray;
use crate::entity::geometry::sphere::Sphere;
use crate::entity::rendering::light::Light;

pub enum RayType {
    Camera,
    Light,
}

/// Base traits used to allow handling of graphical interactions by a given scene.
pub trait Geometry {
    //// Check collision with a given ray from the ray emitter, return the ray's color post-interaction with the geometry object.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3AA, Vec4)>;
    fn get_surface_normal(&self, point: &Vec3AA) -> Vec3AA;
}

// #####################################

pub enum GeometryImpl {
    Plane(Plane),
    Sphere(Sphere),
    Light(Light),
}

impl ActorTrait for GeometryImpl {
    fn get_position(&self) -> Vec3AA {
        match self {
            GeometryImpl::Plane(i) => i.get_position(),
            GeometryImpl::Sphere(i) => i.get_position(),
            GeometryImpl::Light(i) => i.get_position(),
        }
    }
}

impl Geometry for GeometryImpl {
    fn get_surface_normal(&self, point: &Vec3AA) -> Vec3AA {
        match self {
            GeometryImpl::Plane(i) => i.get_surface_normal(point),
            GeometryImpl::Sphere(i) => i.get_surface_normal(point),
            GeometryImpl::Light(i) => i.get_surface_normal(point),
        }
    }

    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3AA, Vec4)> {
        match self {
            GeometryImpl::Plane(i) => i.intersect(ray, ray_type),
            GeometryImpl::Sphere(i) => i.intersect(ray, ray_type),
            GeometryImpl::Light(i) => i.intersect(ray, ray_type),
        }
    }
}
