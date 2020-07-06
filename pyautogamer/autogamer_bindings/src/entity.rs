use pyo3::prelude::*;

/// Represents an entity and provides an interface for adding, removing, and
/// retrieving components from it
#[pyclass]
#[derive(Debug)]
pub struct Entity {
}

#[pymethods]
impl Entity {
    pub fn add(&mut self, component: PyObject) {
        todo!()
    }
}
