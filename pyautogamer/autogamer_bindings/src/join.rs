use std::sync::Arc;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::PyIterProtocol;
use pyo3::exceptions::ValueError;
use pyo3::types::PyTuple;
use specs::{WorldExt, BitSet, hibitset::{BitIter, BitSetLike}, world::Index};
use parking_lot::Mutex;

use crate::*;

/// Iterator over tuples of components, with each tuple tied to a single entity
///
/// Components that are added during iteration will not be yielded by the
/// iterator. If a component that would have been yielded is removed after this
/// object has been created, the iteration will raise an exception when that
/// removed component is reached. Entities that have been removed since this
/// object was created will be skipped as they come up during the iteration.
/// Thus, this will only ever yield entities that are still alive.
#[pyclass]
#[derive(Debug, Clone)]
pub struct Join {
    level: Arc<Mutex<ag::Level>>,
    /// The components that will be yielded in each tuple, in the order they
    /// will appear in the tuple
    components: Vec<PyComponentClass>,
    /// The mask of the entity IDs containing the components being joined over
    ids: BitIter<BitSet>,
}

impl Join {
    /// Joins over the given components and yields a tuple of `PyObject`s
    /// representing (component1, component2, ...). The yielded components are
    /// mutable and any changes will be reflected in the ECS. The `Entity`
    /// python class may also be used to make it possible to modify the entity
    /// directly or get optional components.
    pub fn new(
        level: Arc<Mutex<ag::Level>>,
        components: Vec<PyComponentClass>,
    ) -> Self {
        let ids = {
            use specs::Join;

            let level = level.lock();
            let world = level.world();
            let mut idset = BitSet::new();

            // Initialize the ID set with the currently alive entities mask
            let entities = world.entities();
            let (entities_set, _) = unsafe { entities.open() };
            idset.extend(entities_set);

            for class in &components {
                class.filter_bitset(world, &mut idset);
            }

            idset.iter()
        };

        Self {level, components, ids}
    }

    /// Creates a `specs::Entity` from an ID and only returns it if the entity
    /// is still alive
    fn get_entity_checked(&self, id: Index) -> Option<specs::Entity> {
        let level = self.level.lock();
        let world = level.world();
        let entity = world.entities().entity(id);

        if world.is_alive(entity) {
            Some(entity)
        } else {
            None
        }
    }
}

impl Iterator for Join {
    type Item = PyResult<Py<PyTuple>>;

    fn next(&mut self) -> Option<Self::Item> {
        let gil = GILGuard::acquire();
        let py = gil.python();

        // Search for the next alive entity
        let entity = loop {
            // Since `id` is yielded from `ids` (the mask), it is necessarily a
            // valid entity in the world (though it may not be alive)
            let id = self.ids.next()?;

            // Skip entities that are no longer alive
            if let Some(entity) = self.get_entity_checked(id) {
                break entity;
            }
        };

        let mut values = Vec::with_capacity(self.components.len());
        for class in &self.components {
            let value = match class.read(&self.level, entity, py) {
                Ok(Some(value)) => value,

                // Raise an exception if a component we were iterating over has
                // been removed.
                Ok(None) => return Some(Err(ValueError::py_err(format!("Component `{}` was removed from an entity during iteration over a join operation", class.name())))),

                Err(err) => return Some(Err(err)),
            };

            values.push(value);
        }

        Some(Ok(PyTuple::new(py, values).into_py(py)))
    }
}

#[pyproto]
impl PyIterProtocol for Join {
    fn __iter__(slf: PyRef<Self>) -> Py<Self> {
        slf.into()
    }

    fn __next__(mut slf: PyRefMut<Self>) -> PyResult<Option<PyObject>> {
        slf.next()
            .map(|opt| opt.map(|tuple| tuple.to_object(slf.py())))
            .transpose()
    }
}
