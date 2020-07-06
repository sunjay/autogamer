use autogamer as ag;

use pyo3::prelude::*;
use pyo3::exceptions::ValueError;

use crate::*;
use crate::ui::*;

#[pyclass(subclass, extends=Screen)]
#[derive(Debug)]
pub struct Level {
    level: ag::Level,
    #[pyo3(get)]
    physics: Py<Physics>,
}

#[pymethods]
impl Level {
    #[new]
    pub fn new(game: Py<Game>) -> PyResult<(Self, Screen)> {
        let gil = GILGuard::acquire();
        let py = gil.python();

        let level = ag::Level::new(game.borrow(py).inner());
        let base = Screen::new(game);

        let level = Self {
            level,
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
    pub fn load(&mut self, map: &TileMap) -> PyResult<()> {
        self.level.load(map.inner())
            .map_err(|err| ValueError::py_err(err.to_string()))
    }

    /// Sets the dimensions of the viewport to the given values
    //TODO(PyO3/pyo3#1025): These should be keyword-only arguments with no defaults
    #[args("*", width=1, height=1)]
    pub fn set_viewport_dimensions(&mut self, width: u32, height: u32) {
        todo!()
    }
}
