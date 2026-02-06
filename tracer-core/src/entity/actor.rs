use glam::Vec3A;

// #####################################

/// base 'class' inherited by any object allowing interaction with the current scene.
#[derive(Debug)]
pub struct Actor {
    pub position: Vec3A,
}

pub trait ActorTrait {
    /// Get the actor's current position.
    fn get_position(&self) -> Vec3A;
}

impl ActorTrait for Actor {
    fn get_position(&self) -> Vec3A {
        self.position
    }
}

impl Actor {
    pub const fn new(position: &Vec3A) -> Self {
        Self {
            position: *position,
        }
    }
}

// #####################################
#[derive(Debug)]
pub struct DirectionalActor {
    actor: Actor,
    direction: Vec3A,
}

pub trait DirectionalActorTrait: ActorTrait {
    /// Get the actor's current direction vector.
    fn get_direction(&self) -> Vec3A;
}

impl ActorTrait for DirectionalActor {
    fn get_position(&self) -> Vec3A {
        self.actor.get_position()
    }
}

impl DirectionalActorTrait for DirectionalActor {
    #[inline]
    fn get_direction(&self) -> Vec3A {
        self.direction
    }
}

impl DirectionalActor {
    pub const fn new(position: &Vec3A, direction: &Vec3A) -> Self {
        Self {
            actor: Actor::new(position),
            direction: *direction,
        }
    }
}
