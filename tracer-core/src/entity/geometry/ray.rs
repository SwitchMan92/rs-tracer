use glam::Vec3AA;
use std::fmt;

use crate::entity::actor::{ActorTrait, DirectionalActor, DirectionalActorTrait};

/// Structure holding a ray's geometric data.
#[derive(Debug)]
pub struct Ray {
    dir_actor: DirectionalActor,
}

impl std::ops::Deref for Ray {
    type Target = DirectionalActor;
    fn deref(&self) -> &Self::Target {
        &self.dir_actor
    }
}

impl Ray {
    pub const fn new(position: &Vec3AA, direction: &Vec3AA) -> Self {
        Self {
            dir_actor: DirectionalActor::new(position, direction),
        }
    }
}

impl PartialEq for Ray {
    fn eq(&self, ray: &Ray) -> bool {
        self.dir_actor.get_position() == ray.get_position()
            && self.get_direction() == ray.get_direction()
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.get_position(), self.get_direction())
    }
}
