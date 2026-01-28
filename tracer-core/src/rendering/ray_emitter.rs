use glam::Vec3;
use range2d::Range2D;

use crate::entity::{
    actor::{ActorTrait, DirectionalActor, DirectionalActorTrait},
    geometry::ray::Ray,
};

/// Structure containing and managing an array of rays.
/// Each ray is then associated to a pixel of the render target in the renderer class, at the projection stage.
pub struct RayEmitter {
    dir_actor: DirectionalActor,
    resolution_x: u32,
    resolution_y: u32,
    pub rays: Vec<Ray>,
}

impl std::ops::Deref for RayEmitter {
    type Target = DirectionalActor;
    fn deref(&self) -> &Self::Target {
        &self.dir_actor
    }
}

impl RayEmitter {
    //// Declares and initializes the ray structures, given the screen's resolution.
    fn calculate_rays(&mut self) {
        let screen_bottom: Vec3 = Vec3::new(
            self.get_position().x - self.resolution_x as f32 / 2.,
            self.get_position().y - self.resolution_y as f32 / 2.,
            self.get_position().z,
        );
        let screen_top: Vec3 = Vec3::new(
            self.get_position().x + self.resolution_x as f32 / 2.,
            self.get_position().y + self.resolution_y as f32 / 2.,
            self.get_position().z,
        );
        let screen_unit: Vec3 = Vec3::new(
            (screen_top.x - screen_bottom.x) / self.resolution_x as f32,
            (screen_top.y - screen_bottom.y) / self.resolution_y as f32,
            0.,
        );

        self.rays = Range2D::new(0..self.resolution_y, 0..self.resolution_x)
            .map(|i| {
                Ray::new(
                    &(screen_bottom
                        + Vec3::new(screen_unit.x * i.1 as f32, screen_unit.y * i.0 as f32, 0.)),
                    &self.get_direction(),
                )
            })
            .rev()
            .collect();
    }
}

impl RayEmitter {
    pub fn new(position: Vec3, direction: Vec3, resolution_x: u32, resolution_y: u32) -> Self {
        let mut new_emitter = Self {
            dir_actor: DirectionalActor::new(&position, &direction),
            resolution_x,
            resolution_y,
            rays: Vec::new(),
        };
        new_emitter.calculate_rays();
        new_emitter
    }
}

// #####################################

#[cfg(test)]
mod tests {
    use crate::{entity::geometry::ray::Ray, rendering::ray_emitter::RayEmitter};
    use glam::Vec3;
    use range2d::Range2D;

    #[test]
    fn test_success_calculate_rays() {
        let position = Vec3::new(0., 0., 0.);
        let direction = Vec3::new(1., 0., 0.);

        let emitter = RayEmitter::new(position, direction, 2, 2);

        assert_eq!(emitter.rays.len(), 4);

        Range2D::new(0..2, 0..2).rev().enumerate().for_each(|it| {
            let origin = Vec3::new(-1. + it.1.1 as f32, -1. + it.1.0 as f32, 0.);
            assert_eq!(emitter.rays[it.0], Ray::new(&origin, &direction));
        });
    }
}
