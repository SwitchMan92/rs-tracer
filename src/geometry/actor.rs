use glam::Vec3;

#[derive(Clone, Debug)]
pub struct Actor {
    pub position: Vec3,
}

impl Actor {
    pub const fn new(position: Vec3) -> Self {
        Self { position }
    }
}
