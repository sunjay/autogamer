use pyo3::prelude::*;
use pyo3::PyTraverseError;
use pyo3::gc::{PyGCProtocol, PyVisit};

use crate::*;

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

    //TODO: Determine if we need to drop the Py<...> fields to avoid leaking memory/reference cycles
    fn __clear__(&mut self) {}
}

#[pymethods]
impl Screen {
    #[new]
    pub fn new(game: Py<Game>) -> Self {
        Self {game}
    }

    pub fn update(&mut self, _events: &EventStream) {}

    pub fn draw(&mut self, _renderer: &mut Renderer) {}
}
