use pyo3::prelude::*;
use pyo3::PyTraverseError;
use pyo3::gc::{PyGCProtocol, PyVisit};

use crate::*;

#[pyclass(gc)]
#[derive(Debug)]
pub struct Game {
    window_width: u32,
    window_height: u32,
    current_screen: Option<Py<Screen>>
}

#[pyproto]
impl PyGCProtocol for Game {
    fn __traverse__(&self, visit: PyVisit) -> Result<(), PyTraverseError> {
        let Self {
            window_width: _,
            window_height: _,
            current_screen,
        } = self;

        if let Some(current_screen) = current_screen {
            visit.call(current_screen)?;
        }

        Ok(())
    }

    fn __clear__(&mut self) {
        let Self {
            window_width: _,
            window_height: _,
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
        window_width = 800,
        window_height = 600,
    )]
    pub fn new(
        window_width: u32,
        window_height: u32,
    ) -> Self {
        Self {
            window_width,
            window_height,
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
        todo!()
        // loop {
        //     current_level.dispatcher.run();
        //     current_level.viewport.update();
        //     current_level.map.draw();
        //     current_level.hud.draw();
        // }
    }
}
