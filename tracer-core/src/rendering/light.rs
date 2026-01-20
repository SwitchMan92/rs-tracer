use glam::{Vec3, Vec4};

use crate::{
    geometry::{
        Geometry,
        actor::{Actor, ActorTrait, ActorWithGeometry},
    },
    rendering::ray::Ray,
};

/// Structure holding a given light's representation.
pub struct Light {
    pub actor: Actor,
    pub direction: Vec3,
    pub radius: f32,
    pub color: Vec4,
}

impl std::ops::Deref for Light {
    type Target = Actor;
    fn deref(&self) -> &Self::Target {
        &self.actor
    }
}

impl ActorTrait for Light {
    fn get_position(&self) -> Vec3 {
        self.position
    }
}

impl Geometry for Light {
    /// Render the light object as a sphere, mostly for scene's debugging purpose.
    fn intersect(&self, ray: &Ray, light: &Light) -> Vec4 {
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

                if (0. ..=1.).contains(&t1) || (0. ..=1.).contains(&t2) || (t1 < 0. && t2 > 1.) {
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

impl ActorWithGeometry for Light {}
