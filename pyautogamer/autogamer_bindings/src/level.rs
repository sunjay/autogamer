use pyo3::prelude::*;

use crate::*;
use crate::ui::*;

#[pyclass(subclass, extends=Screen)]
#[derive(Debug)]
pub struct Level {
    #[pyo3(get)]
    physics: Py<Physics>,
}

#[pymethods]
impl Level {
    #[new]
    pub fn new(game: Py<Game>) -> PyResult<(Self, Screen)> {
        let base = Screen::new(game);

        let gil = GILGuard::acquire();
        let py = gil.python();

        let level = Self {
            physics: Py::new(py, Physics::new())?,
        };

        Ok((level, base))
    }

    /// Adds a new entity to this level
    ///
    /// The new entity is given the following components:
    /// * `Player` - indicates that the entity is one of the players of the game
    /// * `Position` - set to `level_start` if that has been defined or (0,0) otherwise
    pub fn add_player(&mut self) -> Entity {
        //TODO: Set position to level start if configured, and (0, 0) otherwise
        Entity {}
    }

    /// Loads a map into this level, automatically discovering entities and
    /// components based on the contents of the map.
    pub fn load(&mut self, map: &TileMap) {
        //TODO: Check if we have an entity with the Player component, and if so
        // add a Position component. Otherwise just store the position for later
        todo!()
    }

    /// Sets the dimensions of the viewport to the given values
    //TODO(PyO3/pyo3#1025): These should be keyword-only arguments with no defaults
    #[args("*", width=1, height=1)]
    pub fn set_viewport_dimensions(&mut self, width: u32, height: u32) {
        todo!()
    }
}
