use autogamer as ag;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct PhysicsEngine {
    physics: ag::PhysicsEngine,
}

impl PhysicsEngine {
    pub fn inner(&self) -> &ag::PhysicsEngine {
        &self.physics
    }

    pub fn inner_mut(&mut self) -> &mut ag::PhysicsEngine {
        &mut self.physics
    }
}

#[pymethods]
impl PhysicsEngine {
    #[new]
    pub fn new() -> Self {
        Self {
            physics: ag::PhysicsEngine::new(),
        }
    }

    pub fn set_gravity(&mut self, gravity: (f64, f64)) {
        let (x_gravity, y_gravity) = gravity;
        self.physics.set_gravity(ag::Vec2::new(x_gravity, y_gravity))
    }
}
