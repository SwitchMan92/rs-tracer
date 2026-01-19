use glam::Mat4;
use glam::Vec3;
use glam::Vec4Swizzles;

use crate::geometry::Geometry;

/// base 'class' inherited by any object allowing interaction with the current scene.
#[derive(Clone, Debug)]
pub struct Actor {
    translation: Mat4,
    rotation_angles: Vec3,
    rotation_matrices: [Mat4; 3],
    scale: Mat4,
}

pub trait ActorTrait {
    /// Get the actor's current position.
    fn get_location(&self) -> Vec3;
    fn get_rotation_angles(&self) -> Vec3;
    fn get_scaling_factors(&self) -> Vec3;

    fn set_location(&mut self, position: &Vec3);
    fn set_rotation(&mut self, euler_angles: &Vec3);
    fn set_scaling(&mut self, scale_factors: &Vec3);
}

impl Actor {
    pub fn new(location: &Vec3, euler_angles: &Vec3, scale_factors: &Vec3) -> Self {
        let mut result_obj = Self::default();
        result_obj.set_location(location);
        result_obj.set_rotation(euler_angles);
        result_obj.set_scaling(scale_factors);
        result_obj
    }

    fn calculate_rotation_matrices(&mut self) {
        let rx = &mut self.rotation_matrices[0];

        let rx_col1 = rx.col_mut(0);
        rx_col1.y = f32::cos(self.rotation_angles.x);
        rx_col1.z = -f32::sin(self.rotation_angles.x);

        let rx_col2 = rx.col_mut(1);
        rx_col2.y = f32::sin(self.rotation_angles.x);
        rx_col2.z = f32::cos(self.rotation_angles.x);

        let ry = &mut self.rotation_matrices[1];

        let ry_col1 = ry.col_mut(0);
        ry_col1.y = f32::cos(self.rotation_angles.y);
        ry_col1.z = -f32::sin(self.rotation_angles.y);

        let ry_col3 = ry.col_mut(2);
        ry_col3.y = f32::sin(self.rotation_angles.y);
        ry_col3.z = f32::cos(self.rotation_angles.y);

        let rz = &mut self.rotation_matrices[2];

        let rz_col1 = rz.col_mut(0);
        rz_col1.x = f32::cos(self.rotation_angles.z);
        rz_col1.y = f32::sin(self.rotation_angles.z);

        let rz_col2 = rz.col_mut(1);
        rz_col2.x = -f32::sin(self.rotation_angles.z);
        rz_col2.y = f32::cos(self.rotation_angles.z);
    }
}

impl Default for Actor {
    fn default() -> Self {
        Self {
            translation: Mat4::default(),
            rotation_angles: Vec3::default(),
            rotation_matrices: [Mat4::default(); 3],
            scale: Mat4::default(),
        }
    }
}

impl ActorTrait for Actor {
    fn get_location(&self) -> Vec3 {
        self.translation.col(3).xyz()
    }

    fn get_rotation_angles(&self) -> Vec3 {
        self.rotation_angles
    }

    fn get_scaling_factors(&self) -> Vec3 {
        Vec3::new(
            self.scale.col(0).x,
            self.scale.col(1).y,
            self.scale.col(2).z,
        )
    }

    fn set_location(&mut self, position: &Vec3) {
        let tr_col = self.translation.col_mut(3);
        tr_col.x = position.x;
        tr_col.y = position.y;
        tr_col.z = position.z;
    }

    fn set_rotation(&mut self, euler_angles: &Vec3) {
        self.rotation_angles = *euler_angles;
        self.calculate_rotation_matrices();
    }

    fn set_scaling(&mut self, scale_factors: &Vec3) {
        self.scale.col(0).x = scale_factors.x;
        self.scale.col(1).y = scale_factors.y;
        self.scale.col(2).z = scale_factors.z;
    }
}

/// Public trait mixin used to allow operations on geomerty and position simultaneously.
pub trait ActorWithGeometry: Geometry + ActorTrait {}

#[cfg(test)]
pub mod tests {
    use glam::Vec3;

    use crate::geometry::actor::{Actor, ActorTrait};

    #[test]
    fn test_success_translate() {
        let final_position = Vec3::new(1., 1., 1.);
        let mut actor = Actor::default();
        actor.set_location(&final_position);

        let location = actor.get_location();
        assert_eq!(location.x, final_position.x);
        assert_eq!(location.y, final_position.y);
        assert_eq!(location.z, final_position.z);
    }

    fn test_success_rotate() {
        let final_position = Vec3::new(1., 1., 1.);
        let mut actor = Actor::default();
        actor.set_location(&final_position);

        let location = actor.get_location();
        assert_eq!(location.x, final_position.x);
        assert_eq!(location.y, final_position.y);
        assert_eq!(location.z, final_position.z);
    }
}
