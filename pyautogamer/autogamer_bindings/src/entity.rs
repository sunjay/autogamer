use std::sync::Arc;

use autogamer as ag;
use pyo3::prelude::*;
use parking_lot::Mutex;

use crate::write_component;

/// Represents an entity and provides an interface for adding, removing, and
/// retrieving components from it
#[pyclass]
#[derive(Debug)]
pub struct Entity {
    level: Arc<Mutex<ag::Level>>,
    entity: specs::Entity,
}

impl Entity {
    pub fn new(level: Arc<Mutex<ag::Level>>, entity: specs::Entity) -> Self {
        Self {level, entity}
    }
}

#[pymethods]
impl Entity {
    pub fn add(&mut self, component: &PyAny) -> PyResult<()> {
        let mut level = self.level.lock();
        let world = level.world_mut();

        write_component(world, self.entity, component)
    }
}
