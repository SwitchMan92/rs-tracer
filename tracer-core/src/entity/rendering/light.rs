use glam::{Vec3, Vec4};

use crate::entity::actor::{ActorTrait, DirectionalActor, DirectionalActorTrait};
use crate::entity::geometry::ray::Ray;
use crate::entity::geometry::{ActorWithGeometry, Geometry};

/// Structure holding a given light's representation.
pub struct Light {
    dir_actor: DirectionalActor,
    radius: f32,
    color: Vec4,
}

impl std::ops::Deref for Light {
    type Target = DirectionalActor;
    fn deref(&self) -> &Self::Target {
        &self.dir_actor
    }
}

impl Light {
    pub const fn new(position: &Vec3, direction: &Vec3, radius: f32, color: Vec4) -> Self {
        Self {
            dir_actor: DirectionalActor::new(position, direction),
            radius,
            color,
        }
    }
}

impl ActorTrait for Light {
    fn get_position(&self) -> Vec3 {
        self.dir_actor.get_position()
    }
}

impl Geometry for Light {
    /// Render the light object as a sphere, mostly for scene's debugging purpose.
    fn intersect(&self, ray: &Ray, light: &Light) -> Vec4 {
        const VOID: Vec4 = Vec4::new(0., 0., 0., 0.);

        let d = ray.get_direction();
        let f = ray.get_position() - self.get_position();

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

                if (0. ..=1.).contains(&t1) || (0. ..=1.).contains(&t2) || (t1 < 0. && t2 > 1.) {
                    let ray_vec = (ray.get_position() + ray.get_direction()).normalize();
                    let light_vec = (light.get_position() + light.get_direction()).normalize();

                    let product = ray_vec.dot(light_vec);
                    return self.color * product;
                }

                VOID
            }
        }
    }
}

impl ActorWithGeometry for Light {}
