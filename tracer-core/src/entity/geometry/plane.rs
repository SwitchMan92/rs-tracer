use glam::Vec3A;

use crate::entity::{
    actor::{ActorTrait, DirectionalActor, DirectionalActorTrait},
    geometry::{Geometry, RayType, ray::Ray},
    rendering::material::{MaterialBound, MaterialType},
};

/// Structure representing a Planar surface.
pub struct Plane {
    dir_actor: DirectionalActor,
    material: MaterialType,
}

impl Plane {
    pub fn new(position: &Vec3A, direction: &Vec3A, material: &MaterialType) -> Self {
        Self {
            dir_actor: DirectionalActor::new(position, direction),
            material: material.to_owned(),
        }
    }
}

impl MaterialBound for Plane {
    fn get_material(&self) -> &MaterialType {
        &self.material
    }
}

impl std::ops::Deref for Plane {
    type Target = DirectionalActor;
    fn deref(&self) -> &Self::Target {
        &self.dir_actor
    }
}

impl ActorTrait for Plane {
    fn get_position(&self) -> Vec3A {
        self.dir_actor.get_position()
    }
}

impl Geometry for Plane {
    fn get_surface_normal(&self, _point: &Vec3A) -> Vec3A {
        self.dir_actor.get_direction()
    }

    //// check if the ray intersects with the current plane structure and return the ray's color post-interaction.
    fn intersect(&self, ray: &Ray, ray_type: &RayType) -> Option<(f32, Vec3A)> {
        let n_dot_l = ray.get_direction().dot(self.get_direction());
        match n_dot_l {
            x if x < 0.001 => None,
            _ => {
                let t =
                    (self.get_position() - ray.get_position()).dot(self.get_direction()) / n_dot_l;

                match ray_type {
                    RayType::Camera => match t {
                        x if x < 1. => None,
                        _ => Some((t, ray.get_position() + t * ray.get_direction())),
                    },
                    RayType::Light => match t {
                        x if (0.0001..=1.).contains(&x) => {
                            Some((t, ray.get_position() + t * ray.get_direction()))
                        }
                        _ => None,
                    },
                }
            }
        }
    }
}

// #####################################

#[cfg(test)]
mod tests {
    use glam::{Vec3A, Vec4};

    use crate::entity::{
        geometry::{Geometry, RayType, plane::Plane, ray::Ray},
        rendering::material::{ColorMaterial, MaterialType},
    };

    #[test]
    fn test_success_intersect() {
        let ray = Ray::new(&Vec3A::new(0., 2., 0.), &Vec3A::new(1., -1., 0.));
        let plane = Plane::new(
            &Vec3A::new(2., 1., 0.),
            &Vec3A::new(2., 1., 0.),
            &MaterialType::Color(ColorMaterial::new(Vec4::ONE)),
        );
        assert!(plane.intersect(&ray, &RayType::Camera) != None);
    }
}
