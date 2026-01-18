use glam::Vec3;

use crate::geometry::Geometry;

#[derive(Clone, Debug)]
pub struct Actor {
    pub position: Vec3,
}

pub trait ActorTrait {
    fn get_position(&self) -> Vec3;
}

impl ActorTrait for Actor {
    fn get_position(&self) -> Vec3 {
        self.position
    }
}

impl Actor {
    pub const fn new(position: Vec3) -> Self {
        Self { position }
    }
}

pub trait ActorWithGeometry: Geometry + ActorTrait {}
