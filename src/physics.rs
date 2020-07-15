use crate::Vec2;

#[derive(Debug)]
pub struct PhysicsEngine {
    gravity: Vec2,
}

impl PhysicsEngine {
    pub fn new() -> Self {
        Self {
            gravity: Vec2::new(0.0, 0.0),
        }
    }

    pub fn gravity(&self) -> Vec2 {
        self.gravity
    }

    pub fn set_gravity(&mut self, gravity: Vec2) {
        self.gravity = gravity;
    }
}
