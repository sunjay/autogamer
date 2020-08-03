use std::sync::Arc;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::PyNativeType;
use parking_lot::Mutex;

use crate::*;

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
    /// Adds the given component to the entity
    ///
    /// If the entity already had this component, the value will be replaced
    /// with the given value.
    pub fn add(&self, component: &PyAny) -> PyResult<()> {
        let mut level = self.level.lock();
        let world = level.world_mut();

        write_component(world, self.entity, component)
    }

    /// Removes a component from this entity given its component class and
    /// returns its previous value if any
    pub fn remove(&self, component_class: &PyAny) -> PyResult<Option<PyObject>> {
        let class = PyComponentClass::from_py(component_class)?;

        let mut level = self.level.lock();
        let world = level.world_mut();

        class.remove(world, self.entity, component_class.py())
    }

    /// Returns a *copy* of a component for this entity given its component
    /// class or returns `None` if this entity does not have that component
    ///
    /// Modifying the copy will NOT modify the component stored with this
    /// entity. Use the `add` method on this entity to update the component
    /// value.
    ///
    /// Alternatively, if you use `join`, the components yielded by that can
    /// modify the component associated with an entity directly. Using join is
    /// often the preferred way of accessing the components associated with an
    /// entity. This method is usually only used for components that are
    /// optionally present on an entity.
    pub fn get(&self, component_class: &PyAny) -> PyResult<Option<PyObject>> {
        let class = PyComponentClass::from_py(component_class)?;
        class.read_copy(&self.level, self.entity, component_class.py())
    }
}
