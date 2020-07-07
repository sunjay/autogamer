use autogamer as ag;
use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Physics {
    physics: ag::Physics,
}

#[pymethods]
impl Physics {
    #[new]
    pub fn new() -> Self {
        Self {
            physics: ag::Physics::new(),
        }
    }

    pub fn set_gravity(&mut self, gravity: (f64, f64)) {
        let (x_gravity, y_gravity) = gravity;
        self.physics.set_gravity(ag::Vec2::new(x_gravity, y_gravity))
    }
}
