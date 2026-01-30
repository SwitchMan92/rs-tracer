pub mod plane;
pub mod ray;
pub mod sphere;

use glam::Vec3A;

use crate::entity::actor::ActorTrait;
use crate::entity::geometry::plane::Plane;
use crate::entity::geometry::ray::Ray;
use crate::entity::geometry::sphere::Sphere;
use crate::entity::rendering::light::Light;
use crate::entity::rendering::material::MaterialType;

pub enum RayType {
    Camera,
    Light,
}

/// Base traits used to allow handling of graphical interactions by a given scene.
pub trait Geometry {
    //// Check collision with a given ray from the ray emitter, return the ray's color post-interaction with the geometry object.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3A)>;
    fn get_surface_normal(&self, point: &Vec3A) -> Vec3A;
    fn get_material(&self) -> &MaterialType;
}

// #####################################

pub enum GeometryImpl {
    Plane(Plane),
    Sphere(Sphere),
    Light(Light),
}

impl ActorTrait for GeometryImpl {
    fn get_position(&self) -> Vec3A {
        match self {
            GeometryImpl::Plane(i) => i.get_position(),
            GeometryImpl::Sphere(i) => i.get_position(),
            GeometryImpl::Light(i) => i.get_position(),
        }
    }
}

impl Geometry for GeometryImpl {
    fn get_surface_normal(&self, point: &Vec3A) -> Vec3A {
        match self {
            GeometryImpl::Plane(i) => i.get_surface_normal(point),
            GeometryImpl::Sphere(i) => i.get_surface_normal(point),
            GeometryImpl::Light(i) => i.get_surface_normal(point),
        }
    }

    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3A)> {
        match self {
            GeometryImpl::Plane(i) => i.intersect(ray, ray_type),
            GeometryImpl::Sphere(i) => i.intersect(ray, ray_type),
            GeometryImpl::Light(i) => i.intersect(ray, ray_type),
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
