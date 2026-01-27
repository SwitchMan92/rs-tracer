use glam::Vec3AA;

// #####################################

/// base 'class' inherited by any object allowing interaction with the current scene.
#[derive(Debug)]
pub struct Actor {
    pub position: Vec3AA,
}

pub trait ActorTrait {
    /// Get the actor's current position.
    fn get_position(&self) -> Vec3AA;
}

impl ActorTrait for Actor {
    fn get_position(&self) -> Vec3AA {
        self.position
    }
}

impl Actor {
    pub const fn new(position: &Vec3AA) -> Self {
        Self {
            position: *position, // copying here as we want ownership of the Vec3AA by the actor.
        }
    }
}

// #####################################
#[derive(Debug)]
pub struct DirectionalActor {
    actor: Actor,
    direction: Vec3AA,
}

pub trait DirectionalActorTrait: ActorTrait {
    /// Get the actor's current direction vector.
    fn get_direction(&self) -> Vec3AA;
}

impl ActorTrait for DirectionalActor {
    fn get_position(&self) -> Vec3AA {
        self.actor.get_position()
    }
}

impl DirectionalActorTrait for DirectionalActor {
    #[inline]
    fn get_direction(&self) -> Vec3AA {
        self.direction
    }
}

impl DirectionalActor {
    pub const fn new(position: &Vec3AA, direction: &Vec3AA) -> Self {
        Self {
            actor: Actor::new(position),
            direction: *direction,
        }
    }
}
