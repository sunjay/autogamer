use autogamer as ag;
use pyo3::prelude::*;
use pyo3::exceptions::ValueError;

#[pymodule]
/// Bindings to the autogamer native module
pub fn autogamer_bindings(_py: Python, pymod: &PyModule) -> PyResult<()> {
    pymod.add_class::<Game>()?;
    pymod.add_class::<Level>()?;
    pymod.add_class::<Entity>()?;
    pymod.add_class::<TileMap>()?;
    Ok(())
}

#[pyclass]
#[derive(Debug)]
pub struct Game {
}

#[pymethods]
impl Game {
    #[new]
    fn new() -> Self {
        Self {}
    }

    /// Adds a new entity to the default level of the game
    ///
    /// See the `add_player` method on `Level` for more details.
    fn add_player(&mut self) -> Entity {
        todo!()
    }

    /// Loads a map into the default level of the game, automatically
    /// discovering entities and components based on the contents of the map.
    ///
    /// See the `add_player` method on `Level` for more details.
    fn load(&mut self, map: &TileMap) {
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
