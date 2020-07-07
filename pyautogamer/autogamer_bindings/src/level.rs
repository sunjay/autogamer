use std::sync::Arc;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::PyTraverseError;
use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::exceptions::ValueError;
use parking_lot::Mutex;

use crate::*;
use crate::ui::*;

#[pyclass(subclass, gc, extends=Screen)]
#[derive(Debug)]
pub struct Level {
    level: Arc<Mutex<ag::Level>>,
    game: Py<Game>,
    #[pyo3(get)]
    physics: Py<Physics>,
}

#[pyproto]
impl PyGCProtocol for Level {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        let Self {
            level: _,
            game,
            physics,
        } = self;

        visit.call(game)?;
        visit.call(physics)?;

        Ok(())
    }

    fn __clear__(&mut self) {
        let Self {
            level: _,
            game,
            physics,
        } = self;

        // Release reference, this decrements the ref counter
        let gil = GILGuard::acquire();
        let py = gil.python();

        py.release(&*game);
        py.release(&*physics);
    }
}

#[pymethods]
impl Level {
    #[new]
    pub fn new(game: Py<Game>) -> PyResult<(Self, Screen)> {
        let gil = GILGuard::acquire();
        let py = gil.python();

        let level = ag::Level::new(game.borrow(py).inner());
        let level = Arc::new(Mutex::new(level));

        let physics = Py::new(py, Physics::new())?;

        let base = Screen::new(game.clone());

        let level = Self {level, game, physics};
        Ok((level, base))
    }

    /// Adds a new entity to this level
    ///
    /// The new entity is given the following components:
    /// * `Player` - indicates that the entity is one of the players of the game
    /// * `Position` - set to `level_start` if that has been defined or (0,0) otherwise
    pub fn add_player(&mut self) -> Entity {
        let entity = self.level.lock().add_player();
        Entity::new(self.level.clone(), entity)
    }

    /// Loads a map into this level, automatically discovering entities and
    /// components based on the contents of the map.
    pub fn load(&mut self, map: &TileMap) -> PyResult<()> {
        let gil = GILGuard::acquire();
        let py = gil.python();

        let mut game = self.game.borrow_mut(py);

        self.level.lock().load(
            map.base_dir(),
            map.inner(),
            game.inner_mut().renderer_mut(),
        ).map_err(|err| ValueError::py_err(err.to_string()))
    }

    /// Sets the dimensions of the viewport to the given values
    //TODO(PyO3/pyo3#1025): These should be keyword-only arguments with no defaults
    #[args("*", width=1, height=1)]
    pub fn set_viewport_dimensions(&mut self, width: u32, height: u32) {
        self.level.lock().set_viewport_dimensions(ag::Size {width, height})
    }
}
