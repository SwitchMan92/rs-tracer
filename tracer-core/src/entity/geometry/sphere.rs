use crate::entity::actor::{Actor, ActorTrait, DirectionalActorTrait};
use crate::entity::geometry::RayType;
use crate::entity::geometry::{Geometry, ray::Ray};

use glam::{Vec3, Vec4};

/// Structure used to represent a spherical renderable.
pub struct Sphere {
    pub actor: Actor,
    pub radius: f32,
    pub color: Vec4,
}

impl std::ops::Deref for Sphere {
    type Target = Actor;
    fn deref(&self) -> &Self::Target {
        &self.actor
    }
}

impl Sphere {
    pub const fn new(position: &Vec3, radius: f32, color: Vec4) -> Self {
        Self {
            actor: Actor::new(position),
            radius,
            color,
        }
    }
}

impl ActorTrait for Sphere {
    fn get_position(&self) -> Vec3 {
        self.actor.get_position()
    }
}

impl Geometry for Sphere {
    //// Return the sphere's normal vector.
    fn get_surface_normal(&self, point: &Vec3) -> Vec3 {
        (point - self.get_position()) / self.radius
    }

    /// Check line-circle plain intersection and return the ray color post-interaction.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3, Vec4)> {
        let d = ray.get_direction();
        let f = ray.get_position() - self.position;

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

                t.map(|t| (t, ray.get_position() + t * ray.get_direction(), self.color))
            }
        }
    }
}

// #####################################

#[cfg(test)]
mod tests {
    use glam::{Vec3, Vec4};

    use crate::entity::{
        actor::Actor,
        geometry::{Geometry, RayType, ray::Ray, sphere::Sphere},
    };

    #[test]
    fn test_success_intersect() {
        const COLOR: Vec4 = Vec4::new(255., 255., 255., 0.);

        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(1., -1., 0.));

        let sphere = Sphere::new(&Vec3::new(2., 1., 0.), 1., COLOR);

        assert!(sphere.intersect(&ray, &RayType::Camera) != None);

        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(-1., -1., 0.));

        let sphere = Sphere {
            actor: Actor::new(&Vec3::new(-2., 1., 0.)),
            color: COLOR,
            radius: 1.,
        };

        assert!(sphere.intersect(&ray, &RayType::Camera) != None);
    }

    // #####################################

    #[test]
    fn test_failure_intersect() {
        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(1., 1., 0.));

        const COLOR: Vec4 = Vec4::new(255., 255., 255., 0.);

        let sphere = Sphere::new(&Vec3::new(-2., 1., 0.), 1., COLOR);

        assert_eq!(sphere.intersect(&ray, &RayType::Camera), None);

        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(-1., 1., 0.));

        let sphere = Sphere::new(&Vec3::new(-2., 1., 0.), 1., COLOR);

        assert_eq!(sphere.intersect(&ray, &RayType::Camera), None);
    }
}
