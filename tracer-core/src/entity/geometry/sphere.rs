use crate::entity::actor::{Actor, ActorTrait, DirectionalActorTrait};
use crate::entity::geometry::ActorWithGeometry;
use crate::entity::geometry::{Geometry, ray::Ray};
use crate::entity::rendering::light::Light;

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
    /// Check line-circle plain intersection and return the ray color post-interaction.
    fn intersect(&self, ray: &Ray, light: &Light) -> Option<(Vec3, Vec4)> {
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

                let t1 = (-b - x) / (2. * a);
                let t2 = (-b + x) / (2. * a);

                let t: f32;

                if (0. ..=1.).contains(&t1) {
                    t = t1;
                } else if (0. ..=1.).contains(&t2) {
                    t = t2;
                } else if t1 < 0. && t2 > 1. {
                    t = t2;
                } else {
                    return None;
                }

                let ray_vec = (ray.get_position() + ray.get_direction()).normalize();
                let light_vec = (light.get_position() + light.get_direction()).normalize();

                let product = ray_vec.dot(light_vec);
                return Some((
                    ray.get_position() + t * ray.get_direction(),
                    self.color * product,
                ));
            }
        }
    }
}

impl ActorWithGeometry for Sphere {}

// #####################################

#[cfg(test)]
mod tests {
    use glam::{Vec3, Vec4};

    use crate::entity::{
        actor::Actor,
        geometry::{Geometry, ray::Ray, sphere::Sphere},
        rendering::light::Light,
    };

    #[test]
    fn test_success_intersect() {
        const COLOR: Vec4 = Vec4::new(255., 255., 255., 0.);
        const VOID: Vec4 = Vec4::new(0., 0., 0., 0.);

        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(1., -1., 0.));

        let sphere = Sphere::new(&Vec3::new(2., 1., 0.), 1., COLOR);

        let light = Light::new(
            &Vec3::new(0., 0., 0.),
            &Vec3::new(0., -1., 0.),
            50.,
            Vec4::new(0., 0., 255., 1.),
        );

        assert!(sphere.intersect(&ray, &light) != None);

        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(-1., -1., 0.));

        let sphere = Sphere {
            actor: Actor::new(&Vec3::new(-2., 1., 0.)),
            color: COLOR,
            radius: 1.,
        };

        assert!(sphere.intersect(&ray, &light) != None);
    }

    // #####################################

    #[test]
    fn test_failure_intersect() {
        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(1., 1., 0.));

        const COLOR: Vec4 = Vec4::new(255., 255., 255., 0.);
        const VOID: Vec4 = Vec4::new(0., 0., 0., 0.);

        let sphere = Sphere::new(&Vec3::new(-2., 1., 0.), 1., COLOR);

        let light = Light::new(
            &Vec3::new(0., 0., 0.),
            &Vec3::new(0., -1., 0.),
            50.,
            Vec4::new(0., 0., 255., 1.),
        );

        assert_eq!(sphere.intersect(&ray, &light), None);

        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(-1., 1., 0.));

        let sphere = Sphere::new(&Vec3::new(-2., 1., 0.), 1., COLOR);

        assert_eq!(sphere.intersect(&ray, &light), None);
    }
}
