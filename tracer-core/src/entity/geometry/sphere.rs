use crate::entity::actor::{Actor, ActorTrait, DirectionalActorTrait};
use crate::entity::geometry::RayType;
use crate::entity::geometry::{Geometry, ray::Ray};
use crate::entity::rendering::material::{MaterialBound, MaterialType};

use glam::Vec3A;

/// Structure used to represent a spherical renderable.
pub struct Sphere {
    pub actor: Actor,
    pub radius: f32,
    material: MaterialType,
}

impl std::ops::Deref for Sphere {
    type Target = Actor;
    fn deref(&self) -> &Self::Target {
        &self.actor
    }
}

impl MaterialBound for Sphere {
    fn get_material(&self) -> &MaterialType {
        &self.material
    }
}

impl Sphere {
    pub fn new(position: &Vec3A, radius: f32, material: &MaterialType) -> Self {
        Self {
            actor: Actor::new(position),
            radius: radius,
            material: material.to_owned(),
        }
    }
}

impl ActorTrait for Sphere {
    fn get_position(&self) -> Vec3A {
        self.actor.get_position()
    }
}

impl Geometry for Sphere {
    fn get_surface_normal(&self, point: &Vec3A) -> Vec3A {
        (point - self.get_position()) / self.radius
    }

    fn get_material(&self) -> &MaterialType {
        &self.material
    }

    /// Check line-circle plain intersection and return the ray color post-interaction.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3A)> {
        let d = ray.get_direction();
        let f = ray.get_position() - self.actor.get_position();

        let a = d.dot(d);
        let b = 2. * f.dot(d);
        let c = f.dot(f) - self.radius * self.radius;

        let discr = b * b - 4. * a * c;

        match discr {
            x if x < 0. => None,
            mut x => {
                x = x.sqrt();

                let t0 = (-b - x) / (2. * a);
                let t1 = (-b + x) / (2. * a);

                let t = match ray_type {
                    RayType::Camera => {
                        if t0 > 1. {
                            Some(t0)
                        } else if t1 > 1. {
                            Some(t1)
                        } else {
                            None
                        }
                    }
                    RayType::Light => {
                        if (0.0001..=1.).contains(&t0) {
                            Some(t0)
                        } else if (0.0001..=1.).contains(&t1) {
                            Some(t1)
                        } else {
                            None
                        }
                    }
                };

                t.map(|t| (t, ray.get_position() + t * ray.get_direction()))
            }
        }
    }
}

// #####################################

#[cfg(test)]
mod tests {
    use glam::{Vec3A, Vec4};

    use crate::entity::{
        geometry::{Geometry, RayType, ray::Ray, sphere::Sphere},
        rendering::material::{ColorMaterial, MaterialType},
    };

    #[test]
    fn test_success_intersect() {
        let ray = Ray::new(&Vec3A::new(0., 2., 0.), &Vec3A::new(1., -1., 0.));
        let sphere = Sphere::new(
            &Vec3A::new(2., 1., 0.),
            1.,
            &MaterialType::Color(ColorMaterial::new(Vec4::ONE)),
        );
        assert!(sphere.intersect(&ray, &RayType::Camera) != None);

        let ray = Ray::new(&Vec3A::new(0., 2., 0.), &Vec3A::new(-1., -1., 0.));
        let sphere = Sphere::new(
            &Vec3A::new(-2., 1., 0.),
            1.,
            &MaterialType::Color(ColorMaterial::new(Vec4::ONE)),
        );
        assert!(sphere.intersect(&ray, &RayType::Camera) != None);
    }

    // #####################################

    #[test]
    fn test_failure_intersect() {
        let ray = Ray::new(&Vec3A::new(0., 2., 0.), &Vec3A::new(1., 1., 0.));
        let sphere = Sphere::new(
            &Vec3A::new(-2., 1., 0.),
            1.,
            &MaterialType::Color(ColorMaterial::new(Vec4::ONE)),
        );
        assert_eq!(sphere.intersect(&ray, &RayType::Camera), None);

        let ray = Ray::new(&Vec3A::new(0., 2., 0.), &Vec3A::new(-1., 1., 0.));
        let sphere = Sphere::new(
            &Vec3A::new(-2., 1., 0.),
            1.,
            &MaterialType::Color(ColorMaterial::new(Vec4::ONE)),
        );
        assert_eq!(sphere.intersect(&ray, &RayType::Camera), None);
    }
}
