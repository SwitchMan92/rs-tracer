use crate::geometry::Geometry;
use crate::geometry::actor::{Actor, ActorTrait, ActorWithGeometry};
use crate::rendering::light::Light;
use crate::rendering::ray::Ray;

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

impl ActorTrait for Sphere {
    fn get_position(&self) -> Vec3 {
        self.position
    }
}

impl Geometry for Sphere {
    /// Check line-circle plain intersection and return the ray color post-interaction.
    fn collide(&self, ray: &Ray, light: &Light) -> Vec4 {
        const VOID: Vec4 = Vec4::new(0., 0., 0., 0.);

        let d = ray.direction;
        let f = ray.origin - self.position;

        let a = d.dot(d);
        let b = 2. * f.dot(d);
        let c = f.dot(f) - self.radius * self.radius;

        let discr = b * b - 4. * a * c;

        match discr {
            x if x < 0. => VOID,
            mut x => {
                x = x.sqrt();

                let t1 = (-b - x) / (2. * a);
                let t2 = (-b + x) / (2. * a);

                if (t1 >= 0. && t1 <= 1.) || (t2 >= 0. && t2 <= 1.) || (t1 < 0. && t2 > 1.) {
                    let ray_vec = (ray.origin + ray.direction).normalize();
                    let light_vec = (light.position + light.direction).normalize();

                    let product = ray_vec.dot(light_vec);
                    return self.color * product;
                }

                VOID
            }
        }
    }
}

impl ActorWithGeometry for Sphere {}

#[cfg(test)]
mod tests {
    use glam::{Vec3, Vec4};

    use crate::{
        geometry::{Geometry, actor::Actor, sphere::Sphere},
        rendering::{light::Light, ray::Ray},
    };

    #[test]
    fn test_success_collide() {
        const COLOR: Vec4 = Vec4::new(255., 255., 255., 0.);
        const VOID: Vec4 = Vec4::new(0., 0., 0., 0.);

        let ray = Ray {
            origin: Vec3::new(0., 2., 0.),
            direction: Vec3::new(1., -1., 0.),
        };

        let sphere = Sphere {
            actor: Actor::new(Vec3::new(2., 1., 0.)),
            color: COLOR,
            radius: 1.,
        };

        let light = Light {
            actor: Actor::new(Vec3::new(0., 0., 0.)),
            direction: Vec3::new(0., -1., 0.),
            radius: 50.,
            color: Vec4::new(0., 0., 255., 1.),
        };

        assert!(sphere.collide(&ray, &light) != VOID);

        let ray = Ray {
            origin: Vec3::new(0., 2., 0.),
            direction: Vec3::new(-1., -1., 0.),
        };

        let sphere = Sphere {
            actor: Actor::new(Vec3::new(-2., 1., 0.)),
            color: COLOR,
            radius: 1.,
        };

        assert!(sphere.collide(&ray, &light) != VOID);
    }

    #[test]
    fn test_failure_collide() {
        let ray = Ray {
            origin: Vec3::new(0., 2., 0.),
            direction: Vec3::new(1., 1., 0.),
        };

        const COLOR: Vec4 = Vec4::new(255., 255., 255., 0.);
        const VOID: Vec4 = Vec4::new(0., 0., 0., 0.);

        let sphere = Sphere {
            actor: Actor::new(Vec3::new(-2., 1., 0.)),
            color: COLOR,
            radius: 1.,
        };

        let light = Light {
            actor: Actor::new(Vec3::new(0., 0., 0.)),
            direction: Vec3::new(0., -1., 0.),
            radius: 50.,
            color: Vec4::new(0., 0., 255., 1.),
        };

        assert_eq!(sphere.collide(&ray, &light), VOID);

        let ray = Ray {
            origin: Vec3::new(0., 2., 0.),
            direction: Vec3::new(-1., 1., 0.),
        };

        let sphere = Sphere {
            actor: Actor::new(Vec3::new(-2., 1., 0.)),
            color: COLOR,
            radius: 1.,
        };

        assert_eq!(sphere.collide(&ray, &light), VOID);
    }
}
