use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Physics {
}

#[pymethods]
impl Physics {
    #[new]
    pub fn new() -> Self {
        Self {}
    }

    pub fn set_gravity(&mut self, gravity: (f64, f64)) {
        let (x_gravity, y_gravity) = gravity;
        todo!()
    }
}
