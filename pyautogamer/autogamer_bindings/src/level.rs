use std::sync::Arc;

use autogamer as ag;
use pyo3::prelude::*;
use pyo3::PyTraverseError;
use pyo3::types::PyTuple;
use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::exceptions::PyValueError;
use parking_lot::Mutex;

use crate::*;
use crate::ui::*;

#[pyclass(subclass, gc, extends=Screen)]
#[derive(Debug)]
pub struct Level {
    level: Arc<Mutex<ag::Level>>,
    game: Py<Game>,
    #[pyo3(get)]
    physics: Py<PhysicsEngine>,
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

    //TODO: Determine if we need to drop the Py<...> fields to avoid leaking memory/reference cycles
    fn __clear__(&mut self) {}
}

#[pymethods]
impl Level {
    #[new]
    pub fn new(py: Python, game: Py<Game>) -> PyResult<(Self, Screen)> {
        let level = ag::Level::new(game.borrow(py).inner());
        let level = Arc::new(Mutex::new(level));

        let physics = Py::new(py, PhysicsEngine::new())?;

        let base = Screen::new(game.clone());

        let level = Self {level, game, physics};
        Ok((level, base))
    }

    #[args(components="*")]
    fn join(&self, components: &PyTuple) -> PyResult<Join> {
        let components = components.into_iter()
            .map(PyComponentClass::from_py)
            .collect::<Result<Vec<_>, _>>()?;
        let level = self.level.clone();
        Ok(Join::new(level, components))
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
    pub fn load(&mut self, py: Python, map: &TileMap) -> PyResult<()> {
        let mut game = self.game.borrow_mut(py);
        let mut image_cache = game.inner_mut().image_cache_mut();

        self.level.lock().load(
            map.base_dir(),
            map.inner(),
            &mut image_cache,
        ).map_err(|err| PyValueError::new_err(err.to_string()))
    }

    pub fn load_sprites(&mut self, py: Python, sheet: &CharacterSpritesheet) -> PyResult<CharacterSprites> {
        let mut game = self.game.borrow_mut(py);
        let mut image_cache = game.inner_mut().image_cache_mut();

        sheet.inner().load(&mut image_cache).map(Into::into)
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }

    /// Sets the dimensions of the viewport to the given values
    #[args("*", width, height)]
    pub fn set_viewport_dimensions(&mut self, width: u32, height: u32) {
        self.level.lock().set_viewport_dimensions(ag::Size {width, height})
    }

    pub fn update(&mut self, py: Python, events: &EventStream) {
        let mut physics = self.physics.borrow_mut(py);
        let physics = physics.inner_mut();

        let mut level = self.level.lock();
        level.update(events, physics)
    }

    pub fn draw(&mut self, renderer: &mut Renderer) -> PyResult<()> {
        let level = self.level.lock();
        level.draw(renderer.inner_mut())
            .map_err(|err| PyValueError::new_err(err.to_string()))
    }
}
