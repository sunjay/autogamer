use std::sync::Arc;

use autogamer as ag;
use pyo3::prelude::*;
use parking_lot::Mutex;

#[pyclass(unsendable)]
#[derive(Debug)]
pub struct Renderer {
    renderer: ag::Renderer,
}

impl Renderer {
    pub fn new(
        canvas: ag::WindowCanvas,
        image_cache: Arc<Mutex<ag::ImageCache>>,
    ) -> Self {
        Self {
            renderer: ag::Renderer::new(
                canvas,
                image_cache,
            ),
        }
    }

    pub fn inner(&self) -> &ag::Renderer {
        &self.renderer
    }

    pub fn inner_mut(&mut self) -> &mut ag::Renderer {
        &mut self.renderer
    }
}

#[pymethods]
impl Renderer {
    //TODO: Public Python API for renderer
}
