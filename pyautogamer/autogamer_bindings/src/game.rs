use autogamer as ag;
use pyo3::prelude::*;
use pyo3::PyTraverseError;
use pyo3::gc::{PyGCProtocol, PyVisit};
use pyo3::exceptions::ValueError;

use crate::*;

#[pyclass(gc, unsendable)]
#[derive(Debug)]
pub struct Game {
    game: ag::Game,
    current_screen: Option<Py<Screen>>
}

impl Game {
    pub fn inner(&self) -> &ag::Game {
        &self.game
    }

    pub fn inner_mut(&mut self) -> &mut ag::Game {
        &mut self.game
    }
}

#[pyproto]
impl PyGCProtocol for Game {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        let Self {
            game: _,
            current_screen,
        } = self;

        if let Some(current_screen) = current_screen {
            visit.call(current_screen)?;
        }

        Ok(())
    }

    fn __clear__(&mut self) {
        let Self {
            game: _,
            current_screen,
        } = self;

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
    #[args(
        "*",
        title = "\"autogamer\".to_string()",
        window_width = 800,
        window_height = 600,
    )]
    pub fn new(
        title: String,
        window_width: u32,
        window_height: u32,
    ) -> Self {
        Self {
            game: ag::Game::new(title, ag::Size {
                width: window_width,
                height: window_height,
            }),
            current_screen: None,
        }
    }

    /// Sets the current screen of the game to the given screen
    pub fn set_screen(&mut self, screen: Py<Screen>) {
        self.current_screen = Some(screen);
    }

    /// Runs the game main loop until either the window is closed or the game
    /// loop is ended by the game itself
    pub fn run(&mut self) -> PyResult<()> {
        let window = self.game.create_window()
            .map_err(|err| ValueError::py_err(err.to_string()))?;

        // Create the texture creator that will load images
        let texture_creator = window.texture_creator();
        self.game.renderer_mut()
            .image_cache_mut()
            .set_texture_creator(texture_creator);

        let current_screen = match self.current_screen.take() {
            Some(screen) => screen,
            // No screen configured, quit immediately
            None => return Ok(()),
        };

        //loop {
        //    current_level.dispatcher.run();
        //    current_level.viewport.update();
        //    current_level.map.draw();
        //    current_level.hud.draw();
        //    //TODO: manage timing
        //}

        Ok(())
    }
}
