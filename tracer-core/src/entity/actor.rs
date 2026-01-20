use glam::Vec3;

use crate::entity::geometry::Geometry;

// #####################################

/// base 'class' inherited by any object allowing interaction with the current scene.
#[derive(Debug)]
pub struct Actor {
    pub position: Vec3,
}

pub trait ActorTrait {
    /// Get the actor's current position.
    fn get_position(&self) -> Vec3;
}

impl ActorTrait for Actor {
    fn get_position(&self) -> Vec3 {
        self.position
    }
}

impl Actor {
    pub const fn new(position: &Vec3) -> Self {
        Self {
            position: *position, // copying here as we want ownership of the Vec3 by the actor.
        }
    }
}

// #####################################
#[derive(Debug)]
pub struct DirectionalActor {
    actor: Actor,
    direction: Vec3,
}

pub trait DirectionalActorTrait: ActorTrait {
    /// Get the actor's current direction vector.
    fn get_direction(&self) -> Vec3;
}

impl ActorTrait for DirectionalActor {
    fn get_position(&self) -> Vec3 {
        self.actor.get_position()
    }
}

impl DirectionalActorTrait for DirectionalActor {
    fn get_direction(&self) -> Vec3 {
        self.direction
    }
}

impl DirectionalActor {
    pub const fn new(position: &Vec3, direction: &Vec3) -> Self {
        Self {
            actor: Actor::new(position),
            direction: *direction,
        }
    }
}
