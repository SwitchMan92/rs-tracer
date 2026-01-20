use glam::{Vec3, Vec4};

use crate::entity::{
    actor::{ActorTrait, DirectionalActor, DirectionalActorTrait},
    geometry::{ActorWithGeometry, Geometry, ray::Ray},
    rendering::light::Light,
};

/// Structure representing a Planar surface.
pub struct Plane {
    dir_actor: DirectionalActor,
    color: Vec4,
}

impl std::ops::Deref for Plane {
    type Target = DirectionalActor;
    fn deref(&self) -> &Self::Target {
        &self.dir_actor
    }
}

impl Plane {
    pub const fn new(position: &Vec3, direction: &Vec3, color: &Vec4) -> Self {
        Self {
            dir_actor: DirectionalActor::new(position, direction),
            color: *color,
        }
    }
}

impl ActorTrait for Plane {
    fn get_position(&self) -> Vec3 {
        self.dir_actor.get_position()
    }
}

impl Geometry for Plane {
    //// check if the ray intersects with the current plane structure and return the ray's color post-interaction.
    fn intersect(&self, ray: &Ray, _light: &Light) -> Option<(Vec3, Vec4)> {
        let n_dot_l = ray.get_direction().dot(self.get_direction());
        match n_dot_l {
            x if x < 0.001 => None,
            _ => {
                let t =
                    (self.get_position() - ray.get_position()).dot(self.get_direction()) / n_dot_l;

                match t {
                    x if x < 1. => None,
                    _ => Some((ray.get_position() + t * ray.get_direction(), self.color * t)),
                }
            }
        }
    }
}

impl ActorWithGeometry for Plane {}

// #####################################

#[cfg(test)]
mod tests {
    use glam::{Vec3, Vec4};

    use crate::entity::{
        actor::DirectionalActor,
        geometry::{Geometry, plane::Plane, ray::Ray},
        rendering::light::Light,
    };

    #[test]
    fn test_success_intersect() {
        const COLOR: Vec4 = Vec4::new(255., 255., 255., 0.);

        let ray = Ray::new(&Vec3::new(0., 2., 0.), &Vec3::new(1., -1., 0.));

        let plane = Plane {
            dir_actor: DirectionalActor::new(&Vec3::new(2., 1., 0.), &Vec3::new(2., 1., 0.)),
            color: COLOR,
        };

        let light = Light::new(
            &Vec3::new(0., 100., 0.),
            &Vec3::new(0., -1., 0.),
            50.,
            Vec4::new(0., 0., 255., 1.),
        );

        assert!(plane.intersect(&ray, &light) != None);
    }
}
