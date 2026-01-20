use glam::Vec3;
use std::fmt;

/// Structure holding a ray's geometric data.
#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl PartialEq for Ray {
    fn eq(&self, ray: &Ray) -> bool {
        self.origin == ray.origin && self.direction == ray.direction
    }
}
