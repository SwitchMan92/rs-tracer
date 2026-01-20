use glam::Vec3;

use crate::geometry::Geometry;

/// base 'class' inherited by any object allowing interaction with the current scene.
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
    pub const fn new(position: Vec3) -> Self {
        Self {
            position: position,
        }
    }
}

/// Public trait mixin used to allow operations on geomerty and position simultaneously.
pub trait ActorWithGeometry: Geometry + ActorTrait {}
