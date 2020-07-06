use autogamer as ag;
use pyo3::prelude::*;
use pyo3::PyTraverseError;
use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::exceptions::ValueError;

#[pymodule]
/// Bindings to the autogamer native module
pub fn autogamer_bindings(py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_wrapped(pyo3::wrap_pymodule!(ui))?;
    py.run("\
import sys
sys.modules['autogamer_bindings.ui'] = ui
    ", None, Some(pymod.dict()))?;

    pymod.add_class::<Game>()?;
    pymod.add_class::<Level>()?;
    pymod.add_class::<Entity>()?;
    pymod.add_class::<TileMap>()?;

    Ok(())
}

#[pymodule]
/// Bindings to the autogamer native UI module
pub fn ui(_py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<Screen>()?;
    Ok(())
}

#[pyclass(gc)]
#[derive(Debug)]
pub struct Game {
    current_screen: Option<Py<Screen>>
}

#[pyproto]
impl PyGCProtocol for Game {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        let Self {current_screen} = self;
        visit.call(current_screen)?;
        Ok(())
    }

    fn __clear__(&mut self) {
        let Self {current_screen} = self;
        // Release reference, this decrements the ref counter
        let gil = GILGuard::acquire();
        let py = gil.python();
        if let Some(current_screen) = current_screen.take() {
            py.release(&current_screen);
        }
    }
}

#[pymethods]
impl Game {
    #[new]
    pub fn new() -> Self {
        Self {
            current_screen: None,
        }
    }

    /// Sets the current screen of the game to the given screen
    pub fn set_screen(&mut self, screen: Py<Screen>) {
        self.current_screen = Some(screen);
    }

    /// Runs the game main loop until either the window is closed or the game
    /// loop is ended by the game itself
    pub fn run(&mut self) {
        let current_screen = match self.current_screen.take() {
            Some(screen) => screen,
            // No screen configured, quit immediately
            None => return,
        };
        // loop {
        //     current_level.dispatcher.run();
        //     current_level.viewport.update();
        //     current_level.map.draw();
        //     current_level.hud.draw();
        // }
    }
}

#[pyclass(subclass, gc)]
#[derive(Debug)]
pub struct Screen {
    #[pyo3(get)]
    game: Py<Game>,
}

#[pyproto]
impl PyGCProtocol for Screen {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        let Self {game} = self;
        visit.call(game)?;
        Ok(())
    }

    fn __clear__(&mut self) {
        let Self {game} = self;
        // Release reference, this decrements the ref counter
        let gil = GILGuard::acquire();
        let py = gil.python();
        py.release(&*game);
    }
}

#[pymethods]
impl Screen {
    #[new]
    pub fn new(game: Py<Game>) -> Self {
        Self {game}
    }

    pub fn update(&mut self, events: i32) {
        //TODO: Figure out type for `events`
    }

    pub fn draw(&mut self, renderer: i32) {
        //TODO: Figure out type for `renderer`
    }
}

#[pyclass]
#[derive(Debug)]
pub struct Level {
}

#[pymethods]
impl Level {
    #[new]
    fn new() -> Self {
        Self {}
    }

    /// Adds a new entity to this level
    ///
    /// The new entity is given the following components:
    /// * `Player` - indicates that the entity is one of the players of the game
    /// * `Position` - set to `level_start` if that has been defined or (0,0) otherwise
    fn add_player(&mut self) -> Entity {
        //TODO: Set position to level start if configured, and (0, 0) otherwise
        todo!()
    }

    /// Loads a map into this level, automatically discovering entities and
    /// components based on the contents of the map.
    fn load(&mut self, map: &TileMap) {
        //TODO: Check if we have an entity with the Player component, and if so
        // add a Position component. Otherwise just store the position for later
    }

    fn fullscreen(&self) {
    }

    fn run(&mut self) {
        // loop {
        //     current_level.dispatcher.run();
        //     current_level.viewport.update();
        //     current_level.map.draw();
        //     current_level.hud.draw();
        // }
    }
}

/// Represents an entity and provides an interface for adding, removing, and
/// retrieving components from it
#[pyclass]
#[derive(Debug)]
pub struct Entity {
}

/// Represents the raw data in a Tiled TMX file.
///
/// This can be queried and modified as needed before being loaded into the
/// game. Any modifications after this has been added to the game will be
/// ignored.
#[pyclass]
#[derive(Debug)]
pub struct TileMap {
    map: ag::TileMap,
}

#[pymethods]
impl TileMap {
    #[new]
    fn new(path: &str) -> PyResult<Self> {
        let map = ag::TileMap::load(path)
            .map_err(|err| ValueError::py_err(err.to_string()))?;
        Ok(Self {map})
    }
}
